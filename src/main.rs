mod entities;
mod handlers;
mod repository;
mod services;
mod errors;

use axum::{Router, routing::{get, post}};
use sea_orm::Database;
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;

use crate::handlers::{post_handler, user_handler};

pub struct AppState {
    pub db: sea_orm::DatabaseConnection,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter("blog_api=debug,tower_http=debug")
        .init();

    dotenvy::dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("Set DATABASE_URL");
    let db = Database::connect(db_url).await.expect("Failed to connect to the database");

    let app: Router = Router::new()
        .route("/users", post(user_handler::create_user))
        .route("/users", get(user_handler::get_users))
        .route("/users/{id}/users", get(user_handler::get_user_by_id))
        .route("/posts", post(post_handler::create_post))
        .layer(TraceLayer::new_for_http())
        .with_state(db);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
