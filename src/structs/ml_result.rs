use rocket::{Request, response, Response};
use rocket::http::ContentType;
use rocket::response::Responder;
use rocket::serde::{Deserialize, Serialize};
use crate::structs::DataPrompt;

#[derive(Debug, Serialize, Deserialize)]
pub struct MlRequestResult {
    pub text: String,
    pub prompt: String,
    pub result: String
}
impl<'r> Responder<'r, 'static> for MlRequestResult {
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