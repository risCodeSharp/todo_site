use serde::{Serialize, Deserialize};
use sqlx::Type;

#[derive(Debug, Serialize, Deserialize, Type)]
#[sqlx(type_name = "workstatus", rename_all = "PascalCase")]
pub enum WorkStatus {
    NotStarted,
    InProgress,
    Completed,
    Cancelled
}

#[derive(Debug, Serialize, Deserialize, Type)]
#[sqlx(type_name = "difficulty", rename_all = "PascalCase")]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}