use rust_web_handson_domain::{model::todo::Todo, repository::RepositoriesModuleExt};

use crate::{client::mysql::Rds, repository::RdsRepositoryImpl};

pub struct RepositoriesModule {
    todo_repository: RdsRepositoryImpl<Todo>,
}

impl RepositoriesModule {
    pub async fn new() -> Self {
        let rds = Rds::new().await;
        let todo_repository = RdsRepositoryImpl::new(rds);
        Self {
            todo_repository: todo_repository,
        }
    }
}

impl RepositoriesModuleExt for RepositoriesModule {
    type TodoRepo = RdsRepositoryImpl<Todo>;
    fn todo_repository(&self) -> &Self::TodoRepo {
        &self.todo_repository
    }
}
