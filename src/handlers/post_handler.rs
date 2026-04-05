use axum::{Json, extract::{Path, State}, http::StatusCode};
use sea_orm::DatabaseConnection;

use crate::{entities::{posts, users}, errors::AppError, services::post_service};
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

pub async fn get_posts(
    State(db): State<DatabaseConnection>
) -> Result<Json<Vec<posts::Model>>, AppError> {
    let posts = post_service::get_all_posts(&db).await?;
    Ok(Json(posts))
}

pub async fn get_post_by_user_id(
    State(db): State<DatabaseConnection>,
    Path(user_id): Path<i32>
) -> Result<Json<Vec<posts::Model>>, AppError> {
    let posts = post_service::get_posts_by_user_id(&db, user_id).await?;
    Ok(Json(posts))
}

pub async fn get_posts_with_authors(
    State(db): State<DatabaseConnection>
) -> Result<Json<Vec<(posts::Model, users::Model)>>, AppError> {
    let result = post_service::get_posts_with_authors(&db).await?;
    Ok(Json(result))
}

pub async fn delete_post(
    State(db): State<DatabaseConnection>,
    Path(id): Path<i32>
) -> Result<StatusCode, AppError> {
    post_service::delete_post_by_id(&db, id).await?;
    Ok(StatusCode::NO_CONTENT)
}