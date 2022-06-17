use std::sync::Arc;

use axum::{
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Extension, Json, Router,
};

use rust_web_handson_app::modules::UseCaseModules;

use rust_web_handson_app::{modules::UseCaseModulesExt, usecase::todo::TodoUseCase};
use rust_web_handson_domain::model::todo::NewTodo;

use crate::model::{
    todo::TodoJson, todo_create_request::TodoCreateRequestJson,
    todo_create_response::TodoCreateResponseJson,
};

// insert をするときは route を追加 → 対応するメソッドを追加する
pub fn router() -> Router {
    return Router::new()
        .route("/", get(get_all::<UseCaseModules>))
        .route("/", post(create::<UseCaseModules>))
        .route("/try", post(create_try::<UseCaseModules>));
}

/**
 * Todo を取得する.
 */
pub async fn get_all<U: UseCaseModulesExt>(
    Extension(modules): Extension<Arc<U>>,
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

/**
 * Todo を作成する (Hands On ver)
 */
pub async fn create<U: UseCaseModulesExt>(
    Json(request_json): Json<TodoCreateRequestJson>,
    Extension(modules): Extension<Arc<U>>,
) -> Result<impl IntoResponse, StatusCode> {
    let result = modules
        .todo_usecase()
        .create_todo(NewTodo::from(request_json))
        .await;

    match result {
        Ok(_result) => {
            let mut headers = HeaderMap::new();
            headers.insert("Location", "http://localhost:8080/todo/1".parse().unwrap());

            return Ok((StatusCode::CREATED, headers));
        }
        Err(e) => {
            tracing::error!("Error : {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }
}

/**
 * Todo を作成する (Try バージョン)
 */
pub async fn create_try<U: UseCaseModulesExt>(
    Json(request_json): Json<TodoCreateRequestJson>,
    Extension(modules): Extension<Arc<U>>,
) -> Result<impl IntoResponse, StatusCode> {
    let result = modules
        .todo_usecase()
        .create_todo_and_get_info(NewTodo::from(request_json))
        .await;

    match result {
        Ok(_result) => {
            let mock_response: TodoCreateResponseJson = TodoCreateResponseJson::new(
                _result.id,
                _result.title,
                _result.description,
                _result.created_at.to_string(),
            );
            let body: Json<TodoCreateResponseJson> = Json(mock_response);

            let mut headers = HeaderMap::new();
            let header_value =
                String::from("http://localhost:8080/todo/") + &_result.id.to_string();
            headers.insert("Location", header_value.parse().unwrap());

            Ok((StatusCode::CREATED, headers, body))
        }
        Err(e) => {
            tracing::error!("Error : {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[cfg(test)]
mod tests {

    use axum::{body::Body, http::Request};
    use chrono::{Local, TimeZone};
    use rust_web_handson_app::{modules::MockUseCaseModulesExt, usecase::todo::MockTodoUseCase};
    use rust_web_handson_domain::model::todo::Todo;
    use tower::ServiceExt;

    use super::*;

    pub fn test_router() -> Router {
        return Router::new()
            .route("/", get(get_all::<MockUseCaseModulesExt>))
            .route("/", post(create::<MockUseCaseModulesExt>))
            .route("/try", post(create_try::<MockUseCaseModulesExt>));
    }

    #[tokio::test]
    async fn create_tryをtowerでテストしてみる() {
        let mut mock_usecase_module = MockUseCaseModulesExt::new();
        let mut mock_todo_usecase = MockTodoUseCase::new();

        let mock_now = Local
            .datetime_from_str("2022/01/01 13:00:00", "%Y/%m/%d %H:%M:%S")
            .unwrap();

        let select = Todo::new(
            1,
            "sample title".to_string(),
            "sample description".to_string(),
            mock_now.clone(),
            mock_now.clone(),
            Some(mock_now.clone()),
        );

        let expect_result: anyhow::Result<Todo> = anyhow::Ok(select);

        mock_todo_usecase
            .expect_create_todo_and_get_info()
            .return_once(|actual_request| {
                let expect_request: NewTodo =
                    NewTodo::new("sample title".to_string(), "sample description".to_string());
                assert_eq!(actual_request, expect_request);
                expect_result
            });

        mock_usecase_module
            .expect_todo_usecase()
            .once()
            .return_const(mock_todo_usecase);

        let modules = Arc::new(mock_usecase_module);

        let router = test_router();
        let app = router.layer(Extension(modules));

        let response = app
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .header("Content-Type", "application/json")
                    .uri("/try")
                    .body(Body::from(
                        "{ \"title\": \"sample title\", \"description\": \"sample description\" }"
                            .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        println!("body: {:#?}", body);

        assert_eq!(&body[..], b"{\"id\":1,\"title\":\"sample title\",\"description\":\"sample description\",\"created_at\":\"2022-01-01 13:00:00 +09:00\"}");
    }

    #[tokio::test]
    async fn get_allが正常に成功した場合はStatusCode_OKが取得できる() {
        // setup
        let mut mock_usecase_module = MockUseCaseModulesExt::new();
        let mut mock_todo_usecase = MockTodoUseCase::new();

        let mock_now = Local
            .datetime_from_str("2022/01/01 13:00:00", "%Y/%m/%d %H:%M:%S")
            .unwrap();

        let select = vec![Todo::new(
            1,
            "hoge".to_string(),
            "fuga".to_string(),
            mock_now.clone(),
            mock_now.clone(),
            Some(mock_now.clone()),
        )];

        let expect_result: anyhow::Result<Vec<Todo>> = anyhow::Ok(select.to_vec());

        mock_todo_usecase
            .expect_get_list()
            .return_once(|| expect_result);

        mock_usecase_module
            .expect_todo_usecase()
            .once()
            .return_const(mock_todo_usecase);

        let modules = Arc::new(mock_usecase_module);
        let router = test_router();
        let app = router.layer(Extension(modules));

        // execute
        let response = app
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        // assert
        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        println!("body: {:#?}", body);
        assert_eq!(&body[..], b"[{\"id\":1,\"title\":\"hoge\",\"description\":\"fuga\",\"created_at\":\"2022-01-01 13:00:00 +09:00\"}]");
    }
}
