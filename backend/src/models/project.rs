use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Project {
    pub id: i32,
    pub name: String,
    pub tags: Option<Vec<String>>,
}


#[derive(Deserialize, Serialize)]
pub struct CreateProject {
    pub name: String,
    pub tags: Option<Vec<String>>,
}

#[derive(Deserialize, Serialize)]
pub struct UpdateProject {
    pub name: String,
    pub tags: Option<Vec<String>>,
}
