use diesel::result::Error as DieselError;
use serde_json::Error as SerdeError;
use thiserror::Error;
use rocket::http::Status;
use rocket::response::{status, Responder};
use rocket::serde::json::Json;
use rocket::Request;

#[derive(Debug, Error)]
pub enum FetchHistoryError {
    #[error("database error: {0}")]
    DatabaseError(#[from] DieselError),
    #[error("serialization error: {0}")]
    SerializationError(#[from] SerdeError),
    #[error("history not found")]
    NotFound,
}
impl<'r> Responder<'r, 'static> for FetchHistoryError {
    fn respond_to(self, request: &'r Request<'_>) -> rocket::response::Result<'static> {
        let status = match self {
            FetchHistoryError::DatabaseError(_) | FetchHistoryError::SerializationError(_) => Status::InternalServerError,
            FetchHistoryError::NotFound => Status::NotFound
        };
        status::Custom(status, Json(self.to_string())).respond_to(request)
    }
}