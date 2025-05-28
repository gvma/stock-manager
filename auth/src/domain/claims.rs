use std::env;

use chrono::{Duration, TimeDelta, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Serialize, Deserialize};

use super::User;

const AUTH_TOKEN_VALIDITY: TimeDelta = Duration::hours(24);

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
  pub sub: String,
  pub exp: usize
}

impl Into<String> for Claims {
    fn into(self) -> String {
        let secret = env::var("TOKEN_ENCRYPT_SECRET").expect("TOKEN_ENCRYPT_SECRET não definido");

        encode(&Header::default(), &self, &EncodingKey::from_secret(secret.as_ref())).unwrap()
    }
}

impl From<User> for Claims {
    fn from(value: User) -> Self {
        Claims {
            sub: value.uuid.to_owned(),
            exp: (Utc::now() + AUTH_TOKEN_VALIDITY).timestamp() as usize
        }
    }
}