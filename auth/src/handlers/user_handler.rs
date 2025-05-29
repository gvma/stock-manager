use axum::{Extension, Json, http::StatusCode};
use lapin::{options::BasicPublishOptions, BasicProperties, Channel};
use sqlx::PgPool;
use bcrypt::{DEFAULT_COST, hash, verify};
use crate::repository::UserRepository;
use crate::domain::{ResetPasswordInput, SignInOutput, SignUpInput, User, UserCreated};

pub async fn sign_in(
  Extension(pool): Extension<PgPool>,
  Json(user): Json<User>
) -> Result<Json<SignInOutput>, (StatusCode, String)> {
  match UserRepository::get_by_email(&pool, user.email).await {
    Ok(found_user) => {
      if verify(user.password, &found_user.password).unwrap() {
        Ok(Json(found_user.into()))
      } else {
        Err((StatusCode::NOT_FOUND, "Usuário não encontrado".to_string()))
      }
    },
    Err(_) => Err((StatusCode::INTERNAL_SERVER_ERROR, "Erro ao buscar usuário".to_string())),
  }
}

pub async fn sign_up(
  Extension(pool): Extension<PgPool>,
  Extension(queue): Extension<Channel>,
  Json(mut user): Json<SignUpInput>
) -> Result<Json<User>, (StatusCode, String)> {
  user.password = encrypt_password(user.password);

  let transaction = pool.begin().await.unwrap();
  match UserRepository::create(&pool, user).await {
    Ok(created_user) => {
      let event = UserCreated { previous_version: None, current_version: created_user.clone(), actor: created_user.email.clone() };
      match queue.basic_publish(
        "",
        "stock-manager-auth",
        BasicPublishOptions::default(),
        &serde_json::to_vec(&event).unwrap(),
        BasicProperties::default()
      ).await {
        Ok(_) => {
          transaction.commit().await.unwrap();
          Ok(Json(created_user))
        },
        Err(_) => {
          transaction.rollback().await.unwrap();
          todo!("cancel transaction and return error, no user should be created")
        }
      }
      
    },
    Err(_) => Err((StatusCode::INTERNAL_SERVER_ERROR, "Erro ao criar usuário".to_string())),
  }
}

pub async fn request_password_reset_code(
  Extension(pool): Extension<PgPool>,
  Json(user): Json<ResetPasswordInput>
) -> Result<(StatusCode, String), (StatusCode, String)> {
  match UserRepository::create_password_reset_code(&pool, user.email).await {
    Ok(user) => {
      println!(
        "Password reset requested for user {}, password reset code is: {}, valid until: {}",
        user.email, user.password_reset_code.unwrap(), user.password_reset_code_expires_at.unwrap()
      );
      Ok((StatusCode::OK, "Processo de resetar senha iniciado".to_string()))
    },
    Err(_) => Err((StatusCode::INTERNAL_SERVER_ERROR, "Erro ao resetar senha".to_string())),
  }
}

pub async fn reset_password(
  Extension(pool): Extension<PgPool>,
  Json(mut user): Json<User>
) -> Result<Json<User>, (StatusCode, String)> {
  user.password = encrypt_password(user.password);

  match UserRepository::reset_password(&pool, user).await {
    Ok(user) => Ok(Json(user)),
    Err(_) => Err((StatusCode::INTERNAL_SERVER_ERROR, "Erro ao resetar senha".to_string())),
  }
}

fn encrypt_password(password: String) -> String {
  hash(password, DEFAULT_COST).unwrap()
}