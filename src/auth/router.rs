use std::sync::Arc;

use axum::{
    Router,
    routing::{get, post},
};

use super::handlers::{
    display_login_form, display_register_form, login_user, logout_user, register_user,
};
use crate::database::AppState;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/register", get(display_register_form).post(register_user))
        .route("/login", get(display_login_form).post(login_user))
        .route("/logout", post(logout_user))
}
