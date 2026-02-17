use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::types::{Difficulty, WorkStatus};

#[derive(Serialize, Deserialize, FromRow)]
pub struct Job {
    pub id: i32,
    pub task_id: i32,
    pub name: String,
    pub status: WorkStatus,
    pub difficulty: Difficulty,
}

#[derive(Serialize, Deserialize)]
pub struct CreateJob {
    pub name: String,
}
#[derive(Serialize, Deserialize)]
pub struct UpdateJob {
    pub name: String,
    pub status: WorkStatus,
    pub difficulty: Difficulty,
}