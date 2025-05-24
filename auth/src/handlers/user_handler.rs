use axum::{Extension, Json, http::StatusCode};
use sqlx::PgPool;
use bcrypt::{DEFAULT_COST, hash, verify};
use crate::repository::user_repository::UserRepository;
use crate::domain::user::{User, SignInInput, SignInOutput};

pub async fn sign_in(Extension(pool): Extension<PgPool>, Json(user): Json<SignInInput>) -> Result<Json<SignInOutput>, (StatusCode, String)> {
  match UserRepository::get_by_email(&pool, user.email).await {
    Ok(found_user) => {
      if verify(user.password, &found_user.password).unwrap() {
        Ok(Json(found_user.to_sign_in_output()))
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