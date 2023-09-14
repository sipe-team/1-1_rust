use crate::domain::dto::board::{BoardCreateRequest, BoardUpdateRequest};
use crate::services::board_service;
use crate::AppState;

use actix_web::{delete, get, post, put, web, HttpResponse, Responder, Scope};

#[get("")]
async fn find_all_boards(state: web::Data<AppState>) -> impl Responder {
    match board_service::find_all(&state.db_conn).await {
        Ok(boards) => HttpResponse::Ok().json(boards),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/{board_id}")]
async fn find_one_board(
    state: web::Data<AppState>,
    board_id: web::Path<i32>,
) -> impl Responder {
    let board_id = board_id.into_inner();

    match board_service::find_one(board_id, &state.db_conn).await {
        Ok(board_option) => match board_option {
            Some(board) => HttpResponse::Ok().json(board),
            None => HttpResponse::NotFound().body("해당 board를 찾을 수 없습니다"),
        },
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[post("")]
async fn add_board(
    state: web::Data<AppState>,
    data: web::Json<BoardCreateRequest>
) -> impl Responder {
    match board_service::create(data.into_inner(), &state.db_conn).await {
        Ok(board) => HttpResponse::Ok().json(board),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[put("/{board_id}")]
async fn update_board(
    state: web::Data<AppState>,
    board_id: web::Path<i32>,
    payload: web::Json<BoardUpdateRequest>,
) -> impl Responder {
    let board_id = board_id.into_inner();
    let payload = payload.into_inner();

    match board_service::update(&state.db_conn, board_id, payload).await {
        Ok(board) => HttpResponse::Ok().json(board),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[delete("/{board_id}")]
async fn delete_board(state: web::Data<AppState>, board_id: web::Path<String>) -> impl Responder {
    match board_id.parse::<i32>() {
        Ok(board_id) => match board_service::delete(board_id, &state.db_conn).await {
            Ok(delete_result) => match delete_result {
                Some(result) => {
                    HttpResponse::Ok().body(format!("{}개의 row가 삭제되었습니다", result.rows_affected))
                }
                None => HttpResponse::Ok().body("board가 삭제되지 않았습니다"),
            },
            Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
        },
        Err(err) => HttpResponse::NotFound().body(err.to_string()),
    }
}

pub fn boards_api() -> Scope {
    web::scope("/boards")
        .service(find_all_boards)
        .service(find_one_board)
        .service(add_board)
        .service(update_board)
        .service(delete_board)
}