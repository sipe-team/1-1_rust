use crate::entity::{prelude::*, swimlane};
use sea_orm::{DatabaseConnection, EntityTrait};

#[derive(Debug, Clone)]
pub struct SwimlaneRepository {
    pub db_conn: DatabaseConnection,
}

impl SwimlaneRepository {
    pub async fn get_swimlanes(&self) -> Vec<swimlane::Model> {
        Swimlane::find()
            .all(&self.db_conn)
            .await
            .expect("Error while fetching all swimlane")
    }
}