use crate::repository::user_repository;
use crate::errors::AppError;
use crate::entities::users;
use sea_orm::DatabaseConnection;

pub async fn create_user(db: &DatabaseConnection, email: String) -> Result<users::Model, AppError> {
    if !email.contains("@") {
        return Err(AppError::InvalidData("Invalid email format".into()))
    }
    user_repository::create_user(db, email)
        .await
        .map_err(|e| AppError::InternalServer(e.to_string()))
}

pub async fn get_all_users(db: &DatabaseConnection) -> Result<Vec<users::Model>, AppError> {
    user_repository::find_all_users(db)
        .await
        .map_err(|e| AppError::InternalServer(e.to_string()))
}

pub async fn get_all_users_by_id(db: &DatabaseConnection, id: i32) -> Result<Option<users::Model>, AppError> {
    user_repository::find_user_by_id(db, id)
        .await
        .map_err(|e| AppError::InternalServer(e.to_string()))
}