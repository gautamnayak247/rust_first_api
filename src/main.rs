#![cfg_attr(target_arch = "wasm32", allow(dead_code))]

#[cfg(not(target_arch = "wasm32"))]
use axum::{
    Json, Router,
    http::StatusCode,
    routing::{get, post, put},
};
#[cfg(not(target_arch = "wasm32"))]
use std::net::SocketAddr;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Employee {
    pub id: u32,
    pub name: String,
    pub role: String,
}

#[cfg(not(target_arch = "wasm32"))]
async fn health() -> &'static str {
    "healthy!"
}

#[cfg(not(target_arch = "wasm32"))]
async fn create_employee(Json(payload): Json<Employee>) -> (StatusCode, Json<Employee>) {
    println!("Creating Employee {:?}", payload);
    (StatusCode::CREATED, Json(payload))
}

#[cfg(not(target_arch = "wasm32"))]
async fn update_employee(Json(payload): Json<Employee>) -> StatusCode {
    println!("Updating Employee {:?}", payload);
    StatusCode::NO_CONTENT
}

#[cfg(not(target_arch = "wasm32"))]
async fn get_employee() -> (StatusCode, Json<Employee>) {
    println!("Getting employees");
    (
        StatusCode::OK,
        Json(Employee {
            id: 1,
            name: "Gautam".to_string(),
            role: "Permanent".to_string(),
        }),
    )
}

#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(health))
        .route("/employees", post(create_employee))
        .route("/employees", put(update_employee))
        .route("/employees", get(get_employee));

    let address = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", address);

    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[cfg(target_arch = "wasm32")]
fn main() {
    panic!("Cannot run binary on WASM target. Use wasm-pack build for WASM or cargo run for server.");
}
