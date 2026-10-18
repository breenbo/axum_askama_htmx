use askama::Template;
use axum::{
    Form,
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse, Redirect},
};

use crate::{
    database::{AppState, add_idea, get_db_ideas},
    errors::AppError,
    models::IdeaForm,
    templates::{IdeasTemplate, RootTemplate},
};

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

pub async fn get_ideas(
    // access directly the sqlite connection
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    tracing::info!("Get /home");

    // last unwrap to manage the Option inside the Result
    // Error in Result managed by ?
    let ids: Vec<String> = get_db_ideas(state.db).await?;

    let ideas = IdeasTemplate {
        tab_title: "Ideas",
        title: "Create ideas",
        ideas: ids,
    };

    let rendered = ideas.render()?;

    Ok((StatusCode::OK, Html(rendered)))
}

// Form() allow to decode strings sent from post request
pub async fn create_idea(
    // access directly the sqlite connection
    State(state): State<AppState>,
    Form(form): Form<IdeaForm>,
) -> Result<Redirect, AppError> {
    tracing::info!("Post /ideas : {}", form.idea);

    add_idea(&form.idea, state.db).await?;

    Ok(Redirect::to("/ideas"))
}
