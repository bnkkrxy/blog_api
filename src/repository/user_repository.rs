use crate::entities::{prelude::Users, users};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection, DbErr, EntityTrait};
use chrono::Utc; 

pub async fn create_user(db: &DatabaseConnection, email: String) -> Result<users::Model, DbErr> {
    let new_user = users::ActiveModel {
        email: Set(email),
        created_at: Set(Utc::now().naive_utc()),
        ..Default::default()
    };
    new_user.insert(db).await
}

pub async fn find_all_users(db: &DatabaseConnection) -> Result<Vec<users::Model>, DbErr> {
    Users::find().all(db).await
}

pub async fn find_user_by_id(db: &DatabaseConnection, id: i32) -> Result<Option<users::Model>, DbErr> {
    Users::find_by_id(id).one(db).await
}

pub async fn delete_user_by_id(db: &DatabaseConnection, user_id: i32) -> Result<sea_orm::DeleteResult, DbErr> {
    Users::delete_by_id(user_id).exec(db).await
}