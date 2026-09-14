.PHONY: build test clean deploy init fmt check

CONTRACT := substrata
NETWORK ?= testnet
SOURCE ?= substrata-admin

build:
	@if command -v stellar >/dev/null 2>&1; then \
		stellar contract build; \
	else \
		cargo build --target wasm32-unknown-unknown --release -p $(CONTRACT); \
	fi

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
