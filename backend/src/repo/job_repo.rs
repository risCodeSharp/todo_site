use sqlx::PgPool;

use crate::models::job::{CreateJob, Job, UpdateJob};


pub struct JobRepository;

impl JobRepository {
    pub async fn create(pool: &PgPool, payload: CreateJob, task_id: i32) -> Result<Job, sqlx::Error> {
        sqlx::query_as::<_,Job>(
            "INSERT INTO jobs(name, task_id) VALUES ($1, $2) RETURNING *"
        )
        .bind(payload.name)
        .bind(task_id)
        .fetch_one(pool)
        .await
    }
    
    pub async fn list_all(pool: &PgPool, task_id: i32) -> Result<Vec<Job>, sqlx::Error> {
        sqlx::query_as::<_,Job>(
            "SELECT * FROM jobs WHERE task_id = $1"
        )
        .bind(task_id)
        .fetch_all(pool)
        .await
        
    }

    pub async fn delete(pool: &PgPool, job_id: i32) -> Result<Job, sqlx::Error> {
        sqlx::query_as::<_,Job>(
            "DELETE FROM jobs WHERE id = $1 RETURNING *"
        )
        .bind(job_id)
        .fetch_one(pool)
        .await
    }

    pub async fn update(pool: &PgPool, payload: UpdateJob, job_id: i32) -> Result<Job, sqlx::Error> {
        sqlx::query_as::<_,Job>(
            "UPDATE jobs SET name = $1, status = $2, difficulty = $3 WHERE id = $4 RETURNING *"
        )
        .bind(payload.name)
        .bind(payload.status)
        .bind(payload.difficulty)
        .bind(job_id)
        .fetch_one(pool)
        .await
    }
}