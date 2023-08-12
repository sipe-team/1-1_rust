use crate::entity::board;
use sea_orm::{
    entity::ActiveValue, ActiveModelTrait, DatabaseConnection, DbErr, DeleteResult, EntityTrait, ModelTrait, IntoActiveModel
};

pub async fn find_one(id: i32, conn: &DatabaseConnection) -> Result<Option<board::Model>, DbErr> {
    board::Entity::find_by_id(id).one(conn).await
}

pub async fn find_all(conn: &DatabaseConnection) -> Result<Vec<board::Model>, DbErr> {
    board::Entity::find().all(conn).await
}

pub async fn create(new_name: String, conn: &DatabaseConnection) -> Result<board::Model, DbErr> {
    board::ActiveModel {
        name: ActiveValue::Set(new_name),
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
    match board::Entity::find_by_id(id).one(conn).await? {
        Some(entity) => Ok(Some(entity.delete(conn).await?)),
        None => Ok(None),
    }
}