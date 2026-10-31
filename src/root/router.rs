use std::sync::Arc;

use axum::{
    Router,
    routing::{get, post},
};

use super::{handlers::root, populate::populate_db};
use crate::database::AppState;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(root))
        .route("/populate", post(populate_db))
}
