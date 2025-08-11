run:
	cargo run

check:
	cargo check

build: 
	cargo build

build-release:
	cargo build --release

doc:
	rustup doc

lib:
	cargo new xxx --lib