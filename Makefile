fmt:
	cargo fmt

run: fmt
	cargo run

test: fmt
	cargo test

clean:
	cargo clean

dynamodb-items:
	node ./mock/dynamodb/items.js