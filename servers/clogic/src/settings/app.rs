use crate::settings::db;
use db::get_conn;
use sea_orm::DatabaseConnection;

#[derive(Debug, Clone)]
pub struct AppState {
    pub conn: DatabaseConnection,
}

impl AppState {
    pub async fn new() -> AppState {
        let instance = AppState {
            conn: get_conn().await,
        };
        instance
    }
}
