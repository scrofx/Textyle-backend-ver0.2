use rocket::response::{self, Responder, Response};
use rocket::http::ContentType;
use rocket::Request;
use serde::{Deserialize, Serialize};
use serde_json::Error;
#[derive(FromForm, Debug, Deserialize, Serialize)]
pub struct DataPrompt {
    pub text: String,
    pub prompt: String
}
impl DataPrompt {
    pub fn from_json(raw_json: &str) -> Result<Self, Error>{
        serde_json::from_str(raw_json)
    }
    pub fn new() -> Self {
        DataPrompt{
            text: String::new(),
            prompt: String::new()
        }
    }
}
impl<'r> Responder<'r, 'static> for DataPrompt {
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