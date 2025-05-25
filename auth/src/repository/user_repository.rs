use chrono::{Duration, TimeDelta, Utc};
use rand::{rng, Rng, distr::Alphanumeric};
use sqlx::PgPool;
use crate::domain::User;

pub struct UserRepository;

const PASSWORD_RESET_CODE_VALIDITY: TimeDelta = Duration::hours(24);

impl UserRepository {
    pub async fn get_by_email(pool: &PgPool, email: String) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as!(User, "SELECT * FROM users WHERE email = $1", email)
            .fetch_one(pool)
            .await?;
        Ok(user)
    }

    pub async fn create(pool: &PgPool, user: User) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            "INSERT INTO users (email, name, password) VALUES ($1, $2, $3) RETURNING *",
            user.email, user.name, user.password
        )
        .fetch_one(pool)
        .await?;

        Ok(user)
    }

    pub async fn create_password_reset_code(pool: &PgPool, email: String) -> Result<User, sqlx::Error> {
        let code: String = rng().sample_iter(&Alphanumeric).take(30).map(char::from).collect();

        let user = sqlx::query_as!(
            User,
            "UPDATE users SET password_reset_code = $2, password_reset_code_expires_at = $3 WHERE email = $1 RETURNING *",
            email, code, (Utc::now() + PASSWORD_RESET_CODE_VALIDITY).naive_utc()
        )
        .fetch_one(pool)
        .await?;

        Ok(user)
    }

    pub async fn reset_password(pool: &PgPool, user: User) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            "
                UPDATE users
                SET password = $3, password_reset_code = NULL, password_reset_code_expires_at = NULL
                WHERE email = $1 AND password_reset_code = $2 AND password_reset_code_expires_at > NOW()
                RETURNING *
            ",
            user.email, user.password_reset_code, user.password
        )
        .fetch_one(pool)
        .await?;

        Ok(user)
    }
}