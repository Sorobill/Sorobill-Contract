.PHONY: build test check verify fmt clean deploy init help

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

## Print available targets
help:
	@echo "Sorobill contract targets:"
	@echo "  verify  - test + check"
	@echo "  test    - cargo test -p sorobill"
	@echo "  check   - cargo check -p sorobill"
	@echo "  build   - stellar contract build (or wasm cargo fallback)"
	@echo "  fmt     - cargo fmt --all"
	@echo "  clean   - cargo clean"
	@echo "  deploy  - build + scripts/deploy.sh (NETWORK, SOURCE)"
	@echo "  init    - scripts/init.sh (NETWORK, SOURCE)"
	@echo "See docs/MAKEFILE.md for details."
