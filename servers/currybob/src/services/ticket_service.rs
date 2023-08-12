use crate::entity::ticket;
use sea_orm::{
    entity::ActiveValue, ActiveModelTrait, DatabaseConnection, DbErr, DeleteResult, EntityTrait, ModelTrait, IntoActiveModel
};

pub async fn find_one(id: i32, conn: &DatabaseConnection) -> Result<Option<ticket::Model>, DbErr> {
    ticket::Entity::find_by_id(id).one(conn).await
}

pub async fn find_all(conn: &DatabaseConnection) -> Result<Vec<ticket::Model>, DbErr> {
    ticket::Entity::find().all(conn).await
}

pub async fn create(new_name: String, conn: &DatabaseConnection) -> Result<ticket::Model, DbErr> {
    ticket::ActiveModel {
        name: ActiveValue::Set(new_name),
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
            let name_changed = new_ticket.name != ticket.name;
            let description_changed = new_ticket.description != ticket.description;
            let start_date_changed = new_ticket.start_date != ticket.start_date;
            let end_date_changed = new_ticket.end_date != ticket.end_date;
            let priority_changed = new_ticket.priority != ticket.priority;

            if
                name_changed ||
                description_changed ||
                start_date_changed ||
                end_date_changed ||
                priority_changed 
            {
                let mut active_model = ticket.into_active_model();
                
                if name_changed {
                    active_model.name = ActiveValue::Set(new_ticket.name.to_owned());
                }

                if description_changed {
                    active_model.description = ActiveValue::Set(new_ticket.description.to_owned());
                }

                if start_date_changed {
                    active_model.start_date = ActiveValue::Set(new_ticket.start_date.to_owned());
                }

                if end_date_changed {
                    active_model.end_date = ActiveValue::Set(new_ticket.end_date.to_owned());
                }

                if priority_changed {
                    active_model.priority = ActiveValue::Set(new_ticket.priority.to_owned());
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
    match ticket::Entity::find_by_id(id).one(conn).await? {
        Some(entity) => Ok(Some(entity.delete(conn).await?)),
        None => Ok(None),
    }
}