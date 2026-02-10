use crate::models::task::{CreateTask, ProjectWithTags, Task, UpdateTask};
use crate::repositories::task_repo::TaskRepository;
use axum::extract::Path;
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use sqlx::PgPool;

pub async fn create_task(
    State(pool): State<PgPool>,
    Path(project_id): Path<i32>,
    Json(payload): Json<CreateTask>,
) -> impl IntoResponse {
    match TaskRepository::create_task(&pool, payload, project_id).await {
        Ok(task) => (StatusCode::CREATED, Json(task)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create task").into_response(),
    }
}

pub async fn get_tasks_by_project_id(
    State(pool): State<PgPool>,
    Path(project_id): Path<i32>
) -> impl IntoResponse {
    match TaskRepository::get_tasks_by_project_id(&pool, project_id).await {
        Ok(project_tasks) => (StatusCode::OK, Json(project_tasks)).into_response(),
        Err(_e) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to get project tasks").into_response()
    }
}  

pub async fn update_task(
    State(pool): State<PgPool>,
    Path((project_id,task_id)): Path<(i32,i32)>,
    Json(payload): Json<UpdateTask>
)-> impl IntoResponse {
    match TaskRepository::update_task(&pool, payload, project_id, task_id).await {
         Ok(task) => (StatusCode::OK, Json(task)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to update task").into_response(),
    }
}

pub async fn delete_task(
    State(pool): State<PgPool>,
    Path((project_id, task_id)) : Path<(i32,i32)>
) -> impl IntoResponse {
    match TaskRepository::delete_task(&pool, project_id, task_id).await {
         Ok(task) => (StatusCode::OK, Json(task)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to delete task").into_response(),
    }
}
