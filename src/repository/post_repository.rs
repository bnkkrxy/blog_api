use crate::entities::{posts, users, prelude::{Posts, Users}};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection, DbErr, EntityTrait}; 
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

pub async fn find_posts_with_authors(db: &DatabaseConnection) -> Result<Vec<(posts::Model, users::Model)>, DbErr> {
    let result = Posts::find().find_with_related(Users).all(db).await?;
    let transformed_result = result
        .into_iter()
        .filter_map(|(post, mut authors)| { authors.pop().map(|author| (post, author))})
        .collect();
    Ok(transformed_result)
}