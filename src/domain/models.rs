// src/domain/models.rs
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Item {
    pub id: i32,
    pub nome: String,
}
