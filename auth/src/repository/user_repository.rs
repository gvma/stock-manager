use sqlx::PgPool;
use crate::domain::user::User;

pub struct UserRepository;

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
}