use core::fmt;

use rocket::{http::Status, serde::json::Json, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct ErrorModel {
    message: String,
    error: Option<String>,
}

#[derive(Responder)]
pub struct Error {
    inner: (Status, Json<ErrorModel>),
}

impl Error {
    pub fn new<S: Into<String>>(
        status: Status,
        message: impl Into<String>,
        error: Option<S>,
    ) -> Self {
        let err = ErrorModel {
            message: message.into(),
            error: error.map(|e| e.into()),
        };
        let inner = (status, Json(err));
        Self { inner }
    }

    pub fn message(status: Status, message: impl Into<String>) -> Self {
        Self::new::<&'static str>(status, message, None)
    }
}

impl<D> From<D> for Error
where
    D: fmt::Display,
{
    fn from(value: D) -> Self {
        Self::new(
            Status::InternalServerError,
            "Unexpected Error",
            Some(value.to_string()),
        )
    }
}