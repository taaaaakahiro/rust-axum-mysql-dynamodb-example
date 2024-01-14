fmt:
	cargo fmt

run: fmt
	cargo run

test: fmt
	cargo test

clean:
	cargo clean

dynamodb-create-table:
	aws dynamodb create-table \
        --region ap-northeast-1 \
        --endpoint-url http://localhost:8000 \
        --table-name users \
        --key-schema AttributeName=id,KeyType=HASH \
        --attribute-definitions AttributeName=id,AttributeType=S \
        --billing-mode PAY_PER_REQUEST

dynamodb-insert-items:
	node ./mock/dynamodb/users.js

dynamodb-delete-table:
	aws dynamodb delete-table \
		--table-name users \
		--endpoint-url http://localhost:8000

dynamodb-init: dynamodb-delete-table dynamodb-create-table dynamodb-insert-items


.PHONY: fmt run test clean dynamodb-create-table dynamodb-delete-table dynamodb-insert-items