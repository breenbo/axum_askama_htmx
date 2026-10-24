use axum::{
    extract::Request,
    middleware::Next,
    response::{IntoResponse, Redirect, Response},
};
use tower_sessions::Session;

pub async fn require_auth(session: Session, request: Request, next: Next) -> Response {
    match session.get::<i64>("user_id").await {
        Ok(Some(_)) => next.run(request).await,
        _ => Redirect::to("/login").into_response(),
    }
}
