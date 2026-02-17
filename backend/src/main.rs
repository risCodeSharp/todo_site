use std::{env, net::SocketAddr};

use axum::{Router, extract::State, routing::{delete, get, post, put}};
use sqlx::{postgres::PgPoolOptions};
use tokio::net::TcpListener;

use tower_http::cors::{CorsLayer, Any};
use axum::http::{HeaderValue, Method};

mod models;
mod types;
mod routes;
mod repo;

#[tokio::main]
async fn main() {
    // Load .env
    dotenvy::dotenv().ok();

    // Cors config
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(Any);

    // Read Database_url from .env
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // create connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool");

    let app = Router::new()
        .route("/", get(handler))
        .route("/projects", post(routes::projects::create_project))
        .route("/projects", get(routes::projects::list_projects))
        .route("/projects/{project_id}", get(routes::projects::get_project_by_id))
        .route("/projects/{project_id}", put(routes::projects::update_project))
        .route("/projects/{project_id}", delete(routes::projects::delete_project))

        .route("/projects/{project_id}/tasks", post(routes::tasks::create_task))
        .route("/projects/{project_id}/tasks", get(routes::tasks::list_tasks))
        .route("/tasks/{task_id}", put(routes::tasks::update_task))
        .route("/tasks/{task_id}", delete(routes::tasks::delete_task))
        
        .route("/projects/{project_id}/tasks/{task_id}/jobs", post(routes::jobs::create_job))
        .route("/projects/{project_id}/tasks/{task_id}/jobs", get(routes::jobs::list_jobs))
        .route("/jobs/{job_id}", put(routes::jobs::update_job))
        .route("/jobs/{job_id}", delete(routes::jobs::delete_job))
        
        .with_state(pool)
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 6767));
    println!("Server running on http://{}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handler(State(pool): State<sqlx::PgPool>) -> String {
    let row: (i32,) = sqlx::query_as("SELECT 1").fetch_one(&pool).await.unwrap();

    format!("Database response: {}", row.0)
}
