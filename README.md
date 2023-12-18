# rust-axum-mysql-dynamodb-example

## Getting Started
```sh
$ docker compose up -d
```

This app now takes advantage of the following tools:
- MySQL
- DynamoDB

### Run the app

```sh
$ cp sample.envrc .envrc
$ rustup update # update rust tool chain
$ cargo update # update rust dependencies
$ cargo run # run api server
```

## Architecture

This example has 4 workspaces as following:

- rust-driver (driver or controller)
- rust-app (app or usecase)
- rust-kernel (kernel or domain)
- rust-adapter (adapter or infrastructure)

## DynamoDB
- create table
```sh
# 1.create table
$ aws dynamodb create-table \
    --region ap-northeast-1 \
    --endpoint-url http://localhost:8000 \
    --table-name examples \
    --key-schema AttributeName=id,KeyType=HASH \
    --attribute-definitions AttributeName=id,AttributeType=S \
    --billing-mode PAY_PER_REQUEST
# 2. check created tables
$ aws dynamodb list-tables --endpoint-url http://localhost:8000
# 3. insert items
$ make init-dynamodb
# 4. check inserted items
$ aws dynamodb scan --endpoint-url http://localhost:8000 --table-name examples
```
- list table
```sh
$ aws dynamodb list-tables --endpoint-url http://localhost:{PORT}
```
- create item
```sh
$ aws dynamodb put-item --endpoint-url http://localhost:{PORT} --table-name example --item '{"Id": {"N": "1"}, "Name": {"S": "user1"}, "Password": {"S": "pw1"}}'
```