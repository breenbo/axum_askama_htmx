use askama::Template;
use axum::{
    Form,
    http::StatusCode,
    response::{Html, IntoResponse, Redirect},
};
use tower_sessions::Session;

use crate::{
    errors::AppError,
    models::IdeaForm,
    templates::{IdeasTemplate, RootTemplate},
};

// all function used in routes
//
pub async fn root() -> Result<impl IntoResponse, AppError> {
    tracing::info!("Get / root");
    let root = RootTemplate {
        title: "Some title",
        some_data: "Some data to display",
    };

    let rendered = root.render()?;

    Ok((StatusCode::OK, Html(rendered)))
}

pub async fn get_ideas(session: Session) -> Result<impl IntoResponse, AppError> {
    tracing::info!("Get /home");

    // last unwrap to manage the Option inside the Result
    // Error in Result managed by ?
    let ids: Vec<String> = session.get("ideas").await?.unwrap_or_default();

    let ideas = IdeasTemplate {
        title: "Ideas".to_string(),
        ideas: ids,
    };

    let rendered = ideas.render()?;

    Ok((StatusCode::OK, Html(rendered)))
}

// Form() allow to decode strings sent from post request
pub async fn create_idea(
    session: Session,
    Form(form): Form<IdeaForm>,
) -> Result<Redirect, AppError> {
    tracing::info!("{}", format!("Post /ideas : {}", form.idea));

    let mut ids: Vec<String> = session.get("ideas").await?.unwrap_or_default();

    ids.push(form.idea);

    tracing::info!("all ids: {:?}", ids);

    // Store back
    session.insert("ideas", ids).await?;

    Ok(Redirect::to("/ideas"))
}
