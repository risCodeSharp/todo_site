use axum::{Json, extract::Path, extract::State, http::StatusCode, response::IntoResponse};
use sqlx::PgPool;

use crate::{models::project::{CreateProject, UpdateProject}, repositories::project_repo::ProjectRepository};

pub async fn create_project(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateProject>,
) -> impl IntoResponse {
    match ProjectRepository::create_project(&pool, payload).await {
        Ok(project) => (StatusCode::OK, Json(project)).into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to create new project",
        )
            .into_response(),
    }
}

pub async fn update_project(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateProject>
) -> impl IntoResponse {
    match ProjectRepository::update_project(&pool, id, payload).await {
        Ok(updated_project) => (StatusCode::OK, Json(updated_project)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to update project").into_response()
    }
}

pub async fn get_projects(State(pool): State<PgPool>) -> impl IntoResponse {
    match ProjectRepository::get_projects(&pool).await {
        Ok(projects) => (StatusCode::OK, Json(projects)).into_response(),
        Err(_e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to fetch all projects",
        )
            .into_response(),
    }
}

pub async fn get_project_by_id(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    match ProjectRepository::get_project_by_id(&pool, id).await {
        Ok(projects) => (StatusCode::OK, Json(projects)).into_response(),
        Err(_e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to fetch project on that id",
        )
            .into_response(),
    }
}

pub async fn delete_project(State(pool): State<PgPool>, Path(id): Path<i32>) -> impl IntoResponse {
    match ProjectRepository::delete_project(&pool, id).await {
        Ok(project) => (StatusCode::OK, Json(project)).into_response(),
        Err(_e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to delete the project",
        )
            .into_response(),
    }
}
