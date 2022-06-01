use chrono::{DateTime, Local};
use rust_web_handson_domain::model::todo::Todo;
use sqlx::FromRow;

#[derive(FromRow)]
pub struct TodoTable {
    pub id: i64,
    pub title: String,
    pub description: String,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
    pub deleted_at: Option<DateTime<Local>>,
}

impl TryFrom<TodoTable> for Todo {
    type Error = anyhow::Error;
    fn try_from(tt: TodoTable) -> Result<Self, Self::Error> {
        Ok(Todo::new(
            tt.id,
            tt.title,
            tt.description,
            tt.created_at,
            tt.updated_at,
            tt.deleted_at,
        ))
    }
}
