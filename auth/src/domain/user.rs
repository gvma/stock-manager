use chrono::{NaiveDateTime};
use serde::{Serialize, Deserialize};

use super::Claims;

#[derive(Serialize, Deserialize, sqlx::FromRow, Clone)]
pub struct User {
    pub uuid: String,
    pub name: String,
    pub email: String,

    #[serde(skip_serializing)]
    pub password: String,

    #[serde(skip_serializing)]
    pub password_reset_code: Option<String>,
    #[serde(skip_serializing)]
    pub password_reset_code_expires_at: Option<NaiveDateTime>
}

impl Into<SignInOutput> for User {
    fn into(self) -> SignInOutput {
        SignInOutput {
            name: self.name.clone(),
            email: self.email.clone(),
            token: Claims::from(self).into()
        }
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct SignInInput {
    pub email: String,
    pub password: String
}

#[derive(Serialize, Deserialize, Default)]
pub struct SignInOutput {
    pub name: String,
    pub email: String,
    pub token: String
}

#[derive(Serialize, Deserialize, Default)]
pub struct SignUpInput {
    pub name: String,
    pub email: String,
    pub password: String
}

#[derive(Serialize, Deserialize, Default)]
pub struct ResetPasswordInput {
    pub email: String
}