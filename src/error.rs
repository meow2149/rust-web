use std::borrow::Cow;

use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use sea_orm::DbErr;
use serde::Serialize;
use thiserror::Error;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("{0}")]
    NotFound(String),
    #[error("{0}")]
    BadRequest(String),
    #[error("database error: {0}")]
    Database(#[from] DbErr),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

impl AppError {
    pub fn not_found(message: impl Into<String>) -> Self {
        Self::NotFound(message.into())
    }

    pub fn bad_request(message: impl Into<String>) -> Self {
        Self::BadRequest(message.into())
    }
}

#[derive(Serialize)]
struct ErrorBody<'a> {
    error: &'a str,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::NotFound(message) => (StatusCode::NOT_FOUND, Cow::Owned(message)),
            AppError::BadRequest(message) => (StatusCode::BAD_REQUEST, Cow::Owned(message)),
            AppError::Database(err) => {
                tracing::error!(target = "database", error = %err, "database error occurred");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Cow::Borrowed("internal server error"),
                )
            }
            AppError::Internal(err) => {
                tracing::error!(error = %err, "internal error");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Cow::Borrowed("internal server error"),
                )
            }
        };

        let body = Json(ErrorBody { error: &message });
        (status, body).into_response()
    }
}
