use std::sync::Arc;

use axum::{Router, routing::get};

use super::handlers::root;
use crate::database::AppState;

pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/", get(root))
}
