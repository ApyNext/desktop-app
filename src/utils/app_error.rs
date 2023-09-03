use hyper::StatusCode;

pub enum AppError {
    //Request errors
    InternalServerError,
    Forbidden(String),
    UnknownError(StatusCode),
}
