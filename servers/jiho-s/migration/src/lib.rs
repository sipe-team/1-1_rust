pub use sea_orm_migration::prelude::*;

mod m20230826_041737_create_board_table;
mod m20230826_042043_create_swimlane_table;
mod m20230826_042052_create_ticket_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230826_041737_create_board_table::Migration),
            Box::new(m20230826_042043_create_swimlane_table::Migration),
            Box::new(m20230826_042052_create_ticket_table::Migration),
        ]
    }
}
