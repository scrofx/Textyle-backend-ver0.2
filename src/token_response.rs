use rocket::{Request, response, Response};
use rocket::http::ContentType;
use rocket::response::Responder;
use rocket::serde::Serialize;
use crate::structs::DataPrompt;

#[derive(Serialize)]
pub struct TokenResponse {
    pub token: String,
}
impl<'r> Responder<'r, 'static> for TokenResponse {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        let json = serde_json::to_string(&self).map_err(|e| {
            rocket::http::Status::InternalServerError
        })?;

        Response::build()
            .header(ContentType::JSON)
            .sized_body(json.len(), std::io::Cursor::new(json))
            .ok()
    }
}