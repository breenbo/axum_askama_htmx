use axum::{Router, routing::get};
use tower_sessions::{MemoryStore, SessionManagerLayer};

use crate::{
    database::AppState,
    handlers::{
        create_idea, delete_idea, display_edit_idea, display_idea, display_ideas, edit_idea, root,
    },
};

pub fn create_router(state: AppState) -> Router {
    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store).with_secure(false);

    Router::new()
        .route("/", get(root))
        .route(
            "/ideas",
            get(display_ideas).post(create_idea).layer(session_layer),
        )
        .route("/ideas/{id}", get(display_idea))
        .route("/ideas/{id}/edit", get(display_edit_idea).post(edit_idea))
        .route(
            "/ideas/{id}/delete",
            get(display_edit_idea).post(delete_idea),
        )
        .with_state(state)
}
