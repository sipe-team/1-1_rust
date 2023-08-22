use crate::entity::{board, swimlane};
use sea_orm::{
    entity::ActiveValue, ActiveModelTrait, DatabaseConnection, DbErr, DeleteResult, EntityTrait, ModelTrait, IntoActiveModel, QueryFilter, ColumnTrait, PaginatorTrait
};

pub async fn find_one(id: i32, conn: &DatabaseConnection) -> Result<Option<board::Model>, DbErr> {
    board::Entity::find_by_id(id).one(conn).await
}

pub async fn find_all(conn: &DatabaseConnection) -> Result<Vec<board::Model>, DbErr> {
    board::Entity::find().all(conn).await
}

pub async fn create(
    new_board: board::CreateModel,
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
    new_board: board::UpdateModel,
) -> Result<Option<board::Model>, DbErr> {
    match find_one(id, conn).await? {
        Some(board) => {
            if new_board.name != board.name {
                let mut active_model = board.into_active_model();
                active_model.name = ActiveValue::Set(new_board.name.to_owned());
                Ok(Some(active_model.update(conn).await?))
            } else {
                Ok(None)
            }
        }
        None => Ok(None),
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