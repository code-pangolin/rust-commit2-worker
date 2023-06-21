use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum HandlerError {
    #[error("internal server error: {0} ")]
    InternalServerError(anyhow::Error),
    #[error("bad request: {0} ")]
    BadRequest(anyhow::Error),
}

impl HandlerError {
    pub fn status(&self) -> StatusCode {
        match self {
            HandlerError::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            HandlerError::BadRequest(_) => StatusCode::BAD_REQUEST,
        }
    }

    pub fn error(&self) -> &anyhow::Error {
        match self {
            HandlerError::InternalServerError(e) => e,
            HandlerError::BadRequest(e) => e,
        }
    }
}

impl IntoResponse for HandlerError {
    fn into_response(self) -> Response {
        let statuscode = self.status();
        statuscode.into_response()
    }
}
