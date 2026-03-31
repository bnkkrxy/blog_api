mod entities;
mod handlers;
mod repository;
mod services;

use sea_orm::{Database, DatabaseConnection};

pub struct AppState {
    pub db: sea_orm::DatabaseConnection,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("Set DATABASE_URL");
    let db = Database::connect(db_url).await.expect("Failed to connect to the database");

    let test_email = "test_user2@mail.com".to_string();
    match repository::user_repository::create_user(&db, test_email).await {
        Ok(user) => println!("Successfully created user: {:?}", user),
        Err(e) => println!("Error creating user: {:?}", e),
    }

    match repository::user_repository::find_all_users(&db).await {
        Ok(users) => println!("Current users in DB: {:?}", users),
        Err(e) => println!("Error fetching users: {:?}", e),
    }

    match repository::user_repository::find_user_by_id(&db, 2).await {
        Ok(user) => println!("Current user in DB: {:?}", user),
        Err(e) => println!("Error fetching user: {:?}", e),
    }

    let test_post1_title = "title text".to_string();
    let test_post1_body = "body text".to_string();
    match repository::post_repository::create_post(&db, test_post1_title, test_post1_body, 2).await {
        Ok(post) => println!("Successfully created post: {:?}", post),
        Err(e) => println!("Error creating post: {:?}", e),
    }

    match repository::post_repository::find_all_posts(&db).await {
        Ok(posts) => println!("Current posts in DB: {:?}", posts),
        Err(e) => println!("Error fetching posts: {:?}", e),
    }

    match repository::post_repository::find_posts_by_user_id(&db, 3).await {
        Ok(post) => println!("Current post in DB: {:?}", post),
        Err(e) => println!("Error fetching post: {:?}", e),
    }

    match repository::post_repository::find_posts_with_authors(&db).await {
        Ok(posts) => println!("Current posts in DB: {:?}", posts),
        Err(e) => println!("Error fetching posts: {:?}", e),
    }

}
