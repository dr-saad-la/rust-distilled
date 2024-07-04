use actix_web::{App, HttpServer};
use actix_web::middleware::Logger;
use crate::routes::calculate::calculate;

mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init(); // Initialize the logger

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default()) // Add logger middleware
            .service(calculate)      // Register the calculate route
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
