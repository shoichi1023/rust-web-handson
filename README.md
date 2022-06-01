# rust-web-handson

このリポジトリは、rust (主にサーバーサイドアプリケーション領域) に習熟するための訓練用リポジトリです。

## Installation


### Sql Migrate

rdsのmigrateにsql-cliを利用しています。
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