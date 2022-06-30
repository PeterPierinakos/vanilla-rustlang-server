clean:
	cargo clean
build:
	cargo test && cargo build --release
install:
	cargo install --path .
run:
	cargo run --release