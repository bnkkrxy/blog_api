mod entities;
mod handlers;
mod repository;
mod services;
mod errors;

use axum::{Router, routing::{get, post}};
use sea_orm::Database;
use std::net::SocketAddr;

use crate::handlers::user_handler;

pub struct AppState {
    pub db: sea_orm::DatabaseConnection,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("Set DATABASE_URL");
    let db = Database::connect(db_url).await.expect("Failed to connect to the database");

    let app: Router = Router::new()
        .route("/users", post(user_handler::create_user))
        .route("/users", get(user_handler::get_users))
        .with_state(db);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
