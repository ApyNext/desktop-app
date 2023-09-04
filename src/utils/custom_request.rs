use hyper::{http::HeaderValue, HeaderMap, StatusCode};
use reqwest::Client;
use serde::Serialize;

use super::app_error::AppError;

pub async fn custom_post_request(
    client: &Client,
    url: String,
    body: &impl Serialize,
    headers: HeaderMap<HeaderValue>,
) -> Result<(), AppError> {
    log::info!("Sending...");
    let res = client
        .post(url)
        .headers(headers)
        .json(body)
        .send()
        .await
        .unwrap();
    log::info!("Sent !");

    match res.status() {
        StatusCode::OK => Ok(()),
        StatusCode::FORBIDDEN => {
            let body = match res.text().await {
                Ok(body) => body,
                Err(e) => return Err(AppError::Forbidden(e.to_string())),
            };
            Err(AppError::Forbidden(body))
        }
        StatusCode::INTERNAL_SERVER_ERROR => Err(AppError::InternalServerError),
        code => Err(AppError::UnknownError(code)),
    }
}
