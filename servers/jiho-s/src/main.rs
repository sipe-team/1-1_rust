use std::net::TcpListener;

use sea_orm::Database;

use jiho_web::configuration::get_configuration;
use jiho_web::startup::run;
use jiho_web::telemetry::{get_subscriber, init_subscriber};
use migration::{Migrator, MigratorTrait};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("jiho-todo".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");

    let db = Database::connect(configuration.database.option()).await
        .expect("Failed to connect database.");
    Migrator::up(&db, None).await.unwrap();

    let listener = TcpListener::bind(configuration.application.address())?;

    run(listener, db)?.await
}