all: build

clean:
	cargo clean

build:
	cargo build

test: build
	cargo test

check:
	cargo check
	cargo clippy --all-targets --all-features -- -D warnings