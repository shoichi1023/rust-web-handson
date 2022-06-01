use std::sync::Arc;

use crate::usecase::todo::TodoUseCase;
use rust_web_handson_infra::{client::mysql::Rds, modules::RepositoriesModule};

use mockall::automock;

pub struct UseCaseModules {
    todo_usecase: TodoUseCase<RepositoriesModule>,
}

#[automock]
impl UseCaseModules {
    pub async fn new(rds: Rds) -> Self {
        // initialize middlewares
        let repositories = Arc::new(RepositoriesModule::new(rds.clone()));

        // make usecase instances
        let todo_usecase = TodoUseCase::new(repositories.clone());

        // make di container
        Self { todo_usecase }
    }

    pub fn todo_usecase(&self) -> &TodoUseCase<RepositoriesModule> {
        &self.todo_usecase
    }
}
