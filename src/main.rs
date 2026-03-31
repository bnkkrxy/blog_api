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

}
