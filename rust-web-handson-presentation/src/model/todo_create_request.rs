use rust_web_handson_domain::model::todo::NewTodo;
use serde::{Deserialize};

// フィールドは public にしてもいいかな...?　どっちでもいい。
// ボイラープレートが増えるので、public にすることが多い
// ボイラープレート... プログラム上必要なんだけど冗長なコード
#[derive(Deserialize, Debug)]
pub struct TodoCreateRequestJson {
    pub title: String,
    pub description: String
}

impl TodoCreateRequestJson {
    pub fn new (title: String, description: String) -> Self {

        Self {
            title,
            description
        }
    }

    // この時点で参照を渡しているが、getter で指定したいならここで clone してしまったほうが良い
    // 基本的にはフィールドを public にして対応する
    pub fn get_title(&self) -> &String {
        &self.title
    }

    pub fn get_description(&self) -> &String {
        &self.description
    }
}

// from trait を実装する
impl From<TodoCreateRequestJson> for NewTodo {

    // From には Error の概念がないのでいらない。TryFrom には必要
    // type Error = anyhow::Error;

    // from メソッドを実装
    fn from(todo_create_requet_json: TodoCreateRequestJson) -> Self {
        NewTodo::new(
            todo_create_requet_json.title,
            todo_create_requet_json.description,
        )
    }
}