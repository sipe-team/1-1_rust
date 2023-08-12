use crate::entity::swimlane;
use sea_orm::{
    entity::ActiveValue, ActiveModelTrait, DatabaseConnection, DbErr, DeleteResult, EntityTrait, ModelTrait, IntoActiveModel
};

pub async fn find_one(id: i32, conn: &DatabaseConnection) -> Result<Option<swimlane::Model>, DbErr> {
    swimlane::Entity::find_by_id(id).one(conn).await
}

pub async fn find_all(conn: &DatabaseConnection) -> Result<Vec<swimlane::Model>, DbErr> {
    swimlane::Entity::find().all(conn).await
}

pub async fn create(new_name: String, conn: &DatabaseConnection) -> Result<swimlane::Model, DbErr> {
    swimlane::ActiveModel {
        name: ActiveValue::Set(new_name),
        ..Default::default()
    }
    .insert(conn)
    .await
}

pub async fn update(
    conn: &DatabaseConnection,
    id: i32,
    new_swimlane: swimlane::UpdateModel,
) -> Result<Option<swimlane::Model>, DbErr> {
    match find_one(id, conn).await? {
        Some(swimlane) => {
            let name_changed = new_swimlane.name != swimlane.name;
            let description_changed = new_swimlane.description != swimlane.description;

            if name_changed || description_changed {
                let mut active_model = swimlane.into_active_model();
                
                if name_changed {
                    active_model.name = ActiveValue::Set(new_swimlane.name.to_owned());
                }

                if description_changed {
                    active_model.description = ActiveValue::Set(new_swimlane.description.to_owned());
                }

                Ok(Some(active_model.update(conn).await?))
            } else {
                Ok(None)
            }
        }
        None => Ok(None),
    }
}

pub async fn delete(id: i32, conn: &DatabaseConnection) -> Result<Option<DeleteResult>, DbErr> {
    match swimlane::Entity::find_by_id(id).one(conn).await? {
        Some(entity) => Ok(Some(entity.delete(conn).await?)),
        None => Ok(None),
    }
}