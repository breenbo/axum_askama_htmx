use std::sync::Arc;

use axum::{Router, routing::get};

use super::handlers::*;
use crate::database::AppState;

// return Arc<AppState> because it will be added after the merge to have a full Router
pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/ideas", get(display_ideas).post(create_idea))
        .route("/ideas/{id}", get(display_idea))
        .route("/ideas/{id}/edit", get(display_edit_idea).post(edit_idea))
        .route(
            "/ideas/{id}/delete",
            get(display_edit_idea).post(delete_idea),
        )
}
