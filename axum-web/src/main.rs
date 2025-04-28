use axum::{
    Router,
    response::Html,
    routing::{get, post},
};
use serde::Serialize;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(say_hello_text))
        .route("/json", get(say_hello_json))
        .route("/post", post(hello_post))
        .route("/hell", get(say_hell));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

#[derive(Serialize)]
struct HelloJson {
    message: String,
}

async fn say_hello_text() -> Html<&'static str> {
    Html("<h1>Hello, world!</h1>")
}

async fn say_hell() -> Html<String> {
    let path = std::path::Path::new("hello.html");
    let content = tokio::fs::read_to_string(path).await.unwrap();
    Html(content)
}

async fn say_hello_json() -> axum::Json<HelloJson> {
    let message: HelloJson = HelloJson {
        message: "Hi from JSON".to_string(),
    };
    axum::Json(message)
}

async fn hello_post() -> Html<String> {
    Html("Hello from Post".to_string())
}
