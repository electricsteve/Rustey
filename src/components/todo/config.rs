use super::constants::COMPONENT_ID;
use crate::core::database::get_component_config;
use surrealdb::Surreal;
use surrealdb::engine::local::Db;
use surrealdb::types::SurrealValue;

#[derive(SurrealValue)]
struct TodoConfig {
    include_uwu: bool, // Testing with this bc I can't think of an actual config option
}

pub async fn config(db: &Surreal<Db>) -> Result<TodoConfig, crate::Error> {
    get_component_config(COMPONENT_ID.to_string(), db).await
}
