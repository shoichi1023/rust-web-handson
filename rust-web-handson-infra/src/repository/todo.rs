use async_trait::async_trait;
use chrono::{Local, TimeZone};
use rust_web_handson_domain::{
    model::todo::{NewTodo, Todo},
    repository::todo::TodoRepository,
};
use sqlx::{mysql::MySqlQueryResult, query_as};

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
        sqlx::query("insert into todo (title, description) values (?, ?)")
            .bind(source.title)
            .bind(source.description)
            .execute(&*pool)
            .await?;
        Ok(())
    }

    async fn create_and_get_info(&self, source: NewTodo) -> anyhow::Result<Todo> {
        let pool = self.pool.0.clone();
        let result: MySqlQueryResult =
            sqlx::query("insert into todo (title, description) values (?, ?)")
                .bind(source.title)
                .bind(source.description)
                .execute(&*pool)
                .await?;

        let todo = query_as::<_, TodoTable>("select * from todo where id=?")
            .bind(result.last_insert_id())
            .fetch_one(&*pool)
            .await?;

        Todo::try_from(todo)

        // let mock_now = Local
        //     .datetime_from_str("2022/01/01 13:00:00", "%Y/%m/%d %H:%M:%S")
        //     .unwrap();
        // let mock_expect = Todo::new(
        //     1,
        //     "sample title".to_string(),
        //     "sample description".to_string(),
        //     mock_now.clone(),
        //     mock_now.clone(),
        //     Some(mock_now.clone()),
        // );
        // anyhow::Ok(mock_expect)
    }
}

#[cfg(test)]
mod test {
    use rust_web_handson_domain::{model::todo::Todo, repository::todo::TodoRepository};

    use crate::{client::mysql::Rds, repository::RdsRepositoryImpl};

    #[tokio::test]
    async fn test_get_all() -> () {
        dotenv::from_filename(".env_test");
        let rds = Rds::new().await;
        let todo_repository: RdsRepositoryImpl<Todo> = RdsRepositoryImpl::new(rds);
        let result_list = todo_repository.get_all().await.unwrap();
        println!("{:?}", result_list.get(0).unwrap());
    }
}
