use axum::{
    Json, Router,
    http::StatusCode,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[derive(Serialize, Deserialize, Debug)]
struct Employee {
    id: u32,
    name: String,
    role: String,
}

async fn create_employee(Json(payload): Json<Employee>) -> (StatusCode, Json<Employee>) {
    println!("Creating Employee {:?}", payload);
    (StatusCode::CREATED, Json(payload))
}

async fn update_employee(Json(payload): Json<Employee>) -> StatusCode {
    println!("Updating Employee {:?}", payload);
    StatusCode::NO_CONTENT
}

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

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(hc)).route(
        "/employees",
        post(create_employee).put(update_employee).get(get_employee),
    );

    let address = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    println!("Server running on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn hc() -> &'static str {
    "healthy!"
}
