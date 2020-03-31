watch:
	cargo watch --clear --exec test

run:
	cargo run

fmt:
	cargo +nightly fmt --all
