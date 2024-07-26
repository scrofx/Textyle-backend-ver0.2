use std::env;
use std::fmt::Error;
use std::future::Future;
use bcrypt::{DEFAULT_COST, hash};
use chrono::Duration;
use diesel::{Connection, PgConnection};
use dotenvy::dotenv;
use rocket::*;
use rocket::form::Form;
use rocket::response::status;
use rocket_sync_db_pools::database;
use crate::jwt_verification::AuthenticatedUser;
use crate::token_response::TokenResponse;
use jwt::{SignWithKey, VerifyWithKey};
use sha2::{Sha256};
use hmac::{Hmac, Mac};
use pyo3::{PyResult, Python};
use pyo3::prelude::PyAnyMethods;
use pyo3::types::PyModule;
use rocket::http::{Method, Status, Header};
use rocket_sync_db_pools::Error::Custom;
use crate::error_types::{FetchHistoryError, LoginError, SignUpError};
use rocket::serde::json::Json;
use crate::structs::*;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Request, Response};
use serde_json::{to_string, Value};
use crate::structs::ml_result::MlRequestResult;

mod token_response;
mod jwt_verification;
mod error_types;
mod structs;
mod schema;



#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
extern crate core;


#[database("postgres_db")]
struct DbConn(diesel::PgConnection);


pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        if request.method() == Method::Options {
            response.set_status(Status::NoContent);
            response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, OPTIONS, DELETE"));
            response.set_header(Header::new("Access-Control-Allow-Headers", "Content-Type, Authorization"));
        }
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}
#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(CORS)
        .attach(DbConn::fairing())
        .mount("/", routes![feed, login, sign_up, test, get_history, options_history, clear_history])

}
#[get("/")]
fn test() -> String{
    return ("Hello there".parse().unwrap());
}
#[delete("/clear")]
async fn clear_history(user: AuthenticatedUser, db: DbConn) -> Result<Status, FetchHistoryError>{
    let res = db.run(move |conn| {
        user.clear_history(conn)
    }).await;
    match res {
        Ok(V) => {
            Ok(Status::Ok)
        }
        Err(e) => {
            Err(FetchHistoryError::DatabaseError(e))
        }
    }
}
#[post("/feed", data = "<data>")]
async fn feed(user: AuthenticatedUser, data: Form<DataPrompt>, db: DbConn) -> Result<MlRequestResult, status::Custom<String>>{
    let data = data.into_inner();
    //pretend to feed data to ml model
    let model_res = model_init(data.text, data.prompt);
    println!("{:?}", model_res);
    let output = MlRequestResult {
        text: "".parse().unwrap(),
        prompt: "".parse().unwrap(),
        result: "".parse().unwrap()
    };
    /*db.run(move |conn| {
        user.append_history(result, conn).expect("function not working")
    }).await;*/

    return Ok(output)

}
#[get("/history")]
async fn get_history(user: AuthenticatedUser, db: DbConn) -> Result<Value, FetchHistoryError>{

    let his = db.run(move |conn| {
        user.get_history(conn).ok()?
    }).await;
    match his {
        None => {
            Err(FetchHistoryError::NotFound)
        }
        Some(his) => {
            println!("{:?}", his);
            Ok(Value::Array(his))
        }
    }

}
#[post("/login", data = "<user_form>")]
async fn login(user_form: Form<LoginForm>, db: DbConn) -> Result<TokenResponse, LoginError> {

    let user_passed = user_form.into_inner();

    let user_found_res = db.run(move |conn| {
        User::find_by_username(conn, user_passed.username.as_ref())
    }).await;

    match user_found_res {
        Ok(V) => {
            match V {
                Some(T) => {
                    match T.verify_password(user_passed.password).await {
                        true => {
                            let key: Hmac<Sha256> = Hmac::new_from_slice("secret_key".as_bytes())?;
                            let token_str = Claims{
                                sub: T.username,
                                exp: (chrono::Utc::now() + Duration::minutes(20)).timestamp() as usize,
                            }.sign_with_key(&key)?;
                            return Ok(TokenResponse {
                                token: token_str,
                            })

                        }
                        false => {
                            Err(LoginError::WrongPassword)
                        }

                    }
                }
                None => {
                    Err(LoginError::UserNotFound)
                }
            }
        }
        Err(e) => {
            Err(LoginError::DatabaseError(e))
        }
    }


}

#[post("/sign_up", data = "<user_form>")]
async fn sign_up(user_form: Form<LoginForm>, db: DbConn)-> Result<status::Created<Json<User>>, SignUpError> {
    let user_passed = user_form.into_inner();
    if user_passed.username.len() == 0 {
        return Err(SignUpError::InvalidPassword)
    } else if user_passed.password.len() == 0 {
        return Err(SignUpError::InvalidPassword)
    }
    let hashed_password = hash(user_passed.password, DEFAULT_COST).map_err(SignUpError::HashingError)?;

    let mut new_user = User {
        username: user_passed.username,
        password_hashed: hashed_password
    };

    new_user = db.run(move |conn| {
        new_user.create_new(conn)
    }).await?;


    Ok(status::Created::new("/").body(Json(new_user)))
}

fn model_init(text: String, prompt: String) -> PyResult<String> {
    //NOT WORKING FUNC, working one is stored at actual server
    Python::with_gil(|py| {
        let model = PyModule::import_bound(py, "model").expect("");
        let model_class = model.getattr("Model")?;
        let model_inst = model_class.call1((text, prompt))?;

        model_inst.call_method0("init_ids")?;

        let res: String = model_inst.call_method0("generate_output")?.extract()?;
        println!("{:?}", res);
        Ok(res)
    })
    
}
