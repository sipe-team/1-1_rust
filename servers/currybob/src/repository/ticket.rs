use crate::entity::{prelude::*, ticket};
use sea_orm::{DatabaseConnection, EntityTrait};

#[derive(Debug, Clone)]
pub struct TicketRepository {
    pub db_conn: DatabaseConnection,
}

impl TicketRepository {
    pub async fn get_tickets(&self) -> Vec<ticket::Model> {
        Ticket::find()
            .all(&self.db_conn)
            .await
            .expect("Error while fetching all ticket")
    }
}