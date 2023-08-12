use crate::entity::board;
use crate::services::board_service;
use crate::AppState;

use actix_web::{delete, get, post, put, web, HttpResponse, Responder, Scope};

#[get("/")]
async fn find_all_boards(state: web::Data<AppState>) -> impl Responder {
    match board_service::find_all(&state.db_conn).await {
        Ok(boards) => HttpResponse::Ok().json(boards),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/{board_id}")]
async fn find_one_board(
    state: web::Data<AppState>,
    board_id: web::Path<String>,
) -> impl Responder {
    match board_id.parse::<i32>() {
        Ok(board_id) => match board_service::find_one(board_id, &state.db_conn).await {
            Ok(board_option) => match board_option {
                Some(board) => HttpResponse::Ok().json(board),
                None => HttpResponse::NotFound().body("Board not found"),
            },
            Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
        },
        Err(err) => HttpResponse::NotFound().body(err.to_string()),
    }
}

#[post("/{new_name}")]
async fn add_board(state: web::Data<AppState>, new_name: web::Path<String>) -> impl Responder {
    match board_service::create(new_name.to_string(), &state.db_conn).await {
        Ok(board) => HttpResponse::Ok().json(board),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[put("/{board_id}")]
async fn update_board(
    state: web::Data<AppState>,
    board_id: web::Path<String>,
    new_board: web::Json<board::UpdateModel>,
) -> impl Responder {
    match board_id.parse::<i32>() {
        Ok(board_id) => {
            match board_service::update(&state.db_conn, board_id, new_board.into_inner())
                .await
            {
                Ok(board_option) => match board_option {
                    Some(board) => HttpResponse::Ok().json(board),
                    None => HttpResponse::NotFound().body("Board not found"),
                },
                Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
            }
        }
        Err(err) => HttpResponse::NotFound().body(err.to_string()),
    }
}

#[delete("/{board_id}")]
async fn delete_board(state: web::Data<AppState>, board_id: web::Path<String>) -> impl Responder {
    match board_id.parse::<i32>() {
        Ok(board_id) => match board_service::delete(board_id, &state.db_conn).await {
            Ok(delete_result) => match delete_result {
                Some(result) => {
                    HttpResponse::Ok().body(format!("{} row deleted", result.rows_affected))
                }
                None => HttpResponse::Ok().body("No board deleted"),
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