use crate::entity::{board, swimlane};
use crate::domain::dto::board::{BoardCreateRequest, BoardUpdateRequest};

use sea_orm::{
    entity::ActiveValue, ActiveModelTrait, DatabaseConnection, DbErr, DeleteResult, EntityTrait, ModelTrait, IntoActiveModel, QueryFilter, ColumnTrait, PaginatorTrait
};
use serde_json::json;

pub async fn find_one(id: i32, conn: &DatabaseConnection) -> Result<Option<board::Model>, DbErr> {
    board::Entity::find_by_id(id).one(conn).await
}

pub async fn find_all(conn: &DatabaseConnection) -> Result<Vec<board::Model>, DbErr> {
    board::Entity::find().all(conn).await
}

pub async fn create(
    new_board: BoardCreateRequest,
    conn: &DatabaseConnection
) -> Result<board::Model, DbErr> {
    board::ActiveModel {
        name: ActiveValue::Set(new_board.name),
        ..Default::default()
    }
    .insert(conn)
    .await
}

pub async fn update(
    conn: &DatabaseConnection,
    id: i32,
    update_data: BoardUpdateRequest,
) -> Result<board::Model, DbErr> {
    let exist = find_one(id, conn).await;

    match exist {
        Ok(Some(board)) => {
            let mut active_model = board.into_active_model();
            active_model = match active_model.set_from_json(json!(update_data)) {
                Ok(()) => active_model,
                Err(e) => return Err(e),
            };

            active_model.update(conn).await
        },
        Ok(None) => {
            let message = "해당 레코드가 없습니다".to_owned();
            return Err(DbErr::RecordNotFound(message))
        },
        Err(e) => return Err(e),
    }
}

pub async fn delete(id: i32, conn: &DatabaseConnection) -> Result<Option<DeleteResult>, DbErr> {
    let swimlane_count = swimlane::Entity::find()
        .filter(swimlane::Column::BoardId.eq(id))
        .count(conn)
        .await?;

    if swimlane_count > 0 {
        return Err(DbErr::Custom("swimlane이 있는 board는 삭제할 수 없습니다".into()));
    }

    match board::Entity::find_by_id(id).one(conn).await? {
        Some(entity) => Ok(Some(entity.delete(conn).await?)),
        None => Ok(None),
    }
}