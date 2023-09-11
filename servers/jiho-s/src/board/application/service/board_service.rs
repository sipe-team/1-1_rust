use sea_orm::DatabaseConnection;

use crate::board::application::board_port::BoardPort;
use crate::board::application::command::board_command::{CreateBoardCommand, UpdateBoardCommand};
use crate::board::application::error::board_error::BoardPortError;
use crate::board::domain::board::Board;

pub async fn create(name: String, port: &DatabaseConnection) -> Result<Board, BoardPortError> {
    let command = CreateBoardCommand {
        name,
    };
    port.create(command).await
}

pub async fn update(id: i32, name: String, port: &DatabaseConnection) -> Result<Board, BoardPortError> {
    let command = UpdateBoardCommand {
        id,
        name,
    };
    port.update(command).await
}

pub async fn get(id: i32, port: &DatabaseConnection) -> Result<Board, BoardPortError> {
    port.find_by_id(id).await
}

pub async fn get_all(port: &DatabaseConnection) -> Result<Vec<Board>, BoardPortError> {
    port.find_all().await
}

pub async fn delete(id: i32, port: &DatabaseConnection) -> Result<(), BoardPortError> {
    port.delete(id).await
}