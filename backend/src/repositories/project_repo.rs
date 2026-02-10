use crate::models::project::{CreateProject, Project, UpdateProject};
use sqlx::PgPool;
use sqlx::error::Error;
pub struct ProjectRepository;

impl ProjectRepository {
    pub async fn create_project(pool: &PgPool, payload: CreateProject) -> Result<Project, Error> {
        sqlx::query_as::<_, Project>(
            "INSERT INTO projects(name, tags) VALUES ($1, $2) RETURNING id, name, tags",
        )
        .bind(payload.name)
        .bind(payload.tags)
        .fetch_one(pool)
        .await
    }
    
    pub async fn get_projects(pool: &PgPool) -> Result<Vec<Project>, Error> {
        sqlx::query_as::<_, Project>("SELECT * FROM projects")
        .fetch_all(pool)
        .await
    }

    pub async fn get_project_by_id(pool: &PgPool, id: i32) -> Result<Project, Error> {
        sqlx::query_as::<_, Project>("SELECT * FROM projects WHERE id = $1")
        .bind(id)
        .fetch_one(pool)
        .await
    }

    pub async fn delete_project(pool: &PgPool, id: i32) -> Result<Project, Error> {
        sqlx::query_as::<_,Project>(
            "DELETE FROM projects WHERE id = $1 RETURNING *"
        ).bind(id)
        .fetch_one(pool)
        .await
    }

    pub async fn update_project(pool: &PgPool, id: i32, payload: UpdateProject) -> Result<Project, Error> {
        sqlx::query_as::<_,Project>(
            "UPDATE projects SET name = $1, tags=$2 WHERE id =$3 RETURNING id, name, tags"
        )
        .bind(payload.name)
        .bind(payload.tags)
        .bind(id)
        .fetch_one(pool)
        .await
    }
}
