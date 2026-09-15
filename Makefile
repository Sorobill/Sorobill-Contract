.PHONY: build test check verify fmt clean deploy init

CONTRACT := sorobill
NETWORK ?= testnet
SOURCE ?= sorobill-admin

## Default: test + check (fast CI path)
verify: test check
	@echo "OK — contract verify passed"

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
