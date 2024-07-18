use rocket::http::Status;
use rocket::response::{status, Responder};
use rocket::serde::json::Json;
use rocket::Request;
use thiserror::Error;
use jwt::error::Error as JwtError;
use diesel::result::Error as DieselError;
use hmac::digest::InvalidLength;

#[derive(Debug, Error)]
pub enum LoginError {
    #[error("database error: {0}")]
    DatabaseError(#[from] DieselError),
    #[error("user not found")]
    UserNotFound,
    #[error("wrong password")]
    WrongPassword,
    #[error("HMAC key error")]
    HmacKeyError,
    #[error("JWT signing error")]
    JwtSigningError(#[from] JwtError),
}
impl From<InvalidLength> for LoginError {
    fn from(_: InvalidLength) -> Self {
        LoginError::HmacKeyError
    }
}

impl<'r> Responder<'r, 'static> for LoginError {
    fn respond_to(self, req: &'r Request<'_>) -> rocket::response::Result<'static> {
        let status = match self {
            LoginError::DatabaseError(_) => Status::InternalServerError,
            LoginError::UserNotFound | LoginError::WrongPassword => Status::Unauthorized,
            LoginError::HmacKeyError | LoginError::JwtSigningError(_) => Status::InternalServerError,
        };
        status::Custom(status, Json(self.to_string())).respond_to(req)
    }
}
