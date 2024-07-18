use thiserror::Error;
use diesel::result::Error as DieselError;
use bcrypt::BcryptError;
use rocket::http::Status;
use rocket::response::{Responder, Response, status};
use rocket::request::Request;
use rocket::serde::json::Json;
#[derive(Debug, Error)]
pub enum SignUpError {
    #[error("Database error")]
    DatabaseError(#[from] DieselError),
    #[error("Hashing error")]
    HashingError(#[from] BcryptError),
    #[error("Connection error")]
    ConnectionError(String),
    #[error("Username taken")]
    UsernameTakenError,
    #[error("Username must be at least 1 char long")]
    InvalidUsername,
    #[error("Password must be at least 1 char long")]
    InvalidPassword
}

impl<'r> Responder<'r, 'static> for SignUpError {
    fn respond_to(self, req: &'r Request<'_>) -> rocket::response::Result<'static> {
        let status = match self {
            SignUpError::DatabaseError(_) | SignUpError::ConnectionError(_) | SignUpError::HashingError(_) => {
                Status::InternalServerError
            },
            SignUpError::UsernameTakenError => Status::Conflict,
            SignUpError::InvalidPassword | SignUpError::InvalidUsername => Status::BadRequest
        };
        status::Custom(status, Json("message: ".to_string() + &*self.to_string())).respond_to(req)
    }
}