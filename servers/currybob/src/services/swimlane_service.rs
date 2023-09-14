use crate::domain::dto::swimlane::{SwimlaneUpdateRequest, SwimlaneCreateRequest};
use crate::entity::{swimlane, ticket};
use crate::services::board_service;

use sea_orm::{QueryFilter, ColumnTrait, PaginatorTrait};
use sea_orm::{
    entity::ActiveValue, ActiveModelTrait, DatabaseConnection, DbErr, DeleteResult, EntityTrait, ModelTrait, IntoActiveModel
};


pub async fn find_one(id: i32, conn: &DatabaseConnection) -> Result<Option<swimlane::Model>, DbErr> {
    swimlane::Entity::find_by_id(id).one(conn).await
}

pub async fn find_all(conn: &DatabaseConnection) -> Result<Vec<swimlane::Model>, DbErr> {
    swimlane::Entity::find().all(conn).await
}

pub async fn create(
    new_swimlane: SwimlaneCreateRequest,
    conn: &DatabaseConnection
) -> Result<swimlane::Model, DbErr> {
    let board = board_service::find_one(new_swimlane.board_id, conn).await?;
    if board.is_none() {
        return Err(DbErr::Custom("board가 존재하지 않습니다".into()))
    }

    swimlane::ActiveModel {
        name: ActiveValue::Set(new_swimlane.name),
        board_id: ActiveValue::Set(new_swimlane.board_id),
        description: ActiveValue::Set(new_swimlane.description),
        ..Default::default()
    }
    .insert(conn)
    .await
}

pub async fn update(
    conn: &DatabaseConnection,
    id: i32,
    new_swimlane: SwimlaneUpdateRequest,
) -> Result<Option<swimlane::Model>, DbErr> {
    match find_one(id, conn).await? {
        Some(swimlane) => {
            let mut changes_detected = false;
            let mut active_model = swimlane.into_active_model();

            if let Some(new_name) = new_swimlane.name {
                let same_name_swimlane = swimlane::Entity::find()
                    .filter(swimlane::Column::Name.eq(new_name.to_owned()))
                    .one(conn)
                    .await?;

                if same_name_swimlane.is_some() {
                    return Err(DbErr::Custom("같은 이름의 swimlane이 존재합니다".into()))
                }

                active_model.name = ActiveValue::Set(new_name);
                changes_detected = true;
                
            }

            if let Some(new_board_id) = new_swimlane.board_id {
                let board = board_service::find_one(new_board_id, conn).await?;
                if board.is_none() {
                    return Err(DbErr::Custom("board가 존재하지 않습니다".into()))
                }

                active_model.board_id = ActiveValue::Set(new_board_id);
                changes_detected = true;
            }
            
            if let Some(new_description) = new_swimlane.description {
                active_model.description = ActiveValue::Set(Some(new_description));
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
    let ticket_count = ticket::Entity::find()
        .filter(ticket::Column::SwimlaneId.eq(id))
        .count(conn)
        .await?;

    if ticket_count > 0 {
        return Err(DbErr::Custom("ticket이 있는 swimlane은 삭제할 수 없습니다".into()));
    }

    match swimlane::Entity::find_by_id(id).one(conn).await? {
        Some(entity) => Ok(Some(entity.delete(conn).await?)),
        None => Ok(None),
    }
}