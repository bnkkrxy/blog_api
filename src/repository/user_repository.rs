use crate::entities::{posts::ActiveModel, prelude::Users, users};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, Database, DatabaseConnection, DbErr, Insert}; 

pub async fn create(db: &DatabaseConnection, email: String) -> Result<users::Model, DbErr> {
    let new_user = users::ActiveModel {
        email: Set(email),
        ..Default::default()
    };
    new_user.insert(db).await
}

pub fn find_all() {

}

pub fn find_by_id() {

}