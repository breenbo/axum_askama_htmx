use axum::{Router, routing::get};
use tower_sessions::{MemoryStore, SessionManagerLayer};

use crate::handlers::{create_idea, get_ideas, root};

pub fn create_router() -> Router {
    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store).with_secure(false);

    Router::new().route("/", get(root)).route(
        "/ideas",
        get(get_ideas).post(create_idea).layer(session_layer),
    )
}
