use crate::{db::DbPool, redis_db::RedisPool};
use axum::{
    extract::{rejection::JsonRejection, FromRequest},
    http::StatusCode,
    response::{IntoResponse, Response},
};

use serde::ser::StdError;
use std::fmt::Display;
use std::sync::Arc;

#[derive(FromRequest)]
#[from_request(via(axum::Json), rejection(AppError))]
pub struct AppJson<T>(pub T);

impl<T> IntoResponse for AppJson<T>
where
    axum::Json<T>: IntoResponse,
{
    fn into_response(self) -> Response {
        axum::Json(self.0).into_response()
    }
}
#[derive(Debug)]
pub enum AppError {
    JsonRejection(JsonRejection),
    DataBaseError(sqlx::Error),
    MappingError(String),
    UnauthorizedError,
    AnyResponsableError(String),
}
impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::JsonRejection(err) => write!(f, "{}", err),
            Self::DataBaseError(err) => write!(f, "{}", err),
            Self::MappingError(description) => write!(f, "{}", description),
            Self::UnauthorizedError => write!(f, "{}", "login or password is wrong!!!"),
            Self::AnyResponsableError(description) => write!(f, "{}", description),
        }
    }
}
impl StdError for AppError {}
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        // How we want errors responses to be serialized
        #[derive(serde::Serialize)]
        struct ErrorResponse {
            status: u16,
            message: String,
        }

        let (status, message) = match self {
            AppError::JsonRejection(rejection) => {
                let description = rejection.to_string();
                tracing::error!(target:"serde events",description );

                // This error is caused by bad user input so don't log it
                (rejection.status(), rejection.body_text())
            }
            AppError::MappingError(description) => {
                tracing::error!(target:"mapping events", description);
                (StatusCode::INTERNAL_SERVER_ERROR, description)
            }
            AppError::UnauthorizedError => (
                StatusCode::UNAUTHORIZED,
                "login or password is wrong!!!".to_string(),
            ),
            AppError::AnyResponsableError(description) => {
                (StatusCode::INTERNAL_SERVER_ERROR, description)
            }
            AppError::DataBaseError(err) => {
                tracing::error!(%err, "error from database");
                let (status, message) = match err {
                    sqlx::Error::Database(err) => (StatusCode::BAD_REQUEST, err.to_string()),
                    _ => (StatusCode::BAD_REQUEST, err.to_string()),
                };
                (status, message)
            }
        };

        (
            status,
            AppJson(ErrorResponse {
                message,
                status: status.as_u16(),
            }),
        )
            .into_response()
    }
}
impl From<String> for AppError {
    fn from(description: String) -> Self {
        Self::AnyResponsableError(description)
    }
}
impl From<JsonRejection> for AppError {
    fn from(rejection: JsonRejection) -> Self {
        Self::JsonRejection(rejection)
    }
}
impl From<sqlx::Error> for AppError {
    fn from(error: sqlx::Error) -> Self {
        Self::DataBaseError(error)
    }
}

#[derive(Clone)]
pub struct AppState {
    pub pool: Arc<DbPool>,
    pub redis_pool: Arc<RedisPool>,
}
