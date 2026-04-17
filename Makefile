# xlstream — developer commands.
#
# New to the project? Just run: `make install`
# Then: `make check` to validate, `make help` to see everything.
#
# Requires GNU make (bundled on Linux, `brew install make` on macOS if you
# want the newer one; Apple's /usr/bin/make also works for our targets).

.DEFAULT_GOAL := help
SHELL := /usr/bin/env bash
.SHELLFLAGS := -eu -o pipefail -c

VENV := .venv
VENV_BIN := $(VENV)/bin
PIP := $(VENV_BIN)/pip
PRECOMMIT := $(VENV_BIN)/pre-commit

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

## install — one command to set up everything

.PHONY: install
install: check-prereqs $(VENV)/.stamp install-rust install-python install-precommit  ## new dev? run this. creates .venv, installs Rust + Python deps + pre-commit hooks
	@echo ""
	@printf "$(BOLD)✓ install complete$(RESET)\n"
	@echo ""
	@echo "Next steps:"
	@echo "  source $(VENV)/bin/activate    # activate the Python venv"
	@echo "  make check                     # validate your setup (fmt + clippy + tests)"
	@echo "  cat docs/phases/README.md      # find the current phase"
	@echo ""

.PHONY: check-prereqs
check-prereqs: ## verify required binaries are on PATH (git, python3, rustup)
	@command -v git     >/dev/null || { echo "✗ git not found" >&2; exit 1; }
	@command -v python3 >/dev/null || { echo "✗ python3 not found (need 3.9+)" >&2; exit 1; }
	@command -v rustup  >/dev/null || { echo "✗ rustup not found. Install: https://rustup.rs" >&2; exit 1; }
	@printf "  %-14s %s\n" "git:"     "$$(git --version)"
	@printf "  %-14s %s\n" "python3:" "$$(python3 --version)"
	@printf "  %-14s %s\n" "rustup:"  "$$(rustup --version | head -n1)"

# Create .venv on first install. Stamp file avoids re-running on every `make install`.
$(VENV)/.stamp:
	@echo "→ creating Python venv at $(VENV)/"
	python3 -m venv $(VENV)
	$(PIP) install --upgrade --quiet pip
	@touch $(VENV)/.stamp

.PHONY: install-rust
install-rust: ## install Rust toolchain from rust-toolchain.toml + rustfmt + clippy + rust-src
	@echo "→ installing Rust toolchain (from rust-toolchain.toml)"
	rustup show
	rustup component add rustfmt clippy rust-src

.PHONY: install-python
install-python: $(VENV)/.stamp ## install maturin + pytest + ruff + pre-commit into .venv
	@echo "→ installing Python dev dependencies into $(VENV)/"
	$(PIP) install --quiet --upgrade maturin pytest ruff pre-commit

.PHONY: install-precommit
install-precommit: $(VENV)/.stamp install-python ## install git hooks (pre-commit, commit-msg, pre-push)
	@echo "→ installing git hooks"
	$(PRECOMMIT) install --install-hooks
	$(PRECOMMIT) install --hook-type commit-msg
	$(PRECOMMIT) install --hook-type pre-push

.PHONY: uninstall
uninstall: ## remove .venv, git hooks, and cargo target
	$(PRECOMMIT) uninstall --hook-type pre-commit || true
	$(PRECOMMIT) uninstall --hook-type commit-msg || true
	$(PRECOMMIT) uninstall --hook-type pre-push   || true
	rm -rf $(VENV) target
	@echo "✓ uninstall complete. Cloned files are untouched."

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
