use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};

use crate::{admin::templates::AdminTemplate, base::templates::Base, errors::AppError};

pub async fn display_admin() -> Result<impl IntoResponse, AppError> {
    let admin = AdminTemplate {
        base: Base {
            tab_title: "Admin panel",
            is_logged_in: true,
            is_admin: true,
        },
    };
    let rendered = admin.render()?;

    Ok((StatusCode::OK, Html(rendered)).into_response())
}
