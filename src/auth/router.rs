use std::sync::Arc;

use axum::{
    Router, middleware,
    routing::{get, post},
};

use super::handlers::{
    display_login_form, display_register_form, login_user, logout_user, register_user,
};
use crate::{
    auth::middlewares::{guest_only, require_auth},
    database::AppState,
};

pub fn router() -> Router<Arc<AppState>> {
    let protected = Router::new()
        .route("/logout", post(logout_user))
        .route_layer(middleware::from_fn(require_auth));

    let guest_only = Router::new()
        .route("/register", get(display_register_form).post(register_user))
        .route("/login", get(display_login_form).post(login_user))
        .route_layer(middleware::from_fn(guest_only));

    guest_only.merge(protected)
}
