use std::sync::Arc;

use argon2::{Argon2, PasswordHash, PasswordVerifier};
use askama::Template;
use axum::{
    Form,
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse, Redirect},
};
use tower_sessions::Session;

use super::{
    database::{create_user, get_user},
    models::{LoginForm, RegisterForm},
    templates::{LoginTemplate, RegisterTemplate},
};
use crate::{base::templates::Base, database::AppState, errors::AppError};

pub async fn display_register_form() -> Result<impl IntoResponse, AppError> {
    let register = RegisterTemplate::default();
    let rendered = register.render()?;

    Ok((StatusCode::OK, Html(rendered)))
}

pub async fn register_user(
    State(state): State<Arc<AppState>>,
    session: Session,
    Form(user): Form<RegisterForm>,
) -> Result<impl IntoResponse, AppError> {
    tracing::info!("Register user: {}", user.email);

    // match the error of insert into sql to check if email is already taken
    match create_user(&user, Arc::clone(&state)).await {
        Ok(user_id) => {
            tracing::info!("User created");
            log_in_session(&session, user_id).await?;

            Ok(Redirect::to("/ideas").into_response())
        }
        Err(AppError::EmailAlreadyTaken) => {
            tracing::error!("Error creating ");
            let reg = RegisterTemplate {
                base: Base::default(),
                name: user.name,
                email: user.email,
                error: Some("Email already taken"),
            };
            let rendered = reg.render()?;

            Ok((StatusCode::CONFLICT, Html(rendered)).into_response())
        }
        Err(e) => Err(e),
    }
}

pub async fn display_login_form() -> Result<impl IntoResponse, AppError> {
    let login = LoginTemplate {
        base: Base {
            tab_title: "Login",
            is_logged_in: false,
        },
    };
    let rendered = login.render()?;

    Ok((StatusCode::OK, Html(rendered)))
}

pub async fn login_user(
    State(state): State<Arc<AppState>>,
    session: Session,
    Form(form): Form<LoginForm>,
) -> Result<impl IntoResponse, AppError> {
    // get user in db with email
    let user = get_user(&form.email, &state).await?;
    // compare password
    let is_valid = verify_password(&form.password, &user.hashed_pwd);
    // if match, render template with logged_in: true in base template?
    if is_valid {
        log_in_session(&session, user.id).await?;

        // need to render something
        Ok(Redirect::to("/ideas"))
    } else {
        tracing::warn!("Bad credentials");

        // TODO: template with error in form, like the register
        Ok(Redirect::to("/login"))
    }
}

pub async fn logout_user(session: Session) -> Result<impl IntoResponse, AppError> {
    session.remove::<i64>("user_id").await?;
    tracing::info!("User logged out");

    Ok(Redirect::to("/login"))
}

fn verify_password(pwd: &str, stored_hash: &str) -> bool {
    let parsed_hash = match PasswordHash::new(stored_hash) {
        Ok(hash) => hash,
        Err(_) => return false,
    };

    Argon2::default()
        .verify_password(pwd.as_bytes(), &parsed_hash)
        .is_ok()
}

async fn log_in_session(session: &Session, user_id: i64) -> Result<(), AppError> {
    session.cycle_id().await?;
    session.insert("user_id", user_id).await?;
    tracing::info!("User logged in: {}", user_id);

    Ok(())
}
