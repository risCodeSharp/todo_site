use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use sqlx::PgPool;

use crate::{
    models::job::{CreateJob, Job, UpdateJob},
    repo::job_repo::JobRepository,
};

pub async fn create_job(
    State(pool): State<PgPool>,
    Path((_, task_id)): Path<(i32, i32)>,
    Json(payload): Json<CreateJob>,
) -> impl IntoResponse {
    match JobRepository::create(&pool, payload, task_id).await {
        Ok(new_job) => (StatusCode::CREATED, Json(new_job)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to create job for a task",
        )
            .into_response(),
    }
}

pub async fn list_jobs(State(pool): State<PgPool>, Path((_, task_id)): Path<(i32, i32)>,
) -> impl IntoResponse {
    match JobRepository::list_all(&pool, task_id).await {
        Ok(job_list) => (StatusCode::CREATED, Json(job_list)).into_response(),
        Err(_e) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch jobs").into_response(),
    }
}

pub async fn update_job(State(pool): State<PgPool>, Path(job_id): Path<i32>, Json(payload): Json<UpdateJob>) -> impl IntoResponse {
    match JobRepository::update(&pool, payload, job_id).await {
        Ok(updated_job) => (StatusCode::OK, Json(updated_job)).into_response(),
        Err(_e) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to update jobs").into_response(),
    }
}
pub async fn delete_job(State(pool): State<PgPool>, Path(job_id): Path<i32>) -> impl IntoResponse {
    match JobRepository::delete(&pool, job_id).await {
        Ok(updated_job) => (StatusCode::OK, Json(updated_job)).into_response(),
        Err(_e) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to update jobs").into_response(),
    }
}