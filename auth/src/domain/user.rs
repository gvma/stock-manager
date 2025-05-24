use std::env;

use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Serialize, Deserialize};

use super::claims::Claims;

#[derive(Serialize, Deserialize, sqlx::FromRow, Default)]
pub struct User {
    pub email: String,
    pub name: String,

    #[serde(skip_serializing)]
    pub password: String
}

impl User {
    pub fn to_sign_in_output(&self) -> SignInOutput {
        SignInOutput {
            email: self.email.clone(),
            name: self.name.clone(),
            token: self.auth_token()
        }
    }
    
    pub fn auth_token(&self) -> String {
        let claims = Claims { sub: self.email.to_owned(), exp: (Utc::now() + Duration::hours(24)).timestamp() as usize };
        let secret = env::var("TOKEN_ENCRYPT_SECRET").expect("TOKEN_ENCRYPT_SECRET não definido");

        encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref())).unwrap()
    }
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