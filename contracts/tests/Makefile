prepare:
	rustup target add wasm32-unknown-unknown

build-administration-contract:
	cd ../administration/contract && cargo build --release --target wasm32-unknown-unknown
	wasm-strip contract/target/wasm32-unknown-unknown/release/administration-contract.wasm 2>/dev/null | true
	cp ../target/wasm32-unknown-unknown/release/administration-contract.wasm wasm

build-main-contract:
	cd ../main/contract && cargo build --release --target wasm32-unknown-unknown
	wasm-strip contract/target/wasm32-unknown-unknown/release/contract.wasm 2>/dev/null | true
	cp ../target/wasm32-unknown-unknown/release/main-contract.wasm wasm

build-price-oracle-contract:
	cd ../price-oracle/contract && cargo build --release --target wasm32-unknown-unknown
	wasm-strip contract/target/wasm32-unknown-unknown/release/price-oracle-contract.wasm 2>/dev/null | true
	cp ../target/wasm32-unknown-unknown/release/price-oracle-contract.wasm wasm

build-database-contract:
	cd ../database/contract && cargo build --release --target wasm32-unknown-unknown
	wasm-strip ../target/wasm32-unknown-unknown/release/database-contract.wasm 2>/dev/null | true
	cp ../target/wasm32-unknown-unknown/release/database-contract.wasm wasm

build-nft-core-contract:
	cd ../nft-core/contract && cargo build --release --target wasm32-unknown-unknown
	cd ../nft-core/client/mint_session && cargo build --release --target wasm32-unknown-unknown
	cd ../nft-core/client/balance_of_session && cargo build --release --target wasm32-unknown-unknown
	cd ../nft-core/client/owner_of_session && cargo build --release --target wasm32-unknown-unknown
	cd ../nft-core/client/get_approved_session && cargo build --release --target wasm32-unknown-unknown
	cd ../nft-core/client/transfer_session && cargo build --release --target wasm32-unknown-unknown
	cd ../nft-core/client/updated_receipts && cargo build --release --target wasm32-unknown-unknown
	wasm-strip ../target/wasm32-unknown-unknown/release/nft-core-contract.wasm
	wasm-strip ../target/wasm32-unknown-unknown/release/mint_call.wasm
	wasm-strip ../target/wasm32-unknown-unknown/release/balance_of_call.wasm
	wasm-strip ../target/wasm32-unknown-unknown/release/owner_of_call.wasm
	wasm-strip ../target/wasm32-unknown-unknown/release/get_approved_call.wasm
	wasm-strip ../target/wasm32-unknown-unknown/release/transfer_call.wasm
	wasm-strip ../target/wasm32-unknown-unknown/release/updated_receipts.wasm
	cp ../target/wasm32-unknown-unknown/release/nft-core-contract.wasm wasm
	cp ../target/wasm32-unknown-unknown/release/mint_call.wasm wasm
	cp ../target/wasm32-unknown-unknown/release/balance_of_call.wasm wasm
	cp ../target/wasm32-unknown-unknown/release/owner_of_call.wasm wasm
	cp ../target/wasm32-unknown-unknown/release/get_approved_call.wasm wasm
	cp ../target/wasm32-unknown-unknown/release/transfer_call.wasm wasm
	cp ../target/wasm32-unknown-unknown/release/updated_receipts.wasm wasm

build-nft-contract:
	cd ../nft-contract/contract && cargo build --release --target wasm32-unknown-unknown
	wasm-strip ../target/wasm32-unknown-unknown/release/nft-contract.wasm
	cp ../target/wasm32-unknown-unknown/release/nft-contract.wasm wasm

test:
	mkdir -p wasm
	make build-administration-contract
	# make build-main-contract
	# make build-nft-core-contract
	# make build-nft-contract
	# make build-price-oracle-contract
	# make build-database-contract
	cargo test

clippy:
	cd contract && cargo clippy --all-targets -- -D warnings
	cd tests && cargo clippy --all-targets -- -D warnings

check-lint: clippy
	cd contract && cargo fmt -- --check
	cd tests && cargo fmt -- --check

lint: clippy
	cd contract && cargo fmt
	cd tests && cargo fmt

clean:
	cd contract && cargo clean
	cd tests && cargo clean
	rm -rf tests/wasm