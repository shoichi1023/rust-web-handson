use chrono::{DateTime, Local};
use rust_web_handson_domain::model::todo::Todo;
use sqlx::FromRow;
#[derive(FromRow)] // from ... 変換処理を司っている Rust の基本処理 を継承するための記述、マッピングしてくれる
pub struct TodoTable {
    pub id: i64, // bigint
    pub title: String,
    pub description: String,
    pub created_at: DateTime<Local>, // chrono の Datetime → Local 時間 (動いているサーバー時間に合わせてパースする)
    pub updated_at: DateTime<Local>,
    pub deleted_at: Option<DateTime<Local>>,
}
// TodoTable.from(Row)

// Trait がデフォで用意されている
// From, TryFrom → 共通の Interface のようなもの
// ドメインモデルのメソッドとして用意している
impl TryFrom<TodoTable> for Todo {
    // try_from なので、Error のときの型を指定する必要がある
    // anyhow::Error の実体は standard IO Error → rust 標準 error の wrapper
    // Error には色々型があるが、いちいち定義するのはめんどくさいので、すべての Error を解決できる anyhow::Error を使っている
    // anyhow::Error → standard IO Error → anyhow Error に変換をしてくれる
    // standard IO Error 以外の Error の大半は standard IO Error を継承しているので大丈夫 (一部例外はある。できの悪いライブラリとか)
    // anyhow → Error の中身に関わらず、Error が来るだろうとしてメモリ領域を持っている
    // Box, Rc, Arc → 値ができた後のメモリ領域をほげほげするもの. Arc はスマートポインタを持っている、Box は値ができた後のメモリ領域を保持している
    // Box<dyn std::error::Error> → dyn は型はわからないが、 std::error::Error を継承した何らかの型がくるという記載
    // anyhow::Error の本体が↑
    // thiserr というライブラリ → struct に std::error::Error を実装してくれる。。だいたいこのライブラリを使って、エラー処理が実装されている
    type Error = anyhow::Error;
    fn try_from(tt: TodoTable) -> Result<Self, Self::Error> {
        Ok(Todo::new(
            tt.id,
            tt.title,
            tt.description,
            tt.created_at,
            tt.updated_at,
            tt.deleted_at,
        ))
    }
}
