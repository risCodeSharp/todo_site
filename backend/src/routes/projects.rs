use axum::{Json, extract::Path, extract::State, http::StatusCode, response::IntoResponse};
use sqlx::PgPool;
use crate::repo::project_repo::ProjectRespository;
use crate::models::project::{CreateProject, Project, UpdateProject};

pub async fn create_project(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateProject> 
) -> impl IntoResponse {
    match ProjectRespository::create(&pool, payload).await {
        Ok(new_project) => {
            println!("{:?}",new_project);
            (StatusCode::CREATED, Json(new_project)).into_response()
            },
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create new project").into_response()
    }
}   

pub async fn list_projects(
    State(pool): State<PgPool>,
) -> impl IntoResponse {
    match ProjectRespository::list_all(&pool).await {
        Ok(project_list) => {
            println!("{:?}",project_list);
            (StatusCode::OK, Json(project_list)).into_response()
            },
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to get list of projects").into_response()
    }
}

pub async fn get_project_by_id (
    State(pool): State<PgPool>,
    Path(project_id): Path<i32>
) -> impl IntoResponse {
    match ProjectRespository::get_by_id(&pool, project_id).await {
        Ok(project) => {
            (StatusCode::OK, Json(project)).into_response()
            },
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to get project").into_response()
    }
}

pub async fn update_project(
    State(pool): State<PgPool>,
    Path(project_id) : Path<i32>,
    Json(payload): Json<UpdateProject> 
) -> impl IntoResponse {
    match ProjectRespository::update(&pool,payload, project_id).await {
        Ok(updated_project) => {
            (StatusCode::OK, Json(updated_project)).into_response()
        },
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to update project").into_response()
    }
}
pub async fn delete_project(
    State(pool): State<PgPool>,
    Path(project_id) : Path<i32>,
) -> impl IntoResponse {
    match ProjectRespository::delete(&pool,project_id).await {
        Ok(deleted_project) => {
            println!("{:?}",deleted_project);
            (StatusCode::OK, Json(deleted_project)).into_response()
            },
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to delete project").into_response()
    }
}