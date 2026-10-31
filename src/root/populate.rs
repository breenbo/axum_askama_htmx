use std::{ops::Range, sync::Arc};

use askama::Template;
use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse},
};

use crate::{
    auth::{database::create_user, models::RegisterForm},
    base::templates::Base,
    database::AppState,
    errors::AppError,
    ideas::database::add_idea,
    root::templates::RootTemplate,
};

// populate db with 5 users, all passwords to hashed("aqwxszedc")
// and 10 ideas per user
// muste be triggered on root page asking POST on "/populate" route
//
const USER_RANGE: Range<u8> = 1..6;
const IDEA_RANGE: Range<u8> = 1..21;

async fn populate_users(app_state: &Arc<AppState>) -> Result<(), AppError> {
    for i in USER_RANGE {
        let user = RegisterForm {
            name: format!("bruno_{i}"),
            email: format!("beru_{i}@gmail.com"),
            password: "aqwxszedc".to_string(),
            role: if i == 1 {
                Some("admin".to_string())
            } else {
                Some("user".to_string())
            },
        };

        create_user(&user, app_state).await?;
    }

    Ok(())
}

async fn populate_ideas(app_state: &Arc<AppState>) -> Result<(), AppError> {
    // for each user, create 20 ideas
    for i in USER_RANGE {
        for j in IDEA_RANGE {
            let description = format!("Idea_{j} for bruno_{i}");

            add_idea(&description, app_state, Some(i as i64)).await?;
        }
    }

    Ok(())
}

async fn reset_db(app_state: &Arc<AppState>) -> Result<(), AppError> {
    sqlx::query!("DELETE FROM ideas")
        .execute(&app_state.db)
        .await?;
    sqlx::query!("DELETE FROM users")
        .execute(&app_state.db)
        .await?;
    sqlx::query!("DELETE FROM sqlite_sequence WHERE name IN ('users','ideas')")
        .execute(&app_state.db)
        .await?;

    Ok(())
}

pub async fn populate_db(
    State(app_state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, AppError> {
    tracing::info!("populate tables");
    reset_db(&app_state).await?;
    populate_users(&app_state).await?;
    populate_ideas(&app_state).await?;

    let root = RootTemplate {
        base: Base {
            ..Default::default()
        },
    };

    let rendered = root.render()?;

    Ok((StatusCode::OK, Html(rendered)))
}
