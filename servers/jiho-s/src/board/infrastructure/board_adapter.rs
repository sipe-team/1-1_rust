use anyhow::Context;
use async_trait::async_trait;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, IntoActiveModel, ModelTrait, TryIntoModel};
use sea_orm::ActiveValue::Set;

use crate::board::application::board_port::BoardPort;
use crate::board::application::command::board_command::{CreateBoardCommand, UpdateBoardCommand};
use crate::board::application::error::board_error::BoardPortError;
use crate::board::domain::board::Board;
use crate::entities::board::ActiveModel;
use crate::entities::prelude::Board as BoardPersistenceModel;

#[async_trait]
impl BoardPort for DatabaseConnection {
    async fn find_by_id(&self, id: i32) -> Result<Board, BoardPortError> {
        let model = BoardPersistenceModel::find_by_id(id).one(self)
            .await
            .context("Failed to get Board find by id.")?
            .ok_or(BoardPortError::NotFound(id))?;
        Ok(model.into())
    }

    async fn find_all(&self) -> Result<Vec<Board>, BoardPortError> {
        let models = BoardPersistenceModel::find().all(self)
            .await
            .context("Failed to get Board find.")?;
        let result = models.iter()
            .map(|model| model.into())
            .collect();
        Ok(result)
    }

    async fn update(&self, command: UpdateBoardCommand) -> Result<Board, BoardPortError> {
        let id = command.id;
        let model = BoardPersistenceModel::find_by_id(id).one(self)
            .await
            .context("Failed to get Board find by id.")?
            .ok_or(BoardPortError::NotFound(id))?;
        let mut active_model = model.into_active_model();
        active_model.name = Set(command.name);
        let board = active_model.update(self).await
            .context("Failed to update Board")?
            .try_into_model()
            .context("Failed to convert Model")?
            .into();
        Ok(board)
    }

    async fn create(&self, command: CreateBoardCommand) -> Result<Board, BoardPortError> {
        let model: ActiveModel = command.into();
        let board = model.save(self)
            .await
            .context("Failed to create Board")?
            .try_into_model()
            .context("Failed to convert Model")?
            .into();
        Ok(board)
    }

    async fn delete(&self, id: i32) -> Result<(), BoardPortError> {
        let model = BoardPersistenceModel::find_by_id(id).one(self)
            .await
            .context("Failed to get Board find by id.")?
            .ok_or(BoardPortError::NotFound(id))?;
        model.delete(self)
            .await
            .context("Failed to delete Board.")?;
        Ok(())
    }
}