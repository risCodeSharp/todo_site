use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Project {
    pub id: i32,
    pub name: String,
    pub description: String,
}  

#[derive(Serialize, Deserialize)]
pub struct CreateProject {
    pub name: String,
    pub description: String
}


#[derive(Serialize, Deserialize)]
pub struct UpdateProject {
    pub name: String,
    pub description: String
}

