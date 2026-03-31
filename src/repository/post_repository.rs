use crate::entities::{posts, users, prelude::{Posts, Users}};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, Database, DatabaseConnection, DbErr, Insert}; 

pub async fn create(db: &DatabaseConnection, title: String, body: String) -> Result<posts::Model, DbErr> {
    let new_post = posts::ActiveModel {
        title: Set(title),
        body: Set(body),
        ..Default::default()
    };
    new_post.insert(db).await
}