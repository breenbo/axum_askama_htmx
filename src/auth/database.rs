use std::sync::Arc;

use argon2::{Argon2, PasswordHasher, password_hash::Error};
use password_hash::phc::SaltString;

use super::models::{RegisterForm, User};
use crate::{database::AppState, errors::AppError};

fn hash_password(pwd: &str) -> Result<String, Error> {
    let salt = SaltString::generate();

    Ok(Argon2::default()
        .hash_password_with_salt(pwd.as_bytes(), salt.as_bytes())?
        .to_string())
}

pub async fn create_user(user: &RegisterForm, app_state: Arc<AppState>) -> Result<i64, AppError> {
    let hashed_pwd = hash_password(&user.password)?;

    let result = sqlx::query!(
        "INSERT INTO users (name, email, password) VALUES (?,?,?)",
        user.name,
        user.email,
        hashed_pwd,
    )
    .execute(&app_state.db)
    .await;

    // get the error on insert => email already taken because unique in table definition
    match result {
        Ok(res) => Ok(res.last_insert_rowid()),
        Err(sqlx::Error::Database(e)) if e.is_unique_violation() => {
            Err(AppError::EmailAlreadyTaken)
        }
        Err(e) => Err(e.into()),
    }
}

pub async fn get_user(email: &str, app_state: &Arc<AppState>) -> Result<User, AppError> {
    let res = sqlx::query!("SELECT * FROM users WHERE email = ?", email)
        .fetch_one(&app_state.db)
        .await?;

    Ok(User {
        id: res.id,
        name: res.name,
        email: res.email,
        hashed_pwd: res.password,
    })
}
