use std::sync::Arc;

use anyhow::Result;
use askama::Template;
use axum::{
    Form,
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse, Redirect, Response},
};
use validator::Validate;

use super::{
    database::{add_idea, get_idea, get_ideas, remove_idea, update_idea},
    models::IdeaForm,
    templates::{EditIdeaTemplate, Idea, IdeaTemplate, IdeasTemplate},
};
use crate::{database::AppState, errors::AppError, ideas::templates::CreateIdeaTemplate};

pub async fn display_ideas(
    // access directly the sqlite connection
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, AppError> {
    tracing::info!("Get /home");

    // last unwrap to manage the Option inside the Result
    // Error in Result managed by ?
    let idas: Vec<Idea> = get_ideas(Arc::clone(&state)).await?;

    let ideas = IdeasTemplate {
        ideas: idas,
        ..Default::default()
    };

    let rendered = ideas.render()?;

    Ok((StatusCode::OK, Html(rendered)))
}

pub async fn display_create_idea() -> Result<impl IntoResponse, AppError> {
    let create_idea = CreateIdeaTemplate::default();
    let rendered = create_idea.render()?;

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
        idea: ida,
        ..Default::default()
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
        idea,
        ..Default::default()
    };

    let rendered = edit_idea.render()?;

    Ok((StatusCode::OK, Html(rendered)))
}

// Form() allow to decode strings sent from post request
pub async fn create_idea(
    // access directly the sqlite connection
    State(state): State<Arc<AppState>>,
    // !!!!!!!!! form always last parameter !!!!!!!!!
    Form(form): Form<IdeaForm>,
) -> Result<Response, AppError> {
    tracing::info!("Post /ideas : {}", form.description);

    if let Some(error_message) = get_error_message("description", &form) {
        let template = CreateIdeaTemplate::with_error_message(error_message, form.description);
        let rendered = template.render()?;

        return Ok((StatusCode::UNPROCESSABLE_ENTITY, Html(rendered)).into_response());
    }

    add_idea(&form.description, Arc::clone(&state)).await?;

    Ok(Redirect::to("/ideas").into_response())
}

pub async fn edit_idea(
    Path(id): Path<u16>,
    State(state): State<Arc<AppState>>,
    // !!!!!!!!! form always last parameter !!!!!!!!!
    Form(form): Form<IdeaForm>,
) -> Result<Response, AppError> {
    tracing::info!("Update idea {id}");

    if let Some(error_message) = get_error_message("description", &form) {
        let template = EditIdeaTemplate::with_error_message(
            error_message,
            Idea {
                id,
                description: form.description,
            },
        );
        let rendered = template.render()?;

        return Ok((StatusCode::UNPROCESSABLE_ENTITY, Html(rendered)).into_response());
    }

    update_idea(id, &form.description, Arc::clone(&state)).await?;

    Ok(Redirect::to(&format!("/ideas/{id}")).into_response())
}

pub async fn delete_idea(
    Path(id): Path<u16>,
    State(state): State<Arc<AppState>>,
) -> Result<Redirect, AppError> {
    tracing::info!("Delete ideas : {}", id);

    remove_idea(id, Arc::clone(&state)).await?;

    Ok(Redirect::to("/ideas"))
}

// helper to get the error on validate a fiels in a form
fn get_error_message(field: &str, form: &IdeaForm) -> Option<String> {
    if let Err(validation_error) = form.validate() {
        // get the first error message
        let error_message = validation_error
            .field_errors()
            .get(field)
            .and_then(|errors| errors.first())
            .and_then(|err| err.message.as_ref())
            .map(|m| m.to_string())
            .unwrap_or_else(|| "Invalid input".to_string());

        tracing::warn!("Post /ideas error: {}", error_message);

        Some(error_message)
    } else {
        None
    }
}
