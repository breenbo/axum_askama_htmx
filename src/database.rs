use std::sync::Arc;

use crate::errors::AppError;
use sqlx::SqlitePool;

// app state to use the db in all components
pub struct AppState {
    pub db: SqlitePool,
}

// use Arc for cheap copy, better than clone
pub async fn connect_sqlite() -> Result<Arc<AppState>, AppError> {
    const DATABASE_URL: &str = "sqlite://database/db.sqlite";
    let db = SqlitePool::connect(DATABASE_URL).await?;
    tracing::info!("Connected to sqlite db");

    sqlx::migrate!("./migrations/").run(&db).await?;
    tracing::info!("Migrations done");

    Ok(Arc::new(AppState { db }))
}
