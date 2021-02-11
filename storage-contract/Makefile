prepare:
	rustup target add wasm32-unknown-unknown

build-contract:
	cargo build --release -p contract --target wasm32-unknown-unknown

copy-keys-from-nctl:
	cp /home/ethilios/CasperLabs/casper-node/utils/nctl/assets/net-1/nodes/node-2/keys/* ../src/keys

deploy-contract:
	make copy-keys-from-nctl
	RUST_BACKTRACE=full casper-client put-deploy -v\
		-n http://localhost:50101 \
		-k ../src/keys/secret_key.pem \
		--chain-name casper-net-1 \
		-s ./target/wasm32-unknown-unknown/release/contract.wasm \
		-p 10000000000000 | jq

test-only:
	cargo test -p tests

lint:
	cargo fmt
	cargo clippy --all-targets --all -- -D warnings -A renamed_and_removed_lints

clean:
	cargo clean

copy-wasm-file-to-test:
	cp target/wasm32-unknown-unknown/release/contract.wasm tests/wasm

test: build-contract copy-wasm-file-to-test test-only

