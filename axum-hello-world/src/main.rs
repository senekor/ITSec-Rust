use axum::{response::Html, routing::get, Router};

#[tokio::main]
async fn main() {
    let hello_world_router = Router::new().route("/", get(hello_world_handler));

    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(hello_world_router.into_make_service())
        .await
        .unwrap();
}

async fn hello_world_handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
