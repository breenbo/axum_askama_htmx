use axum::{Router, routing::get};

use super::handlers::*;
use crate::database::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/ideas", get(display_ideas).post(create_idea))
        .route("/ideas/{id}", get(display_idea))
        .route("/ideas/{id}/edit", get(display_edit_idea).post(edit_idea))
        .route(
            "/ideas/{id}/delete",
            get(display_edit_idea).post(delete_idea),
        )
}
