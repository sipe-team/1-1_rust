use actix_web::{App, HttpServer, Responder, web};
use sea_orm::{Database, DatabaseConnection, EntityTrait};
use tracing_subscriber::fmt::format;

use migration::{Migrator, MigratorTrait};
use crate::entities::prelude::Post;

mod entities;

const DATABASE_URL: &str = "postgres://postgres:password@localhost:5432/todo";

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let db = Database::connect(DATABASE_URL).await.unwrap();
    Migrator::up(&db, None).await.unwrap();


    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .route("/", web::get().to(greet))
            .route("/test", web::get().to(test))
    })
        .bind("127.0.0.1:10004")?
        .run()
        .await
}

async fn greet() -> impl Responder {
    format!("Hello world!")
}

async fn test(db: web::Data<DatabaseConnection>) -> impl Responder {
    let post = Post::find_by_id(1).one(db.get_ref()).await.unwrap().unwrap();
    return format!("{} {} {}", post.id, post.title, post.text);
}
