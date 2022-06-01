use std::sync::Arc;

use axum::{http::StatusCode, response::IntoResponse, routing::get, Extension, Json, Router};
use mockall_double::double;
#[double]
use rust_web_handson_app::modules::UseCaseModules;

use crate::model::todo::TodoJson;

pub fn router() -> Router {
    Router::new().route("/", get(get_all))
}

pub async fn get_all(
    Extension(modules): Extension<Arc<UseCaseModules>>,
) -> Result<impl IntoResponse, StatusCode> {
    let todo_list = modules.todo_usecase().get_list().await;
    match todo_list {
        Ok(tl) => {
            let body: Json<Vec<TodoJson>> =
                Json(tl.into_iter().map(|t| TodoJson::from(t)).collect());
            Ok((StatusCode::OK, body))
        }
        Err(e) => {
            tracing::error!("Error : {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[cfg(test)]
mod test {

    // use super::*;
    // use mockall::{mock, predicate::*};

    // use rust_web_handson_app::{modules::MockUseCaseModules, usecase::todo::MockTodoUseCase};
    // use rust_web_handson_infra::{client::mysql::Rds, modules::RepositoriesModule};

    // #[tokio::test]
    // async fn test_get_all() {
    //     let rds = Rds::new().await;
    //     let mock_module = MockUseCaseModules::new(rds).await;
    //     let mock_todo_usecase: MockTodoUseCase<RepositoriesModule> = MockTodoUseCase::new();
    //     mock_module
    //         .expect_todo_usecase()
    //         .return_const(mock_todo_usecase);

    //     let result = get_all();
    // }
}
