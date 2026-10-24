use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};

use super::templates::RootTemplate;
use crate::{auth::extractors::CurrentUser, base::templates::Base, errors::AppError};

// all function used in routes
//
pub async fn root(user: CurrentUser) -> Result<impl IntoResponse, AppError> {
    tracing::info!("Get / root");
    let root = RootTemplate {
        base: Base {
            tab_title: "Some title",
            is_logged_in: user.0.is_some(),
        },
    };

    let rendered = root.render()?;

    Ok((StatusCode::OK, Html(rendered)))
}
