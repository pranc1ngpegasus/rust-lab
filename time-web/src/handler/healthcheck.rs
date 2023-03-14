use actix_web::{get, HttpResponse, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct HealthcheckResponse {
    message: String,
}

#[get("/healthcheck")]
async fn healthcheck() -> impl Responder {
    HttpResponse::Ok().json(HealthcheckResponse {
        message: "ok".to_string(),
    })
}
