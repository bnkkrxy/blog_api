use crate::repository::{post_repository, user_repository};
use crate::errors::AppError;
use crate::entities::{posts, users};
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

pub async fn get_all_posts(db: &DatabaseConnection) -> Result<Vec<posts::Model>, AppError> {
    post_repository::find_all_posts(db)
        .await
        .map_err(|e| AppError::InternalServer(e.to_string()))
}

pub async fn get_posts_by_user_id(db: &DatabaseConnection, user_id: i32) -> Result<Vec<posts::Model>, AppError> {
    post_repository::find_posts_by_user_id(db, user_id)
        .await
        .map_err(|e| AppError::InternalServer(e.to_string()))
}

pub async fn get_posts_with_authors(db: &DatabaseConnection) -> Result<Vec<(posts::Model, users::Model)>, AppError> {
    post_repository::find_posts_with_authors(db)
        .await
        .map_err(|e| AppError::InternalServer(e.to_string()))
}