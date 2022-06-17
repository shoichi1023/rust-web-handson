use mockall::automock;

use self::todo::{TodoRepository, MockTodoRepository};

pub mod todo;

#[automock(type TodoRepo=MockTodoRepository;)]
pub trait RepositoriesModuleExt {
    type TodoRepo: TodoRepository + Send + Sync;
    fn todo_repository(&self) -> &Self::TodoRepo;
}
