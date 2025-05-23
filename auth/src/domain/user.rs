// src/domain/models.rs
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, sqlx::FromRow, Default)]
pub struct User {
    pub email: String,
    pub name: String,

    #[serde(skip_serializing)]
    pub password: String
}

#[derive(Serialize, Deserialize, Default)]
pub struct SignInInput {
    pub email: String,
    pub password: String
}

#[derive(Serialize, Deserialize, Default)]
pub struct SignInOutput {
    pub email: String,
    pub name: String,
    pub token: String
}