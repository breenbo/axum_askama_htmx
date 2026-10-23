use std::sync::Arc;

use axum::{
    Router,
    routing::{get, post},
};

use super::handlers::{
    create_idea, delete_idea, display_create_idea, display_edit_idea, display_idea, display_ideas,
    edit_idea,
};
use crate::database::AppState;

// return Arc<AppState> because it will be added after the merge to have a full Router
pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/ideas", get(display_ideas).post(create_idea))
        .route("/ideas/create", get(display_create_idea))
        .route("/ideas/{id}", get(display_idea))
        .route("/ideas/{id}/edit", get(display_edit_idea).post(edit_idea))
        .route("/ideas/{id}/delete", post(delete_idea))
}
