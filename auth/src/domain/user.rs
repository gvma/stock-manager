use std::env;

use chrono::{Duration, NaiveDateTime, TimeDelta, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Serialize, Deserialize};

use super::Claims;

const AUTH_TOKEN_VALIDITY: TimeDelta = Duration::hours(24);

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub email: String,
    pub name: Option<String>,

    #[serde(skip_serializing)]
    pub password: String,

    #[serde(skip_serializing)]
    pub password_reset_code: Option<String>,
    #[serde(skip_serializing)]
    pub password_reset_code_expires_at: Option<NaiveDateTime>
}

impl User {
    pub fn to_sign_in_output(&self) -> SignInOutput {
        SignInOutput {
            email: self.email.clone(),
            name: self.name.clone().unwrap(),
            token: self.auth_token()
        }
    }
    
    pub fn auth_token(&self) -> String {
        let claims = Claims { sub: self.email.to_owned(), exp: (Utc::now() + AUTH_TOKEN_VALIDITY).timestamp() as usize };
        let secret = env::var("TOKEN_ENCRYPT_SECRET").expect("TOKEN_ENCRYPT_SECRET não definido");

        encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref())).unwrap()
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct SignInOutput {
    pub email: String,
    pub name: String,
    pub token: String
}

#[derive(Serialize, Deserialize, Default)]
pub struct ResetPasswordInput {
    pub email: String
}