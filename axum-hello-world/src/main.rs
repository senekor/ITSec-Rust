#![allow(unused)]

use axum::{extract::Path, response::Html, routing::get, Json, Router};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[tokio::main]
async fn main() {
    let hello_world_router = Router::new()
        .route("/", get(hello_world_handler))
        .route("/:my_param", get(complicated_handler));

    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(hello_world_router.into_make_service())
        .await
        .unwrap();
}

async fn hello_world_handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

#[derive(Deserialize)]
struct IncomingData {
    some_int: i32,
    some_string: String,
}

#[derive(Serialize)]
struct OutgoingData {
    whatever: Vec<u8>,
}

/// Router::new().route("/:my_param", get(complicated_handler))
///
async fn complicated_handler(
    Path(my_param): Path<u32>,
    Json(incoming_data): Json<IncomingData>,
) -> Json<OutgoingData> {
    Json(OutgoingData {
        whatever: vec![1, 2, 3],
    })
}
