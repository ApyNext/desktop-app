use hyper::StatusCode;

#[derive(Debug)]
pub enum AppError {
    //Request errors
    InternalServerError,
    Forbidden(String),
    UnknownError(StatusCode),
}
