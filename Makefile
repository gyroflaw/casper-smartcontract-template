
prepare:
	rustup target add wasm32-unknown-unknown

build-contract:
	cd contract && cargo build --release --target wasm32-unknown-unknown
	wasm-strip contract/target/wasm32-unknown-unknown/release/hello_contract.wasm 2>/dev/null | true

link:
	ln -s ../../target/wasm32-unknown-unknown/release/hello_contract.wasm hello_contract.wasm

test: build-contract
	cd tests && cargo test

clippy:
	cargo clippy --all-targets -- -D warnings
	cd tests && cargo clippy --all-targets -- -D warnings

check-lint: clippy
	cargo fmt -- --check
	cd tests && cargo fmt -- --check

lint: clippy
	cargo fmt
	cd tests && cargo fmt

clean:
	cd contract && cargo clean
	cd tests && cargo clean
	rm -rf tests/wasm

wasm:
	cd wasm
	wasm-pack build --target web