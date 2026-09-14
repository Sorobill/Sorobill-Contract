.PHONY: build test clean deploy init fmt check

CONTRACT := substrata
WASM := target/wasm32-unknown-unknown/release/$(CONTRACT).wasm
NETWORK ?= testnet
SOURCE ?= substrata-admin

build:
	cargo build --target wasm32-unknown-unknown --release -p $(CONTRACT)

test:
	cargo test -p $(CONTRACT)

check:
	cargo check -p $(CONTRACT)

fmt:
	cargo fmt --all

clean:
	cargo clean

deploy: build
	@./scripts/deploy.sh --network $(NETWORK) --source $(SOURCE)

init:
	@./scripts/init.sh --network $(NETWORK) --source $(SOURCE)

optimize: build
	@if command -v stellar >/dev/null 2>&1; then \
		stellar contract optimize --wasm $(WASM); \
	else \
		echo "stellar CLI not found; skipping optimize"; \
	fi
