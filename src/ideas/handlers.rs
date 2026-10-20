use std::sync::Arc;

use askama::Template;
use axum::{
    Form,
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse, Redirect},
};

use super::{
    database::{add_idea, get_idea, get_ideas, remove_idea, update_idea},
    models::IdeaForm,
    templates::{EditIdeaTemplate, Idea, IdeaTemplate, IdeasTemplate},
};
use crate::{database::AppState, errors::AppError};

pub async fn display_ideas(
    // access directly the sqlite connection
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, AppError> {
    tracing::info!("Get /home");

    // last unwrap to manage the Option inside the Result
    // Error in Result managed by ?
    let ids: Vec<Idea> = get_ideas(Arc::clone(&state)).await?;

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
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, AppError> {
    tracing::info!("Get idea {id}");

    let ida = get_idea(id, Arc::clone(&state)).await?;
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
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, AppError> {
    let idea = get_idea(id, Arc::clone(&state)).await?;

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
    State(state): State<Arc<AppState>>,
    // !!!!!!!!! form always last parameter !!!!!!!!!
    Form(form): Form<IdeaForm>,
) -> Result<Redirect, AppError> {
    tracing::info!("Update idea {id}");

    update_idea(id, &form.idea, Arc::clone(&state)).await?;

    Ok(Redirect::to(&format!("/ideas/{id}")))
}

// Form() allow to decode strings sent from post request
pub async fn create_idea(
    // access directly the sqlite connection
    State(state): State<Arc<AppState>>,
    // !!!!!!!!! form always last parameter !!!!!!!!!
    Form(form): Form<IdeaForm>,
) -> Result<Redirect, AppError> {
    tracing::info!("Post /ideas : {}", form.idea);

    add_idea(&form.idea, Arc::clone(&state)).await?;

    Ok(Redirect::to("/ideas"))
}

pub async fn delete_idea(
    Path(id): Path<u16>,
    State(state): State<Arc<AppState>>,
) -> Result<Redirect, AppError> {
    tracing::info!("Delete ideas : {}", id);

    remove_idea(id, Arc::clone(&state)).await?;

    Ok(Redirect::to("/ideas"))
}
