use axum::{
    http::StatusCode,
    response::{IntoResponse, IntoResponseParts, Response},
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum HandlerError {
    #[error("internal server error: {0} ")]
    InternalServerError(anyhow::Error),
}

impl HandlerError {
    pub fn status(&self) -> StatusCode {
        match self {
            HandlerError::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub fn error(&self) -> &anyhow::Error {
        match self {
            HandlerError::InternalServerError(e) => e,
        }
    }
}

impl IntoResponse for HandlerError {
    fn into_response(self) -> Response {
        let statuscode = self.status();
        statuscode.into_response()
    }
}
