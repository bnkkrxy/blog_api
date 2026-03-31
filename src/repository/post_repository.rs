use crate::entities::{posts, users, prelude::{Posts, Users}};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, Database, DatabaseConnection, DbErr, EntityTrait, Insert, Related}; 
use chrono::Utc;

pub async fn create_post(db: &DatabaseConnection, title: String, body: String, user_id: i32) -> Result<posts::Model, DbErr> {
    let new_post = posts::ActiveModel {
        title: Set(title),
        body: Set(body),
        user_id: Set(user_id),
        created_at: Set(Utc::now().naive_utc()),
        ..Default::default()
    };
    new_post.insert(db).await
}

pub async fn find_all_posts(db: &DatabaseConnection) -> Result<Vec<posts::Model>, DbErr> {
    Posts::find().all(db).await
}

pub async fn find_posts_by_user_id(db: &DatabaseConnection, user_id: i32) -> Result<Vec<posts::Model>, DbErr> {
    Posts::find_by_id(user_id).all(db).await
}

pub async fn find_posts_with_authors(db: &DatabaseConnection) -> Result<Vec<(posts::Model, Option<users::Model>)>, DbErr> {
    Posts::find().find_also_related(Users).all(db).await
}