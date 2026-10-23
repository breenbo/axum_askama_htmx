use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize, Validate)]
pub struct LoginForm {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, Validate)]
pub struct RegisterForm {
    pub name: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8, message = "Must be at least 8 chars long"))]
    pub password: String,
}

pub struct User {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub hashed_pwd: String,
}
