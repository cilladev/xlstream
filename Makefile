# xlstream — developer commands.
#
# Requires GNU make (bundled on Linux, `brew install make` on macOS if you
# want the newer one; Apple's /usr/bin/make also works for our targets).
#
# Run `make help` for the list.

.DEFAULT_GOAL := help
SHELL := /usr/bin/env bash
.SHELLFLAGS := -eu -o pipefail -c

# Colours for `make help`. Set NO_COLOR=1 to disable.
ifdef NO_COLOR
BOLD :=
DIM :=
RESET :=
else
BOLD := $(shell tput bold 2>/dev/null || echo)
DIM := $(shell tput dim 2>/dev/null || echo)
RESET := $(shell tput sgr0 2>/dev/null || echo)
endif

# -----------------------------------------------------------------------------
# meta
# -----------------------------------------------------------------------------

.PHONY: help
help: ## show this help
	@echo "$(BOLD)xlstream — developer commands$(RESET)"
	@echo ""
	@awk 'BEGIN {FS = ":.*?## "} \
	     /^## / {printf "\n$(BOLD)%s$(RESET)\n", substr($$0, 4); next} \
	     /^[a-zA-Z_-]+:.*?## / {printf "  $(BOLD)%-22s$(RESET) %s\n", $$1, $$2}' $(MAKEFILE_LIST)
	@echo ""

## setup

.PHONY: setup
setup: setup-rust setup-precommit setup-python ## install toolchain + hooks + Python dev deps
	@echo "✓ setup complete. Run 'make check' to validate."

.PHONY: setup-rust
setup-rust: ## install Rust toolchain from rust-toolchain.toml (rustup auto-handles it)
	@rustup show >/dev/null || (echo "rustup not found; install from https://rustup.rs"; exit 1)
	@rustup component add rustfmt clippy rust-src

.PHONY: setup-precommit
setup-precommit: ## install pre-commit hooks (commit, commit-msg, pre-push)
	@command -v pre-commit >/dev/null || pip install --user pre-commit
	pre-commit install --install-hooks
	pre-commit install --hook-type commit-msg
	pre-commit install --hook-type pre-push

.PHONY: setup-python
setup-python: ## install maturin + pytest + ruff into the current venv
	pip install maturin pytest ruff

# -----------------------------------------------------------------------------
# Rust
# -----------------------------------------------------------------------------

## Rust — format, lint, test

.PHONY: fmt
fmt: ## cargo fmt --all
	cargo fmt --all

.PHONY: fmt-check
fmt-check: ## cargo fmt --check (non-destructive)
	cargo fmt --all --check

.PHONY: lint
lint: ## cargo clippy -D warnings
	cargo clippy --workspace --all-targets --all-features -- -D warnings

.PHONY: test
test: ## cargo test workspace + doctests
	cargo test --workspace --all-features
	cargo test --workspace --doc

.PHONY: test-fast
test-fast: ## cargo test, no doctests, no all-features
	cargo test --workspace

.PHONY: check
check: fmt-check lint test ## the full local gate: fmt + clippy + tests + doctests

.PHONY: build
build: ## cargo build --release
	cargo build --workspace --release

.PHONY: doc
doc: ## cargo doc (no deps) and open in browser
	cargo doc --workspace --no-deps --open

.PHONY: audit
audit: ## cargo audit (security advisories)
	@command -v cargo-audit >/dev/null || cargo install cargo-audit --locked
	cargo audit

## Rust — benchmarks

.PHONY: bench
bench: ## cargo bench --workspace (full)
	cargo bench --workspace

.PHONY: bench-quick
bench-quick: ## quick smoke benchmark (per-PR CI equivalent)
	cargo bench --bench quick -- --sample-size 10

.PHONY: bench-reference
bench-reference: ## evaluate the 400k-row reference workload with RSS tracking
	cargo run --release -p xlstream-cli -- evaluate \
		benchmarks/fixtures/reference_400k.xlsx \
		--output /tmp/xlstream-out.xlsx \
		--verbose

# -----------------------------------------------------------------------------
# Python bindings
# -----------------------------------------------------------------------------

## Python — build, test

.PHONY: py-develop
py-develop: ## maturin develop --release (rebuild + install into venv)
	cd bindings/python && maturin develop --release

.PHONY: py-build
py-build: ## maturin build --release (wheel in bindings/python/target/wheels/)
	cd bindings/python && maturin build --release

.PHONY: py-test
py-test: py-develop ## build + run pytest
	cd bindings/python && pytest -v

.PHONY: py-lint
py-lint: ## ruff check + format check on bindings/python
	cd bindings/python && ruff check . && ruff format --check .

.PHONY: py-fmt
py-fmt: ## ruff format + fix
	cd bindings/python && ruff format . && ruff check --fix .

# -----------------------------------------------------------------------------
# pre-commit (manual runs)
# -----------------------------------------------------------------------------

## pre-commit

.PHONY: pre-commit
pre-commit: ## run pre-commit hooks on all files (commit stage)
	pre-commit run --all-files

.PHONY: pre-commit-all
pre-commit-all: ## run all hooks including pre-push stage (clippy + tests)
	pre-commit run --all-files --hook-stage pre-push

.PHONY: pre-commit-update
pre-commit-update: ## update pre-commit hook repos to latest pinned versions
	pre-commit autoupdate

# -----------------------------------------------------------------------------
# docs
# -----------------------------------------------------------------------------

## docs

.PHONY: docs
docs: doc ## alias for `make doc`

.PHONY: docs-check
docs-check: ## verify internal markdown links (requires markdown-link-check; optional)
	@command -v markdown-link-check >/dev/null && \
		find docs -name '*.md' -print0 | xargs -0 -n1 markdown-link-check -q || \
		echo "skip: install markdown-link-check for link checking"

# -----------------------------------------------------------------------------
# fixtures
# -----------------------------------------------------------------------------

## fixtures

.PHONY: fixtures
fixtures: ## regenerate benchmark xlsx fixtures (deterministic, seeded)
	cargo run --release --bin generate-fixtures -p xlstream-benchmarks

.PHONY: fixtures-clean
fixtures-clean: ## remove generated fixtures (keeps committed ones)
	rm -f benchmarks/fixtures/reference_*.xlsx benchmarks/fixtures/stress_*.xlsx

# -----------------------------------------------------------------------------
# release (local dry-runs)
# -----------------------------------------------------------------------------

## release

.PHONY: release-dry
release-dry: ## cargo publish --dry-run + maturin build check
	cargo publish -p xlstream-core --dry-run
	cargo publish -p xlstream-parse --dry-run
	cargo publish -p xlstream-io --dry-run
	cargo publish -p xlstream-eval --dry-run
	cd bindings/python && maturin build --release

.PHONY: release-wheels
release-wheels: ## build local wheels into bindings/python/dist (no publish)
	cd bindings/python && maturin build --release --out dist

# -----------------------------------------------------------------------------
# housekeeping
# -----------------------------------------------------------------------------

## housekeeping

.PHONY: clean
clean: ## remove cargo target, Python build artefacts, caches
	cargo clean
	rm -rf bindings/python/target bindings/python/dist bindings/python/wheels
	find . -type d -name '__pycache__' -exec rm -rf {} + 2>/dev/null || true
	find . -type d -name '.pytest_cache' -exec rm -rf {} + 2>/dev/null || true
	find . -type d -name '.ruff_cache' -exec rm -rf {} + 2>/dev/null || true
	find . -type d -name '.mypy_cache' -exec rm -rf {} + 2>/dev/null || true

.PHONY: clean-all
clean-all: clean fixtures-clean ## clean + regeneratable fixtures
	rm -rf .venv

.PHONY: ci-local
ci-local: check py-test audit ## simulate the full CI pipeline locally
	@echo "✓ ci-local complete."
