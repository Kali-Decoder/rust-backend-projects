use axum::{extract::{Path, Query}, routing::get, Json, Router};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[derive(Serialize)]
struct Message {
    response: String,
}

#[derive(Deserialize)]
struct AgeParams {
    age : Option<u64>
}

#[tokio::main]
async fn main() {
    // define a route: GET / -> hello_world function
    let app = Router::new()
        .route("/", get(hello_world))
        .route("/greet/:name", get(name_greeting))
        .route("/age", get(get_age));  
    // address to run on
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running at http://{}", addr);
    // create listener
    let listener = TcpListener::bind(addr).await.unwrap();
    // start the server
    axum::serve(listener, app).await.unwrap();
}

// handler function
async fn hello_world() -> Json<Message> {
    let message = Message {
        response: "Hello, World!".to_string(),
    };
    Json(message)
}

async fn name_greeting(Path(name): Path<String>) -> Json<Message> {
    let message = Message {
        response: format!("Hello World {}", name),
    };
    Json(message)
}

async fn get_age(Query(params) : Query<AgeParams>) -> Json<Message> {
    let age = params.age.unwrap_or(32);
    Json(Message { response: format!("My age is {}",age) })
}
