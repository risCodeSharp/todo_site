use sqlx::PgPool;

use crate::models::task::{CreateTask, Task, UpdateTask};

pub struct TaskRespository;

impl TaskRespository {
    pub async fn create(pool: &PgPool, payload: CreateTask, project_id: i32) -> Result<Task, sqlx::Error>{
        sqlx::query_as::<_,Task>(
            "INSERT INTO tasks(project_id, name) VALUES ($1, $2) RETURNING *"
        )
        .bind(project_id)
        .bind(payload.name)
        .fetch_one(pool)
        .await
    }
    
    pub async fn list_all(pool: &PgPool, project_id: i32) -> Result<Vec<Task>, sqlx::Error> {
        sqlx::query_as::<_,Task>(
            "SELECT * FROM tasks WHERE project_id = $1"
        )
        .bind(project_id)
        .fetch_all(pool)
        .await
    }

    pub async fn update(pool: &PgPool, payload: UpdateTask, task_id: i32) -> Result<Task, sqlx::Error> {
        sqlx::query_as::<_,Task>(
            "UPDATE tasks SET name = $1, status = $2 WHERE id = $3 RETURNING *"
        )
        .bind(payload.name)
        .bind(payload.status)
        .bind(task_id)
        .fetch_one(pool)
        .await
    }

    pub async fn delete(pool: &PgPool, task_id: i32) -> Result<Task, sqlx::Error> {
        sqlx::query_as::<_,Task>(
            "DELETE FROM tasks WHERE id = $1 RETURNING *"
        )
        .bind(task_id)
        .fetch_one(pool)
        .await
    }
}