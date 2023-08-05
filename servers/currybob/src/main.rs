use crate::repository::prelude::{BoardRepository, SwimlaneRepository, TicketRepository};
use actix_web::{middleware, web::{self, Data}, App, HttpServer};
use api::prelude::{tickets_api, boards_api, swimlanes_api};

mod entity;
mod api;
mod repository;

#[derive(Debug, Clone)]
pub struct AppState {
    board_repository: BoardRepository,
    swimlane_repository: SwimlaneRepository,
    ticket_repository: TicketRepository,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    setup_environment();

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let db_conn = sea_orm::Database::connect(&db_url).await.unwrap();

    let board_repository = BoardRepository { db_conn: db_conn.clone() };
    let swimlane_repository = SwimlaneRepository { db_conn: db_conn.clone() };
    let ticket_repository = TicketRepository { db_conn: db_conn.clone() };


    let state = AppState { 
        board_repository,
        swimlane_repository,
        ticket_repository
    };
    
    let server_url = get_server_url();

    let server = HttpServer::new(move || {
        App::new()
            .app_data(Data::new(state.clone()))
            .wrap(middleware::Logger::default())
            .configure(init)
    })
    .bind(&server_url)?;

    println!("Starting server at {}", server_url);
    server.run().await?;
    Ok(())
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(boards_api());
    cfg.service(swimlanes_api());
    cfg.service(tickets_api());
}

fn setup_environment() {
    std::env::set_var("RUST_LOG", "debug");
    dotenv::dotenv().ok();
}

fn get_server_url() -> String {
    let host = std::env::var("HOST").expect("HOST is not set in .env file");
    let port = std::env::var("PORT").expect("PORT is not set in .env file");
    format!("{}:{}", host, port)
}
