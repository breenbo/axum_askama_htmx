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
