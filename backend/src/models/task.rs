use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Task {
    pub id: i32,
    pub project_id: i32,
    pub name: String,
    pub is_completed: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectWithTags {
    pub project_id: i32,
    pub task_list: Vec<TaskItem>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct TaskItem {
    pub id: i32,
    pub name: String,
    pub is_completed: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTask {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTask {
    pub name: String,
    pub is_completed: bool,
}
