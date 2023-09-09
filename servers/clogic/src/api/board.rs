use crate::models::prelude::Board;
use crate::settings::app::AppState;
use actix_web::{get, web, HttpResponse, Responder};
use sea_orm::EntityTrait;

#[get("/api/v1/boards")]
pub async fn get_boards(data: web::Data<AppState>) -> impl Responder {
    match Board::find().into_json().all(&data.conn).await {
        Ok(v) => HttpResponse::Ok().json(v),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
