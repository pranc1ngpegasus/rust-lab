mod graphql;
use axum::{extract::Extension, routing::get, Router};
use graphql::Query;
use rusty_gql::*;
use rusty_gql_axum::*;
use std::{net::SocketAddr, path::Path};
type ContainerType = Container<Query, EmptyMutation, EmptySubscription>;

async fn gql_handler(
    container: axum::extract::Extension<ContainerType>,
    req: GqlRequest,
) -> GqlResponse {
    let result = execute(&container, req.0).await;
    GqlResponse::from(result)
}

async fn gql_playground() -> impl axum::response::IntoResponse {
    axum::response::Html(playground_html("/", None))
}

#[tokio::main]
async fn main() {
    let schema_docs = read_schemas(Path::new("./schema")).unwrap();
    let schema_docs: Vec<&str> = schema_docs.iter().map(|s| &**s).collect();
    let container = Container::new(
        schema_docs.as_slice(),
        Query,
        EmptyMutation,
        EmptySubscription,
        Default::default(),
    )
    .unwrap();
    let app = Router::new()
        .route("/graphiql", get(gql_playground))
        .route("/", get(gql_handler).post(gql_handler))
        .layer(Extension(container));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
