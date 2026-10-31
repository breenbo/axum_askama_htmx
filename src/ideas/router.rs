use std::sync::Arc;

use axum::{
    Router, middleware,
    routing::{get, post},
};

use super::handlers::{
    create_idea, delete_idea, display_create_idea, display_edit_idea, edit_idea, index_ideas,
    show_idea,
};
use crate::{auth::middlewares::require_auth, database::AppState};

// return Arc<AppState> because it will be added after the merge to have a full Router
pub fn router() -> Router<Arc<AppState>> {
    let protected = Router::new()
        .route("/ideas/create", get(display_create_idea))
        .route("/ideas/{id}/edit", get(display_edit_idea).post(edit_idea))
        .route("/ideas/{id}/delete", post(delete_idea))
        .route("/ideas", get(index_ideas).post(create_idea))
        .route("/ideas/{id}", get(show_idea))
        .route_layer(middleware::from_fn(require_auth));

    let public = Router::new();

    public.merge(protected)
}
