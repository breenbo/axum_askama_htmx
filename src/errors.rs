use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};
use thiserror::Error;

// Error managment
#[derive(Error, Debug)]
pub enum AppError {
    #[error("Failed to render template")]
    Template(#[from] askama::Error),

    #[error("Session Error")]
    Session(#[from] tower_sessions::session::Error),
}

// transform AppError to http response for Axum
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        tracing::error!("{self}");

        let (status, error_message) = match self {
            AppError::Template(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error while creating the page",
            ),
            AppError::Session(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error session management",
            ),
        };

        (
            status,
            Html(format!("<h1>Error</h1><p>{error_message}</p>")),
        )
            .into_response() // convertit le tuple en Response attendu par Axum
    }
}
