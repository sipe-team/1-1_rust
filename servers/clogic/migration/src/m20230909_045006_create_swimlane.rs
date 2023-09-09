use crate::m20220101_000001_create_table::Board;
use crate::m20230909_045006_create_swimlane::Swimlane::BoardId;
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
                    .col(ColumnDef::new(Swimlane::Name).string().not_null())
                    .col(ColumnDef::new(Swimlane::BoardId).integer().not_null())
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("fk-swimlane-board")
                            .from_tbl(Swimlane::Table)
                            .from_col(Swimlane::BoardId)
                            .to_tbl(Board::Table)
                            .to_col(Board::Id),
                    )
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
pub enum Swimlane {
    Table,
    Id,
    Name,
    BoardId,
}
