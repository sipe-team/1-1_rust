mod api;
mod models;
mod schemas;
mod settings;

use actix_web::{web, App, HttpServer};
use env_logger::init;
use settings::app::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    init();

    let state = AppState::new().await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .service(api::hello)
            .service(api::board::get_boards)
            .service(api::board::create_boards)
            .service(api::board::update_board)
            .service(api::board::get_board)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
