use crate::models::board::ActiveModel;
use crate::models::prelude::Board;
use crate::schemas::boards::{BoardCreateRequest, BoardUpdateRequest};
use crate::settings::app::AppState;
use actix_web::{get, post, put, web, HttpResponse, Responder};
use log::info;
use sea_orm::{ActiveModelBehavior, ActiveModelTrait, DbErr, EntityTrait, IntoActiveModel};
use serde::Serialize;
use serde_json;
use serde_json::json;

#[get("/api/v1/boards")]
pub async fn get_boards(data: web::Data<AppState>) -> impl Responder {
    match Board::find().into_json().all(&data.conn).await {
        Ok(v) => HttpResponse::Ok().json(v),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[post("/api/v1/boards")]
pub async fn create_boards(
    payload: web::Json<BoardCreateRequest>,
    data: web::Data<AppState>,
) -> impl Responder {
    let active_object = match ActiveModel::from_json(json!(payload)) {
        Ok(v) => v,
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
    };

    match active_object.insert(&data.conn).await {
        Ok(obj) => HttpResponse::Ok().json(obj),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[get("/api/v1/boards/{id}")]
pub async fn get_board(path: web::Path<i32>, data: web::Data<AppState>) -> impl Responder {
    match Board::find_by_id(path.into_inner()).one(&data.conn).await {
        Ok(Some(obj)) => HttpResponse::Ok().json(obj),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
        Ok(None) => HttpResponse::NotFound().body("404"),
    }
}

#[put("/api/v1/boards/{id}")]
pub async fn update_board(
    path: web::Path<i32>,
    payload: web::Json<BoardUpdateRequest>,
    data: web::Data<AppState>,
) -> impl Responder {
    let board_id = path.into_inner();
    let obj = match Board::find_by_id(board_id).one(&data.conn).await {
        Ok(Some(v)) => v,
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
        Ok(None) => return HttpResponse::NotFound().body("404"),
    };

    let mut active_object = obj.into_active_model();
    active_object = match active_object.set_from_json(json!(payload)) {
        Ok(()) => active_object,
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
    };

    match active_object.update(&data.conn).await {
        Ok(v) => HttpResponse::Ok().json(v),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
