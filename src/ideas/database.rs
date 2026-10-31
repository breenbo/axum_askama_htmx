use std::sync::Arc;

use super::templates::Idea;
use crate::{database::AppState, errors::AppError};

pub async fn add_idea(
    idea: &str,
    app_state: &Arc<AppState>,
    user_id: Option<i64>,
) -> Result<(), AppError> {
    if let Some(user_id) = user_id {
        sqlx::query!(
            "INSERT INTO ideas (description, state, user_id) VALUES (?, 'pending',?)",
            idea,
            user_id
        )
        .execute(&app_state.db)
        .await?;
    }

    Ok(())
}

pub async fn update_idea(
    id: u16,
    new_idea: &str,
    app_state: &Arc<AppState>,
) -> Result<(), AppError> {
    sqlx::query!("UPDATE ideas SET description=? WHERE id=?", new_idea, id)
        .execute(&app_state.db)
        .await?;

    Ok(())
}

pub async fn get_ideas(
    app_state: &Arc<AppState>,
    user_id: Option<i64>,
) -> Result<Vec<Idea>, AppError> {
    // get only one column with query_scalar!, as query! return the full row
    let ideas = if let Some(user_id) = user_id {
        sqlx::query!(
            "SELECT description, id FROM ideas WHERE state = 'pending' AND user_id = ?",
            user_id
        )
        .fetch_all(&app_state.db)
        .await?
        .iter()
        .map(|idea| Idea {
            description: idea.description.to_string(),
            id: idea.id as u16,
        })
        .collect()
    } else {
        Vec::new()
    };

    Ok(ideas)
}

pub async fn get_idea(id: u16, app_state: &Arc<AppState>) -> Result<Idea, AppError> {
    let record = sqlx::query!("SELECT description,id FROM ideas WHERE id = ?", id)
        .fetch_one(&app_state.db)
        .await?;

    let idea = Idea {
        description: record.description.to_string(),
        id: record.id as u16,
    };

    Ok(idea)
}

pub async fn remove_idea(id: u16, app_state: &Arc<AppState>) -> Result<(), AppError> {
    sqlx::query!("DELETE FROM ideas WHERE id = ?", id)
        .execute(&app_state.db)
        .await?;

    Ok(())
}
