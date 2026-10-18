use crate::errors::AppError;
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
    sqlx::query!("INSERT INTO ideas (description) VALUES (?)", idea)
        .execute(&db)
        .await?;

    Ok(())
}

pub async fn get_db_ideas(db: SqlitePool) -> Result<Vec<String>, AppError> {
    // get only one column with query_scalar!, as query! return the full row
    let ideas = sqlx::query_scalar!("SELECT description FROM ideas")
        .fetch_all(&db)
        .await?;

    Ok(ideas)
}
