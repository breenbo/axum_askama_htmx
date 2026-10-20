use std::sync::Arc;

use axum::Router;
use axum::extract::State;
use tower_sessions::{MemoryStore, SessionManagerLayer};

use crate::database::AppState;
use crate::ideas::router as ideas;
use crate::root::router as root;

pub fn create_router(State(state): State<Arc<AppState>>) -> Router {
    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store).with_secure(false);

    Router::new()
        .merge(root::router())
        .merge(ideas::router())
        .with_state(state)
        .layer(session_layer)
}
