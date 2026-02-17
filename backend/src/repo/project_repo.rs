use sqlx::PgPool;
use crate::models::project::{CreateProject, Project, UpdateProject};

pub struct ProjectRespository;

impl ProjectRespository {
    pub async fn create(pool: &PgPool, payload: CreateProject) -> Result<Project, sqlx::Error> {
        sqlx::query_as::<_, Project>(
            "INSERT INTO projects(name, description) VALUES ($1, $2) RETURNING id, name, description"
        )
        .bind(payload.name)
        .bind(payload.description)
        .fetch_one(pool)
        .await
    }

    
    pub async fn get_by_id(pool: &PgPool, project_id: i32) -> Result<Project,sqlx::Error> {
        sqlx::query_as::<_,Project>(
            "SELECT * FROM projects WHERE id = $1"
        )
        .bind(project_id)
        .fetch_one(pool)
        .await
    }

    pub async fn list_all(pool: &PgPool) -> Result<Vec<Project>, sqlx::Error> {
        sqlx::query_as::<_,Project>(
            "SELECT * FROM projects"
        )
        .fetch_all(pool)
        .await
    } 
    pub async fn delete(pool: &PgPool, project_id: i32) -> Result<Project, sqlx::Error> {
        sqlx::query_as::<_,Project>(
            "DELETE FROM projects WHERE id = $1 RETURNING *"
        )
        .bind(project_id)
        .fetch_one(pool)
        .await
    }

    pub async fn update(pool: &PgPool, payload: UpdateProject, project_id: i32) -> Result<Project, sqlx::Error>{
        sqlx::query_as::<_, Project>(
            "UPDATE projects SET name = $1, description = $2 WHERE id = $3 RETURNING *"
        )
        .bind(payload.name)
        .bind(payload.description)
        .bind(project_id)
        .fetch_one(pool)
        .await
    }


}
