use std::sync::Arc;

use axum::Router;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use tower_sessions::{MemoryStore, SessionManagerLayer};

use crate::admin::router as admin;
use crate::auth::router as auth;
use crate::database::AppState;
use crate::ideas::router as ideas;
use crate::root::router as root;

pub fn create_router(state: Arc<AppState>) -> Router {
    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store).with_secure(false);

    Router::new()
        .merge(root::router())
        .merge(auth::router())
        .merge(admin::router())
        .merge(ideas::router())
        .with_state(state)
        .layer(session_layer)
        .fallback(not_found)
}

async fn not_found() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Route not found")
}
