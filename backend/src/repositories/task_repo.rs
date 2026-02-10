use crate::models::task::{CreateTask, ProjectWithTags, Task, TaskItem, UpdateTask};
use sqlx::PgPool;

pub struct TaskRepository;

impl TaskRepository {
    pub async fn create_task(
        pool: &PgPool,
        payload: CreateTask,
        project_id: i32,
    ) -> Result<Task, sqlx::Error> {
        sqlx::query_as::<_,Task>(
            "INSERT INTO tasks(project_id, name) VALUES ($1, $2) RETURNING id, project_id, name, is_completed"
        )
        .bind(project_id)
        .bind(payload.name)
        .bind(project_id)
        .fetch_one(pool)
        .await
    }
    pub async fn get_tasks_by_project_id(
        pool: &PgPool,
        project_id: i32,
    ) -> Result<ProjectWithTags, sqlx::Error> {
        let task_list: Vec<TaskItem> = sqlx::query_as::<_, TaskItem>(
            r#"
            SELECT t.id, t.name, t.is_completed 
            FROM tasks t 
            JOIN projects p ON p.id = t.project_id
            WHERE p.id = $1
            "#,
        )
        .bind(project_id)
        .fetch_all(pool)
        .await?;

        let tags =
            sqlx::query_scalar::<_, Option<Vec<String>>>("SELECT tags FROM projects WHERE id = $1")
                .bind(project_id)
                .fetch_one(pool)
                .await?;

        Ok(ProjectWithTags {
            project_id,
            task_list,
            tags,
        })
    }

    // This takes project_id, id, name ,is_completed
    pub async fn update_task(
        pool: &PgPool,
        payload: UpdateTask,
        project_id: i32,
        task_id: i32,
    ) -> Result<Task, sqlx::Error> {
        sqlx::query_as::<_,Task>(
            "UPDATE tasks SET name = $1, is_completed = $2 WHERE project_id = $3 AND id = $4 RETURNING id, project_id, name, is_completed"
        )
        .bind(payload.name)
        .bind(payload.is_completed)
        .bind(project_id)
        .bind(task_id)
        .fetch_one(pool)
        .await
    }

    pub async fn delete_task(
        pool: &PgPool,
        project_id: i32,
        task_id: i32,
    ) -> Result<Task, sqlx::Error> {
        sqlx::query_as::<_, Task>("DELETE FROM tasks WHERE project_id = $1 AND id = $2 RETURNING *")
            .bind(project_id)
            .bind(task_id)
            .fetch_one(pool)
            .await
    }
}

// impl TaskRepository {
//     pub async fn create(pool: &PgPool, payload: CreateTask) -> Result<Task, sqlx::Error> {
//         sqlx::query_as::<_, Task>(
//             "INSERT INTO tasks (name, is_completed) VALUES ($1, $2) RETURNING id, name, is_completed"
//         )
//         .bind(payload.name)
//         .bind(false)
//         .fetch_one(pool)
//         .await
//     }

//     pub async fn update_by_id(pool: &PgPool, id: i32, payload: UpdateTask) -> Result<Task, sqlx::Error> {
//         sqlx::query_as::<_,Task> (
//             "UPDATE tasks SET is_completed = $1 WHERE id = $2 RETURNING id, name, is_completed"
//         )
//         .bind(payload.is_completed)
//         .bind(id)
//         .fetch_one(pool)
//         .await
//     }

//     pub async fn get_all_tasks(pool: &PgPool) -> Result<Vec<Task>, sqlx::Error> {
//         sqlx::query_as::<_,Task> (
//             "SELECT * FROM tasks"
//         )
//         .fetch_all(pool)
//         .await
//     }

//     pub async fn delete_task_by_id(pool: &PgPool, id: i32) -> Result<u64, sqlx::Error> {
//         let result = sqlx::query!("DELETE FROM tasks WHERE id = $1", id)
//         .execute(pool).await?;
//         println!("Deleted {} row(s)", result.rows_affected());
//         Ok(result.rows_affected())
//     }
// //     pub async fn delete_task(pool: PgPool, id: usize)
// //         ->
// }
