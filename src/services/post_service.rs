use crate::repository::{self, post_repository, user_repository};
use crate::errors::AppError;
use crate::entities::posts;
use sea_orm::DatabaseConnection;

pub async fn create_post(db: &DatabaseConnection, title: String, body: String, user_id: i32) -> Result<posts::Model, AppError> {
    if title.is_empty() {
        return Err(AppError::InvalidData("Title is empty".to_string()));
    }

    let user = user_repository::find_user_by_id(db, user_id)
        .await
        .map_err(|e| AppError::InternalServer(e.to_string()))?;
    if user.is_none() {
        return Err(AppError::NotFound(format!("User with ID {} not found", user_id)));
    }

    post_repository::create_post(db, title, body, user_id)
        .await
        .map_err(|e| AppError::InternalServer(e.to_string()))
}

