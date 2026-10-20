use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};
use thiserror::Error;

// Error managment
// first line: displaye into the terminal
// second line: define the enum error
#[derive(Error, Debug)]
pub enum AppError {
    // add the precise error message in the terminal with {0}
    #[error("Askama error: {0}")]
    Template(#[from] askama::Error),

    #[error("Session Error: {0}")]
    Session(#[from] tower_sessions::session::Error),

    #[error("Db Error: {0}")]
    Db(#[from] sqlx::Error),

    #[error("Migration Error: {0}")]
    Migration(#[from] sqlx::migrate::MigrateError),
}

// transform AppError to http response for Axum
// display error message to the user in the browser
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        tracing::error!("{self}");

        let (status, error_message) = match self {
            AppError::Template(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error while creating the page",
            ),
            // session error (cookies or storage)
            AppError::Session(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error session management",
            ),
            // refined error db message (404 for not found, etc.)
            AppError::Db(err) => match err {
                sqlx::Error::RowNotFound => (StatusCode::NOT_FOUND, "Ressource not found"),
                _ => (StatusCode::INTERNAL_SERVER_ERROR, "Error db management"),
            },
            AppError::Migration(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Error during migration"),
        };

        (
            status,
            Html(format!(
                r#"
                    <h1>Error {status}</h1>
                    <p>{error_message}</p>
                    <a href="/" class="mt-8">Go back home</a>
                "#
            )),
        )
            .into_response() // convertit le tuple en Response attendu par Axum
    }
}
