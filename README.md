# rust_axum_mysql_dynamodb

Stock price and stats viewer.

## Getting Started

### Middleware

Launch the middleware by executing docker compose:

```
cd local-middleware
docker compose up -d
```

This app now takes advantage of the following tools:

- MySQL
- DynamoDB (future)

### Setting up database tables

Please run SQLs in `migrations` directory. `up.sql` can be up tables, `down.sql` removes them.

### Run the app

```
cargo run
```

After running the command, you can see tracing logs.

```
    Finished dev [unoptimized + debuginfo] target(s) in 0.42s
     Running `target/debug/bootstrap`
2021-12-21T13:49:54.407374Z  INFO stock_metrics_driver::startup: Server listening on 127.0.0.1:8080
```

## Architecture

This example has 4 workspaces as following:

- stock-metrics-driver (driver or controller)
- stock-metrics-app (app or usecase)
- stock-metrics-kernel (kernel or domain)
- stock-metrics-adapter (adapter or infrastructure)


## DynamoDB
- テーブル作成
```sh
$ aws dynamodb \
  --endpoint-url http://localhost:8000 \
    create-table \
  --table-name example \
  --attribute-definitions \
    AttributeName=Id,AttributeType=N \
  --key-schema \
    AttributeName=Id,KeyType=HASH \
  --billing-mode PAY_PER_REQUEST

```
- テーブル一覧
```sh
$ aws dynamodb list-tables --endpoint-url http://localhost:8000
```
- アイテム作成
```sh
$ aws dynamodb put-item --endpoint-url http://localhost:8000 --table-name example --item '{"Id": {"N": "1"}, "Name": {"S": "user1"}}'
```