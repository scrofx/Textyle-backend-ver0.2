use diesel::prelude::*;
#[derive(FromForm, Debug)]
pub struct LoginForm {
    pub username: String,
    pub password: String,
}