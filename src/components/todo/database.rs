use serenity::all::UserId;
use surrealdb::engine::local::Db;
use surrealdb::Surreal;
use super::constants::COMPONENT_ID;

pub async fn migrate(db: &Surreal<Db>) -> Result<(), surrealdb::Error> {
    db.query(
        "

        ",
    ).await?;
    Ok(())
}

pub fn add_todo(user: UserId, content: String, db: &Surreal<Db>) {
    let uid = user.get();
}