use axum::{
    Json, Router,
    extract::{Path, State},
    routing::{delete, get, post},
};

use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::Mutex;
use uuid::Uuid;

#[derive(Serialize, Clone)]
struct Todo {
    task: String,
    name: String,
    id: String,
}
#[derive(Clone)]
struct AppState {
    todos: Arc<Mutex<Vec<Todo>>>,
}

#[derive(Deserialize)]
struct CreateTodo {
    name:String,
    task: String,
}

#[tokio::main]
async fn main() {
    let state = AppState {
        todos: Arc::new(Mutex::new(Vec::new())),
    };

    let app = Router::new()
        .route("/", get(get_todos))
        .route("/create", post(crate_todo))
        .route("/todos/:id", delete(delete_todo))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Todo API running at http://{}", addr);

    let listner = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listner, app).await.unwrap();
}

async fn get_todos(State(state): State<AppState>) -> Json<Vec<Todo>> {
    let todos = state.todos.lock().await;
    Json(todos.clone())
}

async fn crate_todo(
    State(state): State<AppState>,
    Json(payload): Json<CreateTodo>,
) -> Json<Todo> {
    let mut todos = state.todos.lock().await;
    let todo = Todo {
        id: Uuid::new_v4().to_string(),
        task: payload.task,
        name: payload.name,
    };
    todos.push(todo.clone());
    Json(todo)
}


async fn delete_todo(State(state): State<AppState>, Path(id): Path<String>) -> Json<bool> {
    let mut todos = state.todos.lock().await;
    let len_before = todos.len();
    todos.retain(|t| t.id != id);
    Json(todos.len() < len_before)
}