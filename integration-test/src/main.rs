// main 関数がないと怒られてしまう
fn main() {
    println!("Hello, world!");
}
#[cfg(test)]
mod tests {

    use once_cell::sync::OnceCell;
    use reqwest::Response;
    use rust_web_handson_presentation::model::todo_create_response::TodoCreateResponseJson;

    static INSTANCE: OnceCell<bool> = OnceCell::new();

    pub fn initialize() {
        INSTANCE.get_or_init(|| {
            println!("Before All");
            true
        });
        println!("Before Each");
    }

    #[tokio::test]
    async fn _201_todoを正常に作成することができる() -> anyhow::Result<()> {
        initialize();

        let client = reqwest::Client::new();
        let res: Response = client
            .post("http://127.0.0.1:8080/todo/try")
            .body("{ \"title\": \"sample title\", \"description\": \"sample description\" }")
            .header("Content-Type", "application/json")
            .send()
            .await?;

        assert_eq!(res.status().as_u16(), 201);
        assert_eq!(res.headers().contains_key("Location"), true);
        // ↓ DB 操作ができればアサーション可能
        // assert_eq!(
        //     res.headers().get("Location").unwrap(),
        //     &"http://localhost:8080/todo/1"
        // );

        // テストケースごとにテストデータを用意できれば id なども検証できる (やる必要があるかは考えどころ)
        let response_json = res.json::<TodoCreateResponseJson>().await?;
        assert_eq!(response_json.title, "sample title");
        assert_eq!(response_json.description, "sample description");
        Ok(())
    }

    /**
     * Blocking のテスト実行例
     */
    #[test]
    fn sample_e2e_test_blocking() -> Result<(), Box<dyn std::error::Error>> {
        initialize();

        let resp = reqwest::blocking::get("https://httpbin.org/get")?
            // .json::<String>()?;
            .text()?;
        println!("{:#?}", resp);
        Ok(())
    }
}
