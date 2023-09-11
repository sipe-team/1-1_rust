use actix_web::{delete, get, post, put, Responder, Scope, web};
use actix_web::web::Json;
use sea_orm::DatabaseConnection;

use crate::board::application::error::board_error::BoardPortError;
use crate::board::application::service::board_service::{create, delete, get, get_all, update};
use crate::board::routes::board_dto::{BoardResponseDto, CreateBoardRequestDto, UpdateBoardRequestDto};

#[post("")]
async fn add_board(
    json: web::Json<CreateBoardRequestDto>,
    connection: web::Data<DatabaseConnection>,
) -> Result<impl Responder, BoardPortError> {
    let new_board_name = json.into_inner().name;
    create(new_board_name, connection.get_ref())
        .await
        .map(BoardResponseDto::from)
        .map(Json)
}

#[delete("/{board_id}")]
async fn delete_board(
    path: web::Path<i32>,
    connection: web::Data<DatabaseConnection>,
) -> Result<impl Responder, BoardPortError> {
    let board_id = path.into_inner();
    delete(board_id, connection.get_ref())
        .await
        .map(Json)
}

#[get("/{board_id}")]
pub async fn get_board(
    path: web::Path<i32>,
    connection: web::Data<DatabaseConnection>,
) -> Result<impl Responder, BoardPortError> {
    let board_id = path.into_inner();
    get(board_id, connection.get_ref())
        .await
        .map(BoardResponseDto::from)
        .map(Json)
}

#[get("")]
pub async fn get_boards(
    connection: web::Data<DatabaseConnection>,
) -> Result<impl Responder, BoardPortError> {
    let result: Vec<BoardResponseDto> = get_all(connection.get_ref())
        .await?
        .iter()
        .map(BoardResponseDto::from)
        .collect();

    Ok(Json(result))
}

#[put("/{id}")]
pub async fn put_boards(
    path: web::Path<i32>,
    json: web::Json<UpdateBoardRequestDto>,
    connection: web::Data<DatabaseConnection>,
) -> Result<impl Responder, BoardPortError> {
    let board_id = path.into_inner();
    let new_board_name = json.into_inner().name;
    update(board_id, new_board_name, connection.get_ref())
        .await
        .map(BoardResponseDto::from)
        .map(Json)
}

pub fn board_scope() -> Scope {
    web::scope("/api/boards")
        .service(add_board)
        .service(delete_board)
        .service(get_board)
        .service(get_boards)
        .service(put_boards)
}