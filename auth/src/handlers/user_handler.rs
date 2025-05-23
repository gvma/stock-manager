use axum::{Extension, Json, http::StatusCode};
use sqlx::PgPool;
use bcrypt::{DEFAULT_COST, hash, verify};
use chrono::{Utc, Duration};
use jsonwebtoken::{encode, EncodingKey, Header};
use std::env;
use crate::repository::user_repository::UserRepository;
use crate::domain::user::{User, SignInInput, SignInOutput};
use crate::domain::claims::Claims;

pub async fn sign_in(Extension(pool): Extension<PgPool>, Json(user): Json<SignInInput>) -> Result<Json<SignInOutput>, (StatusCode, String)> {
  match UserRepository::get_by_email(&pool, user.email).await {
    Ok(found_user) => {
      if verify(user.password, &found_user.password).unwrap() {
        let my_claims = Claims { sub: found_user.email.to_owned(), exp: (Utc::now() + Duration::hours(24)).timestamp() as usize };
        let secret = env::var("TOKEN_ENCRYPT_SECRET").expect("TOKEN_ENCRYPT_SECRET não definido");
        let token = encode(&Header::default(), &my_claims, &EncodingKey::from_secret(secret.as_ref())).unwrap();
        let response = SignInOutput{
          email: found_user.email,
          name: found_user.name,
          token
        };
        Ok(Json(response))
      } else {
        Err((StatusCode::NOT_FOUND, "Usuário não encontrado".to_string()))
      }
    },
    Err(_) => Err((StatusCode::INTERNAL_SERVER_ERROR, "Erro ao buscar usuário".to_string())),
  }
}

pub async fn sign_up(Extension(pool): Extension<PgPool>, Json(mut user): Json<User>) -> Result<Json<User>, (StatusCode, String)> {
  user.password = encrypt_password(user.password);

  match UserRepository::create(&pool, user).await {
    Ok(created_user) => Ok(Json(created_user)),
    Err(_) => Err((StatusCode::INTERNAL_SERVER_ERROR, "Erro ao criar usuário".to_string())),
  }
}

fn encrypt_password(password: String) -> String {
  hash(password, DEFAULT_COST).unwrap()
}