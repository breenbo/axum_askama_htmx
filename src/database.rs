use crate::{errors::AppError, templates::Idea};
use sqlx::SqlitePool;

// app state to use the db in all components
#[derive(Clone)]
pub struct AppState {
    pub db: SqlitePool,
}

pub async fn connect_sqlite() -> Result<AppState, AppError> {
    const DATABASE_URL: &str = "sqlite://database/db.sqlite";
    let db = SqlitePool::connect(DATABASE_URL).await?;
    tracing::info!("Connected to sqlite db");

    sqlx::migrate!("./migrations/").run(&db).await?;
    tracing::info!("Migrations done");

    Ok(AppState { db })
}

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
