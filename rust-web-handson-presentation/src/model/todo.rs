use rust_web_handson_domain::model::todo::Todo;
use serde::Serialize;

#[derive(Serialize)]
pub struct TodoJson {
    id: i64,
    title: String,
    description: String,
    created_at: String,
}

impl From<Todo> for TodoJson {
    fn from(t: Todo) -> Self {
        TodoJson {
            id: t.id,
            title: t.title,
            description: t.description,
            created_at: t.created_at.to_string(),
        }
    }
}
