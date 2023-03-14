mod handler;

use crate::handler::healthcheck::healthcheck;
use crate::handler::time::{days_before, now};
use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(healthcheck)
            .service(now)
            .service(days_before)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
