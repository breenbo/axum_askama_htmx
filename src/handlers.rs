use askama::Template;
use axum::{
    Form,
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse, Redirect},
};

use crate::{
    database::{AppState, add_idea, get_idea, get_ideas, update_idea},
    errors::AppError,
    models::IdeaForm,
    templates::{EditIdeaTemplate, Idea, IdeaTemplate, IdeasTemplate, RootTemplate},
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

pub async fn display_ideas(
    // access directly the sqlite connection
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    tracing::info!("Get /home");

    // last unwrap to manage the Option inside the Result
    // Error in Result managed by ?
    let ids: Vec<Idea> = get_ideas(state.db).await?;

    let ideas = IdeasTemplate {
        tab_title: "Axum ideas",
        title: "Axum ideas",
        ideas: ids,
    };

    let rendered = ideas.render()?;

    Ok((StatusCode::OK, Html(rendered)))
}

// get one idea, the id is passed in the url "ideas/1" => use Path to get the value
pub async fn display_idea(
    Path(id): Path<u16>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    tracing::info!("Get idea {id}");

    let ida = get_idea(id, state.db).await?;
    let idea = IdeaTemplate {
        tab_title: "Axum Idea",
        title: "Axum Idea",
        idea: ida,
    };

    let rendered = idea.render()?;

    Ok((StatusCode::OK, Html(rendered)))
}

pub async fn display_edit_idea(
    Path(id): Path<u16>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let idea = get_idea(id, state.db).await?;

    let edit_idea = EditIdeaTemplate {
        tab_title: "Axum Idea",
        title: "Axum Idea",
        idea,
    };

    let rendered = edit_idea.render()?;

    Ok((StatusCode::OK, Html(rendered)))
}

pub async fn edit_idea(
    Path(id): Path<u16>,
    State(state): State<AppState>,
    // !!!!!!!!! form always last parameter !!!!!!!!!
    Form(form): Form<IdeaForm>,
) -> Result<Redirect, AppError> {
    tracing::info!("Update idea {id}");

    update_idea(id, &form.idea, state.db).await?;

    Ok(Redirect::to(&format!("/ideas/{id}")))
}

// Form() allow to decode strings sent from post request
pub async fn create_idea(
    // access directly the sqlite connection
    State(state): State<AppState>,
    // !!!!!!!!! form always last parameter !!!!!!!!!
    Form(form): Form<IdeaForm>,
) -> Result<Redirect, AppError> {
    tracing::info!("Post /ideas : {}", form.idea);

    add_idea(&form.idea, state.db).await?;

    Ok(Redirect::to("/ideas"))
}
