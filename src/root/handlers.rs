use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};

use super::templates::RootTemplate;
use crate::errors::AppError;

// all function used in routes
//
pub async fn root() -> Result<impl IntoResponse, AppError> {
    tracing::info!("Get / root");
    let root = RootTemplate {
        tab_title: "Some title",
        title: "Awesome ideas",
    };

    let rendered = root.render()?;

    Ok((StatusCode::OK, Html(rendered)))
}
