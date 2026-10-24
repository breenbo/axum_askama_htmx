use axum::{extract::FromRequestParts, http::request::Parts};
use tower_sessions::Session;

use crate::errors::AppError;

pub struct CurrentUser(pub Option<i64>);

impl<S> FromRequestParts<S> for CurrentUser
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        // récupère la session depuis SessionLayoutManager
        let session = Session::from_request_parts(parts, state)
            .await
            .map_err(|_| AppError::SessionLayerMissing)?;

        let user_id = session.get::<i64>("user_id").await?;

        Ok(CurrentUser(user_id))
    }
}
