use crate::{domain::Event, repository::EventRepository};
use axum::{http::StatusCode, Extension, Json};
use mongodb::Database;
use futures_util::TryStreamExt;

pub async fn list(Extension(pool): Extension<Database>) -> Result<Json<Vec<Event>>, (StatusCode, String)> {
    match EventRepository::list(&pool).await {
        Ok(events) => {
            Ok(Json(events.try_collect().await.unwrap()))
        },
        Err(_) => Err((StatusCode::INTERNAL_SERVER_ERROR, "Erro ao buscar usuário".to_string())),
    }
}
