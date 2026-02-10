mod models;
mod repositories;
mod routes;


use axum::{
    Router, routing::{delete, get, post, put}
};
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;

use tower_http::cors::{CorsLayer, Any};
use axum::http::{HeaderValue, Method};

#[tokio::main]
async fn main() {
    // Load Environmental variable
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Cors config
    let cors = CorsLayer::new() 
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::PUT, Method::POST, Method::DELETE])
        .allow_headers(Any);

    // Connect to the database
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to connect to DB");

    // Set up Router
    let app = Router::new()
    .route("/projects", post(routes::projects::create_project))
    .route("/projects", get(routes::projects::get_projects))
    .route("/projects/{project_id}", get(routes::projects::get_project_by_id))
    .route("/projects/{project_id}", put(routes::projects::update_project))
    .route("/projects/{project_id}", delete(routes::projects::delete_project))

    .route("/projects/{project_id}", post(routes::tasks::create_task))
    .route("/projects/{project_id}/tasks", get(routes::tasks::get_tasks_by_project_id))
    .route(
        "/projects/{project_id}/tasks/{task_id}",
        put(routes::tasks::update_task),
    )
    .route(
        "/projects/{project_id}/tasks/{task_id}",
        delete(routes::tasks::delete_task),
    )
    .with_state(pool)
    .layer(cors);

    // Start Server
    println!("Server running on http://localhost:6767");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:6767").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
