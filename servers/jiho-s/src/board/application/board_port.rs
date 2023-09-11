use async_trait::async_trait;

use crate::board::application::command::board_command::{CreateBoardCommand, UpdateBoardCommand};
use crate::board::application::error::board_error::BoardPortError;
use crate::board::domain::board::Board;

#[async_trait]
pub trait BoardPort {
    async fn find_by_id(&self, id: i32) -> Result<Board, BoardPortError>;
    async fn find_all(&self) -> Result<Vec<Board>, BoardPortError>;
    async fn update(&self, command: UpdateBoardCommand) -> Result<Board, BoardPortError>;
    async fn create(&self, command: CreateBoardCommand) -> Result<Board, BoardPortError>;
    async fn delete(&self, id: i32) -> Result<(), BoardPortError>;
}