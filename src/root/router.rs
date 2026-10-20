use axum::{Router, routing::get};

use super::handlers::root;
use crate::database::AppState;

pub fn router() -> Router<AppState> {
    Router::new().route("/", get(root))
}
