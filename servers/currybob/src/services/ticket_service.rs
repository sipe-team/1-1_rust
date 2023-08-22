use crate::entity::ticket;
use sea_orm::{
    entity::ActiveValue, ActiveModelTrait, DatabaseConnection, DbErr, DeleteResult, EntityTrait, ModelTrait, IntoActiveModel
};

use super::swimlane_service;

pub async fn find_one(id: i32, conn: &DatabaseConnection) -> Result<Option<ticket::Model>, DbErr> {
    ticket::Entity::find_by_id(id).one(conn).await
}

pub async fn find_all(conn: &DatabaseConnection) -> Result<Vec<ticket::Model>, DbErr> {
    ticket::Entity::find().all(conn).await
}

pub async fn create(
    new_ticket: ticket::CreateModel,
    conn: &DatabaseConnection
) -> Result<ticket::Model, DbErr> {
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
    new_ticket: ticket::UpdateModel,
) -> Result<Option<ticket::Model>, DbErr> {
    match find_one(id, conn).await? {
        Some(ticket) => {
            let mut changes_detected = false;
            let mut active_model = ticket.into_active_model();

            if let Some(new_swimlane_id) = new_ticket.swimlane_id {
                if swimlane_service::find_one(new_swimlane_id, conn).await?.is_some() {
                    active_model.swimlane_id = ActiveValue::Set(new_swimlane_id);
                    changes_detected = true;
                }
            }

            if let Some(new_name) = new_ticket.name {
                active_model.name = ActiveValue::Set(new_name);
                changes_detected = true;
            }
            
            if let Some(new_description) = new_ticket.description {
                active_model.description = ActiveValue::Set(Some(new_description));
                changes_detected = true;
            }

            if let Some(new_start_date) = new_ticket.start_date {
                active_model.start_date = ActiveValue::Set(Some(new_start_date));
                changes_detected = true;
            }

            if let Some(new_end_date) = new_ticket.end_date {
                active_model.end_date = ActiveValue::Set(Some(new_end_date));
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