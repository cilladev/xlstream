#!/usr/bin/env bash
# Validates commit message format: `<crate>: <imperative, lowercase>` OR
# one of the allowed non-crate prefixes: `docs:`, `chore:`, `ci:`, `refactor:`,
# `fix:`, `feat:`, `perf:`, `test:`, `build:`, `release:`.
#
# Run by the `commit-msg` pre-commit hook. Argument $1 is the path to the
# commit message file.

set -euo pipefail

if [[ $# -lt 1 ]]; then
    echo "commit-msg hook: missing commit message file argument" >&2
    exit 2
fi

MSG_FILE="$1"

# Read the first non-empty, non-comment line.
FIRST_LINE=$(grep -v '^#' "$MSG_FILE" | grep -v '^$' | head -n1 || true)

# Allow merge / revert / fixup commits through untouched — git generates
# these in a fixed format that our rule would reject.
if [[ "$FIRST_LINE" =~ ^(Merge|Revert|fixup!|squash!|amend!) ]]; then
    exit 0
fi

# Pattern: prefix is either a known kind OR a crate name (xlstream-...).
# Everything after the colon must start lowercase and not end with a period.
REGEX='^(xlstream-[a-z][a-z0-9-]*|docs|chore|ci|refactor|fix|feat|perf|test|build|release): [a-z].*[^.]$'

if ! [[ "$FIRST_LINE" =~ $REGEX ]]; then
    cat >&2 <<EOF
commit-msg hook: first line does not match the required format.

Got:
  ${FIRST_LINE}

Expected:
  <prefix>: <imperative, lowercase, no trailing period>

where <prefix> is one of:
  - a crate name (xlstream-core, xlstream-eval, xlstream-io, xlstream-parse,
    xlstream-cli, xlstream-python)
  - docs, chore, ci, refactor, fix, feat, perf, test, build, release

Examples:
  xlstream-eval: add VLOOKUP wildcard support
  docs: clarify whole-column aggregate handling
  chore: bump calamine to 0.35

See docs/standards/commits.md for the full convention.
EOF
    exit 1
fi

# Forbid the Claude trailers.
if grep -qE '^Co-Authored-By: Claude' "$MSG_FILE"; then
    echo "commit-msg hook: remove 'Co-Authored-By: Claude' trailer (CLAUDE.md rule)." >&2
    exit 1
fi

if grep -q 'Generated with Claude Code' "$MSG_FILE"; then
    echo "commit-msg hook: remove 'Generated with Claude Code' footer (CLAUDE.md rule)." >&2
    exit 1
fi

exit 0
