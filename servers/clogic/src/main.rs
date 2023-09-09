mod settings;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use settings::app::AppState;

#[get("/")]
async fn hello(data: web::Data<AppState>) -> impl Responder {
    let _conn = &data.conn;
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello).app_data(AppState::new()))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
