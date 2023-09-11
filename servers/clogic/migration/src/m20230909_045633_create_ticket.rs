use crate::m20230909_045006_create_swimlane::Swimlane;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Ticket::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Ticket::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Ticket::SwimlaneId).integer().not_null())
                    .col(ColumnDef::new(Ticket::Name).string().not_null())
                    .col(ColumnDef::new(Ticket::Description).string().not_null())
                    .col(ColumnDef::new(Ticket::StartAt).date_time())
                    .col(ColumnDef::new(Ticket::EndAt).date_time())
                    .col(
                        ColumnDef::new(Ticket::Priority)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("fk-ticket-swimlane")
                            .from_tbl(Ticket::Table)
                            .from_col(Ticket::SwimlaneId)
                            .to_tbl(Swimlane::Table)
                            .to_col(Swimlane::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Ticket::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Ticket {
    Table,
    Id,
    SwimlaneId,
    Name,
    Description,
    StartAt,
    EndAt,
    Priority,
}
