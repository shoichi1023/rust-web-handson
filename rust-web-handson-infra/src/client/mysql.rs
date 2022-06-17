use std::env;
use std::sync::Arc;

use sqlx::{mysql::MySqlPoolOptions, MySql, Pool};

#[derive(Clone)]
pub struct Rds(pub(crate) Arc<Pool<MySql>>);

impl Rds {
    // new はコンストラクタと思えば良い
    pub async fn new() -> Rds {
        let pool = MySqlPoolOptions::new()
            .max_connections(
                // :: はクラスのメソッドを呼び出す
                // この型のこのメソッドを呼び出す (実態のメソッド、セルフを実装しているかどうか)
                // * 借用して、そのポインタを渡している
                *&env::var("DATABASE_CONNECTIONS") // 環境変数から引っ張ってくる
                    // それが取れているかどうかを確認する
                    // 取れなかったらパニックしている、アプリが起動しないので
                    .unwrap_or_else(|_| panic!("DATABASE_CONNECTIONS must be set!"))
                    // u32 にパース (キャスト) している
                    .parse::<u32>()
                    .unwrap_or_else(|_| panic!("DATABASE_CONNECTIONS must be integer!")),
            )
            // connect が非同期なので、await している
            .connect(
                &env::var("DATABASE_URL").unwrap_or_else(|_| panic!("DATABASE_URL must be set!")),
            )
            .await
            // 型が帰ってきたので、それを wrap している
            .unwrap_or_else(|_| {
                panic!("Cannot connect to the database. Please check your configuration.")
            });
        Rds(Arc::new(pool))
    }
}
