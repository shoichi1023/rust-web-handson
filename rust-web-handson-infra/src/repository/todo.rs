use async_trait::async_trait;
use rust_web_handson_domain::{
    model::todo::{NewTodo, Todo},
    repository::todo::TodoRepository,
};
use sqlx::query_as;

use crate::dao::todo::TodoTable;

use super::RdsRepositoryImpl;

#[async_trait]
impl TodoRepository for RdsRepositoryImpl<Todo> {
    async fn get_all(&self) -> anyhow::Result<Vec<Todo>> {
        let pool = self.pool.0.clone();
        let todo_list = query_as::<_, TodoTable>("select * from todo")
            .fetch_all(&*pool)
            .await?;
        todo_list.into_iter().map(|t| Todo::try_from(t)).collect()
    }

    async fn insert(&self, source: NewTodo) -> anyhow::Result<()> {
        let pool = self.pool.0.clone();
        sqlx::query("insert into todo (name, description) values (?, ?)")
            .bind(source.title)
            .bind(source.description)
            .execute(&*pool)
            .await?;
        Ok(())
    }
}
