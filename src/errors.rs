use actix_web::{HttpResponse, error::ResponseError, http::StatusCode};
use serde::Serialize;
use thiserror::Error;

// AppError enum with error variants

// impl ResponseError for AppError - Actix error handling

// From<T> implementations for various error types

#[derive(Debug, Serialize)]
pub struct ErrorBody<'a> {
    pub error: ErrorEnvelope<'a>,
}

#[derive(Debug, Serialize)]
pub struct ErrorEnvelope<'a> {
    pub code: &'a str,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Error)]
pub enum AppError {
    #[error("Not found")]
    NotFound,
    #[error("Internal Error")]
    InternalError,
    #[error("Bad request: {0}")]
    BadRequest(String),
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match *self {
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let (code, msg, details) = match self {
            AppError::NotFound => ("NOT_FOUND", "URL NOT FOUND".to_string(), None),
            AppError::InternalError => (
                "INTERNAL_SERVER_ERROR",
                "Internal server error".to_string(),
                None,
            ),
            AppError::BadRequest(m) => ("BAD_REQUEST", m.clone(), None),
        };

        HttpResponse::build(self.status_code()).json(ErrorBody {
            error: ErrorEnvelope {
                code,
                message: msg,
                details,
            },
        })
    }
}
