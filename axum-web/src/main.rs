mod record;

use axum::{extract::Path, http::StatusCode, response::Json, routing::get, Extension, Router};
use record::Conn;
use serde_json::{json, Value};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let conn = Conn::new().await;

    let app = Router::new()
        .route("/ping", get(ping))
        .route("/users", get(users))
        .route("/users/:id", get(user_by_id))
        .layer(Extension(Arc::new(conn)));

    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn ping() -> Result<String, StatusCode> {
    Ok("pong".to_string())
}

async fn users(Extension(conn): Extension<Arc<Conn>>) -> Result<Json<Value>, StatusCode> {
    let user = conn.users().await;
    match user {
        Ok(u) => Ok(Json(json!(u))),
        Err(e) => {
            println!("failed to get user: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn user_by_id(
    Path(id): Path<i32>,
    Extension(conn): Extension<Arc<Conn>>,
) -> Result<Json<Value>, StatusCode> {
    let user = conn.user_by_id(id).await;
    match user {
        Ok(u) => Ok(Json(json!(u))),
        Err(e) => {
            println!("failed to get user by id: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
