use async_trait::async_trait;
use mockall::automock;

use crate::model::todo::{NewTodo, Todo};

#[automock]
#[async_trait]
pub trait TodoRepository {
    async fn get_all(&self) -> anyhow::Result<Vec<Todo>>;
    async fn insert(&self, source: NewTodo) -> anyhow::Result<()>;
}
