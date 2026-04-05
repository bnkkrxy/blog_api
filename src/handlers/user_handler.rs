use axum::{Json, extract::{Path, State}, http::StatusCode};
use sea_orm::DatabaseConnection;

use crate::{entities::users, errors::AppError, services::user_service};


#[derive(serde::Deserialize)]
pub struct CreateUserRequest {
    pub email: String,
}
#[axum::debug_handler]
pub async fn create_user(
    State(db): State<DatabaseConnection>, 
    Json(payload): Json<CreateUserRequest>
) -> Result<(StatusCode, Json<users::Model>), AppError> {
    let new_user = user_service::create_user(&db, payload.email).await?;
    Ok((StatusCode::CREATED, Json(new_user)))
}

pub async fn get_users(
    State(db): State<DatabaseConnection>,
) -> Result<Json<Vec<users::Model>>, AppError> {
    let users = user_service::get_all_users(&db).await?;
    Ok(Json(users))
}

pub async fn get_user_by_id(
    Path(id): Path<i32>,
    State(db): State<DatabaseConnection>
) -> Result<Json<Option<users::Model>>, AppError> {
    let user = user_service::get_user_by_id(&db, id).await?;
    Ok(Json(user))
}

pub async fn delete_user(
    Path(id): Path<i32>,
    State(db): State<DatabaseConnection>
) -> Result<StatusCode, AppError> {
    user_service::delete_user_by_id(&db, id).await?;
    Ok(StatusCode::NO_CONTENT)
}