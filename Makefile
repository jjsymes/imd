ARGS?=

setup:
	rustup target add x86_64-pc-windows-gnu
	rustup target add x86_64-unknown-linux-gnu

build:
	cargo build --target x86_64-pc-windows-gnu
	cargo build --target x86_64-unknown-linux-gnu

run:
	cargo run -- $(ARGS)

test:
	cargo test

release:
	cargo build --release --target x86_64-pc-windows-msvc
	cargo build --release --target x86_64-unknown-linux-gnu

variables:
	@echo "ARGS: $(ARGS)"