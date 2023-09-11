use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Swimlane::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Swimlane::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Swimlane::Name).char_len(128).not_null())
                    .col(ColumnDef::new(Swimlane::Description).char_len(512))
                    .col(ColumnDef::new(Swimlane::BoardId).integer().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Swimlane::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Swimlane {
    Table,
    Id,
    Name,
    Description,
    BoardId,
}
