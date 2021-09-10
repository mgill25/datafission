clip:
	cargo clippy -- -D warnings

ptest:
	cargo test -- --nocapture
