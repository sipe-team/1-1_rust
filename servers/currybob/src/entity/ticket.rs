//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.2


use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "Ticket")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(unique)]
    pub swimlane_id: i32,
    #[sea_orm(column_type = "Text", unique)]
    pub name: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub description: Option<String>,
    #[sea_orm(column_type = "DateTime")]
    pub start_date: Option<i64>,
    #[sea_orm(column_type = "DateTime")]
    pub end_date: Option<i64>,
    pub priority: i32,
}

impl Model {
    pub fn start_date_as_naive_date_time(&self) -> Option<NaiveDateTime> {
        self.start_date.map(|timestamp| NaiveDateTime::from_timestamp(timestamp, 0))
    }

    pub fn end_date_as_naive_date_time(&self) -> Option<NaiveDateTime> {
        self.end_date.map(|timestamp| NaiveDateTime::from_timestamp(timestamp, 0))
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
