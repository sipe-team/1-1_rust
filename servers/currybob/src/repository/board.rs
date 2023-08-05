use crate::entity::{prelude::*, board};
use sea_orm::{DatabaseConnection, EntityTrait};

#[derive(Debug, Clone)]
pub struct BoardRepository {
    pub db_conn: DatabaseConnection,
}

impl BoardRepository {
    pub async fn get_boards(&self) -> Vec<board::Model> {
        Board::find()
            .all(&self.db_conn)
            .await
            .expect("Error while fetching all board")
    }
}