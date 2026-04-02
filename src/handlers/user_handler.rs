use axum::{Json, extract::State, http::StatusCode};
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