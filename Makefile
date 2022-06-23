clean:
	cargo clean
build:
	cargo build --release
install:
	cargo install --path .
run:
	cargo test && cargo run --release