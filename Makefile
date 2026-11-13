# coding-lanes — developer targets
#
# The gate is one command: make check
#
# The CLI tool is out of scope for now; there are no targets for it.

CARGO ?= $(shell command -v cargo 2>/dev/null || echo $(HOME)/.cargo/bin/cargo)
RAUL_PKG := coding-lanes-tui

.DEFAULT_GOAL := help

.PHONY: help
help: ## Show this help
	@echo "coding-lanes — common targets"
	@echo ""
	@grep -E '^[a-zA-Z0-9_.-]+:.*##' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*## "}; {printf "  \033[36m%-14s\033[0m %s\n", $$1, $$2}'

.PHONY: build build-release
build: ## Build the workspace
	$(CARGO) build

build-release: ## Build release binaries
	$(CARGO) build --release

.PHONY: run raul
run raul: ## Run raul (TUI)
	$(CARGO) run -p $(RAUL_PKG)

.PHONY: test
test: ## Run the test suite
	$(CARGO) test --workspace

.PHONY: fmt fmt-check
fmt: ## Format the code
	$(CARGO) fmt --all

fmt-check: ## Check formatting
	$(CARGO) fmt --all -- --check

.PHONY: lint
lint: ## Run clippy with warnings as errors
	$(CARGO) clippy --all-targets -- -D warnings

.PHONY: check ci
check: fmt-check lint test ## The gate: fmt, clippy, tests
ci: check ## Alias for check

.PHONY: doc
doc: ## Build the API docs
	$(CARGO) doc --no-deps --workspace

.PHONY: clean
clean: ## Remove build artifacts
	$(CARGO) clean
