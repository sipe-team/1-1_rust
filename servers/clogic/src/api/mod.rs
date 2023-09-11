pub mod board;

use actix_web::{get, HttpResponse, Responder};

#[get("/health")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hi!")
}
