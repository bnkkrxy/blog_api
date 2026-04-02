use axum::{Json, extract::State, http::StatusCode};
use sea_orm::DatabaseConnection;

use crate::{entities::posts, errors::AppError, services::post_service};
#[derive(serde::Deserialize)]
pub struct CreatePostRequest {
    pub title: String,
    pub body: String,
    pub user_id: i32,
}

#[axum::debug_handler]
pub async fn create_post(
    State(db): State<DatabaseConnection>,
    Json(payload): Json<CreatePostRequest>
) -> Result<(StatusCode, Json<posts::Model>), AppError> {
    let new_post = post_service::create_post(&db, payload.title, payload.body, payload.user_id).await?;
    Ok((StatusCode::CREATED, Json(new_post)))
}