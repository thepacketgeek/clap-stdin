test: lint
# tests use the binaries so we need to build them first
	cargo build --bins --features test_bin
	cargo test
	cargo build --bins --features test_bin_tokio
	cargo test --features tokio

doc:
	cargo doc --features tokio

lint:
	cargo fmt --message-format human -- --check
	cargo check
	cargo check --features tokio
	RUSTDOCFLAGS=-Dwarnings cargo doc -q --no-deps --lib --features tokio
	cargo clippy -q --no-deps -- -D warnings
	cargo clippy -q --no-deps --features tokio -- -D warnings

clean:
	cargo clean
