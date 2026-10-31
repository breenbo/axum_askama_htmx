use std::sync::Arc;

use axum::{Router, routing::get};

use crate::database::AppState;

use super::handlers::display_admin;

pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/admin", get(display_admin))
}
