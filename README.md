# rust-web-handson

このリポジトリは、rust (主にサーバーサイドアプリケーション領域) に習熟するための訓練用リポジトリです。

## Installation


### Sql Migrate

rdsのmigrateにsqlx-cliを利用しています。
cargoによりインストールしてください。

```bash
cargo install sqlx-cli
```

### Middleware

ミドルウェアの環境構築にdocker composeを利用しています。
各自インストールし、下記コマンドで起動してください。

```bash
cd local-middleware
docker-compose up -d
```


## Getting Started


### Sql Migrate

rdsコンテナの起動を確認したら下記コマンドでrdsのデータのmigrateを行ってください。

```bash
sqlx migrate run --source ./migrations/schema --ignore-missing
```

必要であればテストデータのmigrateも行ってください。

```bash
sqlx migrate run --source ./migrations/test-data --ignore-missing
```

### Build & Run

下記コマンドで `./rust-web-handson-presentation/src/bin` 配下のファイルごとにビルドが行われます。
実行後、`target/debug` 配下にバイナリが生成されるので任意の場所から実行してください。

```bash
cargo build
```

また、下記コマンドではビルドから起動までを一括で行なえます。
好みに合わせて使い分けてください。

```bash
cargo run --bin $file-name
```
* 例:
```bash
cargo run --bin todo
```

### Testing
それぞれのパッケージで実施しているテストとその概要を記載します

#### integration-test
* 作成した TODO API を実際に HTTP リクエストを実施してアサーションします
* HTTP Client として reqwest を利用しています
* テスト実行の際にはアプリケーションをローカル環境で起動させてください

#### rust-web-handson-presentation
* テストの際は application 層をモックにし、application 層の振る舞いを定義して実行しています
* モックには `mockall`, `mockall_double` を利用しています
* tower を使ったテストを実施しています: https://github.com/tokio-rs/axum/blob/main/examples/testing/src/main.rs

#### rust-web-handson-app
* テストの際は infrastructure 層をモックにし、infrastructure 層の振る舞いを定義して実行しています
* モックには `mockall`, `mockall_double` を利用しています

#### rust-web-handson-infra
* テストの際は、実際にローカルで起動している DB に対して接続を実施し、SQL が実行できているかどうかを検証するテストを記載しています
  * 単体テスト時に、専用のインメモリ DB を建てることも検討しましたが、現在良さげなツールがなかったので↑の方針としています

# HTTP Client
* http ディレクトリでは、.http ファイルが格納されており、VS Code の [REST Client](https://marketplace.visualstudio.com/items?itemName=humao.rest-client) プラグインを利用することで簡単に HTTP リクエストを実施することができます。
* .http では環境変数によって値を変えることができます。(local の場合は url: localhost, port:8080, dev の場合は url: hgoehoge.com, port: 80 など)
  * 環境変数の設定: "F1" → "setting json" で "Open Settings(JSON)" を選択し、`settings.json` に以下を追加
```json
    "rest-client.environmentVariables": {
        "local": {
            "baseUrl": "http://localhost:8080"
        }
    },
```
ファイル全体例: 
```json
{
    "workbench.editor.enablePreview": false,
    "workbench.editor.enablePreviewFromCodeNavigation": true,
    "rest-client.environmentVariables": {
        "local": {
            "baseUrl": "http://localhost:8080"
        }
    },
    "editor.formatOnSave": true
}
```
