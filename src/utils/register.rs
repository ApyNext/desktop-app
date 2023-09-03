//use dioxus::prelude::Scope;
use serde::{Deserialize, Serialize};

//use crate::API_URL;

//use super::{app_error::AppError, custom_request::custom_post_request};

/*pub async fn register_request(cx: Scope, body: String) -> Result<(), AppError> {
    custom_post_request(cx, format!("{}/register", API_URL), body).await
}*/

#[derive(Serialize, Deserialize)]
pub struct RegisterUser<'a> {
    pub username: &'a str,
    pub email: &'a str,
    pub password: &'a str,
    pub birthdate: i64,
    pub is_male: Option<bool>,
}
