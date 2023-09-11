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
                    .col(ColumnDef::new(Ticket::Name).char_len(128).not_null())
                    .col(ColumnDef::new(Ticket::Description).char_len(512))
                    .col(ColumnDef::new(Ticket::StartDate).date_time())
                    .col(ColumnDef::new(Ticket::EndDate).date_time())
                    .col(ColumnDef::new(Ticket::Priority).char_len(52).not_null())
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
    StartDate,
    EndDate,
    Priority
}
