use actix_web::{App, HttpServer, Responder, web};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
    })
        .bind("127.0.0.1:10004")?
        .run()
        .await
}

async fn greet() -> impl Responder {
    format!("Hello world!")
}
