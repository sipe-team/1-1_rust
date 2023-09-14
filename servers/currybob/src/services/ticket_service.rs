use crate::entity::ticket;
use crate::domain::dto::ticket::{TicketUpdateRequest, TicketCreateRequest};

use chrono::NaiveDateTime;
use sea_orm::{
    entity::ActiveValue, ActiveModelTrait, DatabaseConnection, DbErr, DeleteResult, EntityTrait, ModelTrait, IntoActiveModel, ColumnTrait, QueryFilter
};

use super::swimlane_service;

pub async fn find_one(id: i32, conn: &DatabaseConnection) -> Result<Option<ticket::Model>, DbErr> {
    ticket::Entity::find_by_id(id).one(conn).await
}

pub async fn find_all(conn: &DatabaseConnection) -> Result<Vec<ticket::Model>, DbErr> {
    ticket::Entity::find().all(conn).await
}

pub async fn create(
    new_ticket: TicketCreateRequest,
    conn: &DatabaseConnection
) -> Result<ticket::Model, DbErr> {
    let swimlane = swimlane_service::find_one(new_ticket.swimlane_id, conn).await?;
    if swimlane.is_none() {
        return Err(DbErr::Custom("swimlane이 존재하지 않습니다".into()))
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
    new_ticket: TicketUpdateRequest,
) -> Result<Option<ticket::Model>, DbErr> {
    match find_one(id, conn).await? {
        Some(ticket) => {
            let mut changes_detected = false;
            let mut active_model = ticket.into_active_model();

            if let Some(new_swimlane_id) = new_ticket.swimlane_id {
                let swimlane = swimlane_service::find_one(new_swimlane_id, conn).await?;
                if swimlane.is_none() {
                    return Err(DbErr::Custom("swimlane이 존재하지 않습니다".into()))
                }

                active_model.swimlane_id = ActiveValue::Set(new_swimlane_id);
                changes_detected = true;
            }

            if let Some(new_name) = new_ticket.name {
                let same_name_ticket = ticket::Entity::find()
                    .filter(ticket::Column::Name.eq(new_name.to_owned()))
                    .one(conn)
                    .await?;

                if same_name_ticket.is_some() {
                    return Err(DbErr::Custom("같은 이름의 ticket이 존재합니다".into()))
                }

                active_model.name = ActiveValue::Set(new_name);
                changes_detected = true;
            }
            
            if let Some(new_description) = new_ticket.description {
                active_model.description = ActiveValue::Set(Some(new_description));
                changes_detected = true;
            }

            if let Some(new_start_date) = new_ticket.start_date {            
                let start_date = NaiveDateTime::from_timestamp_opt(new_start_date, 0);

                active_model.start_date = ActiveValue::Set(start_date);
                changes_detected = true;
            }

            if let Some(new_end_date) = new_ticket.end_date {
                let end_date = NaiveDateTime::from_timestamp_opt(new_end_date, 0);

                active_model.end_date = ActiveValue::Set(end_date);
                changes_detected = true;
            }

            if let Some(new_priority) = new_ticket.priority {
                active_model.priority = ActiveValue::Set(new_priority);
                changes_detected = true;
            }

            if changes_detected {
                Ok(Some(active_model.update(conn).await?))
            } else {
                Ok(None)
            }
        }
        None => Ok(None),
    }
}

pub async fn delete(id: i32, conn: &DatabaseConnection) -> Result<Option<DeleteResult>, DbErr> {
    match ticket::Entity::find_by_id(id).one(conn).await? {
        Some(entity) => Ok(Some(entity.delete(conn).await?)),
        None => Ok(None),
    }
}