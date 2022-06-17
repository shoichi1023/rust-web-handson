use std::result;

use async_trait::async_trait;
use chrono::{Local, TimeZone};
use mockall::automock;
use rust_web_handson_domain::{
    model::todo::{NewTodo, Todo},
    repository::{todo::TodoRepository, RepositoriesModuleExt},
};

use super::UseCaseImpl;

#[automock]
#[async_trait]
pub trait TodoUseCase {
    async fn get_list(&self) -> anyhow::Result<Vec<Todo>>;
    async fn create_todo(&self, new_todo: NewTodo) -> anyhow::Result<()>;
    async fn create_todo_and_get_info(&self, new_todo: NewTodo) -> anyhow::Result<(Todo)>;
}

// 動的型付をする際に、安全に並列処理が実行できるようにする
#[async_trait]
impl<R: RepositoriesModuleExt + Sync + Send> TodoUseCase for UseCaseImpl<Todo, R> {
    async fn get_list(&self) -> anyhow::Result<Vec<Todo>> {
        self.repositories.todo_repository().get_all().await
    }

    async fn create_todo(&self, new_todo: NewTodo) -> anyhow::Result<()> {
        self.repositories.todo_repository().insert(new_todo).await
    }

    async fn create_todo_and_get_info(&self, new_todo: NewTodo) -> anyhow::Result<(Todo)> {
        self.repositories
            .todo_repository()
            .create_and_get_info(new_todo)
            .await
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use chrono::Local;
    use rust_web_handson_domain::repository::{
        todo::MockTodoRepository, MockRepositoriesModuleExt,
    };
    use std::sync::Arc;

    #[tokio::test]
    async fn test_get_list() -> () {
        // setup
        let mut mock_repositories = MockRepositoriesModuleExt::new();
        let mut mock_todo_repo = MockTodoRepository::new();

        let now = Local::now();

        let select = vec![Todo::new(
            1,
            "hoge".to_string(),
            "fuga".to_string(),
            now.clone(),
            now.clone(),
            Some(now.clone()),
        )];

        let expect_result: anyhow::Result<Vec<Todo>> = anyhow::Ok(select.to_vec());

        mock_todo_repo
            .expect_get_all()
            .return_once(|| expect_result);

        mock_repositories
            .expect_todo_repository()
            .once()
            .return_const(mock_todo_repo);

        let todo_usecase = UseCaseImpl::new(Arc::new(mock_repositories));

        // execte
        let result = todo_usecase.get_list().await.unwrap();

        let expect = vec![Todo::new(
            1,
            "hoge".to_string(),
            "fuga".to_string(),
            now.clone(),
            now.clone(),
            Some(now.clone()),
        )];

        // assert
        assert_eq!(result, expect);
    }

    #[tokio::test]
    async fn test_NewTodoを作成して作成した結果を返す() -> () {
        let mut mock_repositories = MockRepositoriesModuleExt::new();
        let mut mock_todo_repo = MockTodoRepository::new();

        let mock_now = Local
            .datetime_from_str("2022/01/01 13:00:00", "%Y/%m/%d %H:%M:%S")
            .unwrap();

        let mock_expect = Todo::new(
            1,
            "sample title".to_string(),
            "sample description".to_string(),
            mock_now.clone(),
            mock_now.clone(),
            Some(mock_now.clone()),
        );

        mock_todo_repo
            .expect_create_and_get_info()
            .return_once(|mock_request: NewTodo| {
                assert_eq!(mock_request.title, "sample title");
                assert_eq!(mock_request.description, "sample description");
                anyhow::Ok(mock_expect)
            });

        mock_repositories
            .expect_todo_repository()
            .once()
            .return_const(mock_todo_repo);

        let expect = Todo::new(
            1,
            "sample title".to_string(),
            "sample description".to_string(),
            mock_now.clone(),
            mock_now.clone(),
            Some(mock_now.clone()),
        );

        let new_todo = NewTodo::new("sample title".to_string(), "sample description".to_string());

        let todo_usecase = UseCaseImpl::new(Arc::new(mock_repositories));

        // execute
        let result = todo_usecase
            .create_todo_and_get_info(new_todo)
            .await
            .unwrap();

        //assert
        assert_eq!(result, expect);
    }
}
