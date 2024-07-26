use diesel::dsl::Values;
use diesel::prelude::*;
use diesel::result::Error;
use diesel::sql_query;
use diesel::sql_types::Jsonb;
use hmac::{Hmac, Mac};
use jwt::VerifyWithKey;
use rocket::http::Status;
use rocket::Request;
use rocket::request::{FromRequest, Outcome};
use serde_json::{json, Value};
use sha2::Sha256;
use crate::DbConn;
use crate::error_types::FetchHistoryError;
use crate::schema::user_histories::dsl::user_histories;
use crate::schema::user_histories::{requests, username};
use crate::structs::{Claims, MlRequestResult, User, UserHistory};


#[derive(Debug)]
pub struct AuthenticatedUser(User);

impl AuthenticatedUser {
    pub fn append_history(mut self, data: MlRequestResult, conn: &mut PgConnection) -> Result<Option<Self>, diesel::result::Error>{

        let data_json: Value = serde_json::to_value(&data).expect("Serge json not OK");



        println!("{:?}", data_json);

        let query = diesel::sql_query(
            "UPDATE user_histories \
            SET requests = array_append(requests, $1::jsonb) \
         WHERE username = $2"
        );

        let rows_aff = query
            .bind::<Jsonb, _>(data_json)
            .bind::<diesel::sql_types::Text, _>(&self.0.username)
            .execute(conn)?;


        return Ok(Some(self))

    }
    pub fn get_history(self, conn: &mut PgConnection) -> Result<Option<Vec<Value>>, FetchHistoryError>{
        let query = format!(
            "SELECT username, requests FROM user_histories WHERE username = '{}'",
            &self.0.username
        );
        let history = sql_query(query).load::<UserHistory>(conn)?;

        match history.len() {
            0 => {
                Ok(Some(vec![json!({})]))
            }
            _ => {
                Ok(Some(history[0].requests.clone()))
            }
        }



    }
    pub fn clear_history(self, conn: &mut PgConnection) -> Result<bool, diesel::result::Error> {
        diesel::update(user_histories.filter(username.eq(&self.0.username))).set(requests.eq::<Vec<Value>>(vec![])).execute(conn)?;

        return Ok(true)

    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let db = request.guard::<DbConn>().await;
        if let Outcome::Success(db) = db {
            let token = request.headers().get_one("Authorization")
                .map(|s| s.replace("Bearer ", ""));
            if let Some(token) = token {
                let key: Hmac<Sha256> = Hmac::new_from_slice(Secret_key.as_bytes()).expect("Invalid key length");
                let claims: Claims = token.verify_with_key(&key).expect("Verify with key was broked");
                if claims.exp > chrono::Utc::now().timestamp() as usize {

                   let user = db.run(move |conn| {
                       User::find_by_username(conn, claims.sub.as_str())
                   }).await;
                    return match user {
                        Ok(user) => {
                            match user {
                                None => {
                                    Outcome::Error((Status::Unauthorized, ()))
                                }
                                Some(V) => {
                                    Outcome::Success(AuthenticatedUser(V))
                                }
                            }
                        }
                        Err(E) => { Outcome::Error((Status::Unauthorized, ())) }
                    }
                }

            }
        }
        return Outcome::Error((Status::Unauthorized, ()))
    }
}
