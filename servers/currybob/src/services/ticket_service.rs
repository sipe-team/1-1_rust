use crate::entity::ticket;
use crate::domain::dto::ticket::{TicketUpdateRequest, TicketCreateRequest, TicketSortQuery};

use chrono::NaiveDateTime;
use sea_orm::QueryOrder;
use sea_orm::{
    entity::ActiveValue, ActiveModelTrait, DatabaseConnection, DbErr, DeleteResult, EntityTrait, ModelTrait, IntoActiveModel, ColumnTrait, QueryFilter, Select
};

use super::swimlane_service;

fn convert_order(order: Option<&String>) -> Option<sea_orm::Order> {
    match order {
        Some(s) if s.to_lowercase() == "asc" => Some(sea_orm::Order::Asc),
        Some(s) if s.to_lowercase() == "desc" => Some(sea_orm::Order::Desc),
        _ => None
    }
}

fn build_select_query(query: &TicketSortQuery) -> Select<ticket::Entity> {
    let mut select = ticket::Entity::find();

    if let Some(order_by) = convert_order(query.name.as_ref()) {
        select = select.order_by(ticket::Column::Name, order_by);
    }

    if let Some(order_by) = convert_order(query.start_date.as_ref()) {
        select = select.order_by(ticket::Column::StartDate, order_by);
    }

    if let Some(order_by) = convert_order(query.end_date.as_ref()) {
        select = select.order_by(ticket::Column::EndDate, order_by);
    }

    if let Some(order_by) = convert_order(query.priority.as_ref()) {
        select = select.order_by(ticket::Column::Priority, order_by);
    }

    select
}

pub async fn find_all(
    conn: &DatabaseConnection,
    query: &TicketSortQuery
) -> Result<Vec<ticket::Model>, DbErr> {
    build_select_query(query).all(conn).await
}

pub async fn find_all_by_swimlane_id(
    swimlane_id: i32,
    conn: &DatabaseConnection,
    query: &TicketSortQuery
) -> Result<Vec<ticket::Model>, DbErr> {
    build_select_query(query)
        .filter(ticket::Column::SwimlaneId.eq(swimlane_id))
        .all(conn)
        .await
}

pub async fn find_one(id: i32, conn: &DatabaseConnection) -> Result<Option<ticket::Model>, DbErr> {
    ticket::Entity::find_by_id(id).one(conn).await
}

pub async fn create(
    new_ticket: TicketCreateRequest,
    conn: &DatabaseConnection
) -> Result<ticket::Model, DbErr> {
    let swimlane = swimlane_service::find_one(new_ticket.swimlane_id, conn).await?;
    if swimlane.is_none() {
        let message = "swimlane이 존재하지 않습니다".to_owned();
        return Err(DbErr::Custom(message))
    }

    ticket::ActiveModel {
        swimlane_id: ActiveValue::Set(new_ticket.swimlane_id),
        name: ActiveValue::Set(new_ticket.name),
        priority: ActiveValue::Set(new_ticket.priority),
        ..Default::default()
    }
    .insert(conn)
    .await
}

pub async fn update(
    conn: &DatabaseConnection,
    id: i32,
    update_data: TicketUpdateRequest,
) -> Result<ticket::Model, DbErr> {
    let exist = find_one(id, conn).await;

    match exist {
        Ok(Some(ticket)) => {
            let mut active_model = ticket.into_active_model();

            if let Some(new_swimlane_id) = update_data.swimlane_id {
                let swimlane = swimlane_service::find_one(new_swimlane_id, conn).await?;
                if swimlane.is_none() {
                    let message = "swimlane이 존재하지 않습니다".to_owned();
                    return Err(DbErr::Custom(message))
                }

                active_model.swimlane_id = ActiveValue::Set(new_swimlane_id);
            }

            if let Some(new_name) = update_data.name {
                let same_name_ticket = ticket::Entity::find()
                    .filter(ticket::Column::Name.eq(new_name.to_owned()))
                    .one(conn)
                    .await?;

                if same_name_ticket.is_some() {
                    let message = "같은 이름의 ticket이 존재합니다".to_owned();
                    return Err(DbErr::Custom(message))
                }

                active_model.name = ActiveValue::Set(new_name);
            }
            
            if let Some(new_description) = update_data.description {
                active_model.description = ActiveValue::Set(Some(new_description));
            }

            if let Some(new_start_date) = update_data.start_date {            
                let start_date = NaiveDateTime::from_timestamp_opt(new_start_date, 0);

                active_model.start_date = ActiveValue::Set(start_date);
            }

            if let Some(new_end_date) = update_data.end_date {
                let end_date = NaiveDateTime::from_timestamp_opt(new_end_date, 0);

                active_model.end_date = ActiveValue::Set(end_date);
            }

            if let Some(new_priority) = update_data.priority {
                active_model.priority = ActiveValue::Set(new_priority);
            }

            active_model.update(conn).await
        }
        Ok(None) => {
            let message = "해당 레코드가 없습니다".to_owned();
            return Err(DbErr::RecordNotFound(message))
        },
        Err(e) => return Err(e),
    }
}

pub async fn delete(id: i32, conn: &DatabaseConnection) -> Result<Option<DeleteResult>, DbErr> {
    match ticket::Entity::find_by_id(id).one(conn).await? {
        Some(entity) => Ok(Some(entity.delete(conn).await?)),
        None => Ok(None),
    }
}