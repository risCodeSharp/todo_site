use std::task;

use crate::models::task::{CreateTask, UpdateTask};
use crate::repo::task_repo::TaskRespository;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::{Json, response::IntoResponse};
use sqlx::PgPool;

pub async fn create_task(
    State(pool): State<PgPool>,
    Path(project_id): Path<i32>,
    Json(payload): Json<CreateTask>,
) -> impl IntoResponse {
    match TaskRespository::create(&pool, payload, project_id).await {
        Ok(new_task) => (StatusCode::CREATED, Json(new_task)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to create new task",
        )
            .into_response(),
    }
}

pub async fn list_tasks(
    State(pool): State<PgPool>,
    Path(project_id): Path<i32>,
) -> impl IntoResponse {
    match TaskRespository::list_all(&pool, project_id).await {
        Ok(task_list) => (StatusCode::OK, Json(task_list)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to delete task",
        )
            .into_response(),
    }
}

pub async fn update_task(
    State(pool): State<PgPool>,
    Path(task_id): Path<i32>,
    Json(payload): Json<UpdateTask>,
) -> impl IntoResponse {
    match TaskRespository::update(&pool, payload, task_id).await {
        Ok(update_task) => (StatusCode::OK, Json(update_task)).into_response(),
        Err(_) => (StatusCode::NOT_MODIFIED, "Task is not updated!").into_response(),
    }
}

pub async fn delete_task(
    State(pool): State<PgPool>,
    Path(task_id): Path<i32>,
) -> impl IntoResponse {
    match TaskRespository::delete(&pool, task_id).await {
        Ok(delete_task) => (StatusCode::OK, Json(delete_task)).into_response(),
        Err(_) =>(StatusCode::BAD_REQUEST, "Failed delete to task!").into_response()
    }
}