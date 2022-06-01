use mockall::automock;
use rust_web_handson_domain::{
    model::todo::Todo,
    repository::todo::{MockTodoRepository, TodoRepository},
};

use crate::{client::mysql::Rds, repository::RdsRepositoryImpl};

pub struct RepositoriesModule {
    todo_repository: RdsRepositoryImpl<Todo>,
}

impl RepositoriesModule {
    pub fn new(rds: Rds) -> Self {
        let todo_repository = RdsRepositoryImpl::new(rds.clone());
        Self {
            todo_repository: todo_repository,
        }
    }
}

#[automock(type TodoRepo=MockTodoRepository;)]
pub trait RepositoriesModuleExt {
    type TodoRepo: TodoRepository;
    fn todo_repository(&self) -> &Self::TodoRepo;
}

impl RepositoriesModuleExt for RepositoriesModule {
    type TodoRepo = RdsRepositoryImpl<Todo>;
    fn todo_repository(&self) -> &Self::TodoRepo {
        &self.todo_repository
    }
}
