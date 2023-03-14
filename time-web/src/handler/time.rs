use actix_web::{get, web, HttpResponse, Responder};
use chrono::{Duration, Utc};
use serde::Serialize;

const FORMAT: &str = "%Y/%m/%d %H:%M:%S";

#[derive(Serialize)]
struct DaysBeforeResponse {
    calculated: String,
}

#[get("/days/before/{num}")]
async fn days_before(path: web::Path<(i64,)>) -> impl Responder {
    let calculated = Utc::now() - Duration::days(path.into_inner().0);
    let formatted = format!("{}", calculated.format(FORMAT));

    HttpResponse::Ok().json(DaysBeforeResponse {
        calculated: formatted,
    })
}

#[derive(Serialize)]
struct NowResponse {
    now: String,
}

#[get("/now")]
async fn now() -> impl Responder {
    let formatted = format!("{}", Utc::now().format(FORMAT));

    HttpResponse::Ok().json(NowResponse { now: formatted })
}
