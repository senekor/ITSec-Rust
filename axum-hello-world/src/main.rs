#![allow(unused)]

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Html,
    routing::{get, post},
    Json, Router,
};
use persistence::SqliteInMemory;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::Arc;

mod persistence;

#[derive(Deserialize)]
struct IncomingData {
    some_int: i32,
    some_string: String,
}

#[derive(Serialize)]
struct OutgoingData {
    whatever: Vec<u8>,
}

struct AppState {
    db: SqliteInMemory,
}

#[tokio::main]
async fn main() {
    let db = SqliteInMemory::connect()
        .await
        .expect("failed to connect to in-memory sqlite");
    db.migrate().await.expect("failed to run db migrations");

    let state = Arc::new(AppState { db });

    let hello_world_router = Router::new()
        .route("/", get(hello_world_handler))
        .route("/:my_param", get(complicated_handler))
        .route("/text/:text", post(text_enter_handler))
        .route("/texts", get(texts_handler));

    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(hello_world_router.with_state(state).into_make_service())
        .await
        .expect("failed to bind to 127.0.0.1:3000");
}

async fn hello_world_handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
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

async fn text_enter_handler(
    State(state): State<Arc<AppState>>,
    Path(text): Path<String>,
) -> Result<StatusCode, StatusCode> {
    state.db.enter_text(&text).await.map_err(|e| {
        println!("{:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(StatusCode::CREATED)
}

async fn texts_handler(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<String>>, StatusCode> {
    let texts = state.db.all_texts().await.map_err(|e| {
        println!("{:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(texts))
}
