use crate::models::prelude::Board;
use crate::settings::app::AppState;
use actix_web::{get, web, Responder};
use sea_orm::EntityTrait;

#[get("/api/v1/boards")]
pub async fn get_boards(data: web::Data<AppState>) -> impl Responder {
    let _conn = &data.conn;
    let x = Board::find().into_json().all(_conn).await.unwrap();
    web::Json(x)
}
