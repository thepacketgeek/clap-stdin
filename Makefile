test: lint
# tests use the binaries so we need to build them first
	cargo build --bins --features test_bin
	cargo test
	cargo build --bins --features test_bin_tokio
	cargo test --features tokio

doc:
	cargo doc

lint:
	cargo fmt --message-format human -- --check
	cargo check
	RUSTDOCFLAGS=-Dwarnings cargo doc -q --no-deps --lib
	cargo clippy -q --no-deps -- -D warnings

clean:
	cargo clean
