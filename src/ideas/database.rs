use sqlx::SqlitePool;

use super::templates::Idea;
use crate::errors::AppError;

pub async fn add_idea(idea: &str, db: SqlitePool) -> Result<(), AppError> {
    sqlx::query!(
        "INSERT INTO ideas (description, state) VALUES (?, 'pending')",
        idea
    )
    .execute(&db)
    .await?;

    Ok(())
}

pub async fn update_idea(id: u16, new_idea: &str, db: SqlitePool) -> Result<(), AppError> {
    sqlx::query!("UPDATE ideas SET description=? WHERE id=?", new_idea, id)
        .execute(&db)
        .await?;

    Ok(())
}

pub async fn get_ideas(db: SqlitePool) -> Result<Vec<Idea>, AppError> {
    // get only one column with query_scalar!, as query! return the full row
    let ideas = sqlx::query!("SELECT description, id FROM ideas WHERE state = 'pending'")
        .fetch_all(&db)
        .await?
        .iter()
        .map(|idea| Idea {
            description: idea.description.to_string(),
            id: idea.id as u16,
        })
        .collect();

    Ok(ideas)
}

pub async fn get_idea(id: u16, db: SqlitePool) -> Result<Idea, AppError> {
    let record = sqlx::query!("SELECT description,id FROM ideas WHERE id = ?", id)
        .fetch_one(&db)
        .await?;

    let idea = Idea {
        description: record.description.to_string(),
        id: record.id as u16,
    };

    Ok(idea)
}

pub async fn remove_idea(id: u16, db: SqlitePool) -> Result<(), AppError> {
    sqlx::query!("DELETE FROM ideas WHERE id = ?", id)
        .execute(&db)
        .await?;

    Ok(())
}
