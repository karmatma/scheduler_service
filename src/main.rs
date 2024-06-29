use actix_web::{App, HttpServer, Responder};
use tracing::info;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber;

#[actix_web::get("/greet")]
async fn greet() -> impl Responder {
    info!("Greet endpoint called, responding with greetings!");
    format!("Hello World!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Hello, world!");

    let log_dir = "logs";
    let file_appender = RollingFileAppender::new(Rotation::DAILY, log_dir, "scheduler_service.log");

    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    tracing_subscriber::fmt()
        .with_file(true)
        .with_writer(non_blocking)
        .init();

    info!("Starting Actix-Web server");

    let port = 8080;

    HttpServer::new(|| App::new().service(greet))
        .bind(("127.0.0.1", port))?
        .run()
        .await
}
