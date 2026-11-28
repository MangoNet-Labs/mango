#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."

case "${1:-}" in
  quick)
    bash -n ci/check.sh
    git diff --check
    git diff --cached --check
    if [[ -n "${2:-}" ]]; then
      base=$(git rev-parse --verify "${2}^{commit}")
      git diff --check "$base" HEAD
    fi
    test -f rust-toolchain.toml
    test -f Cargo.lock
    test -s SECURITY.md
    ;;
  rpc-regressions)
    cargo test --locked --jobs 1 -p mgo-graphql-rpc --lib types::cursor::tests:: -- --nocapture
    cargo test --locked --jobs 1 -p mgo-indexer --bin mgo-indexer tests::test_indexer_config_debug_does_not_expose_connection_values -- --exact
    ;;
  rpc-config)
    cargo test --locked --jobs 1 -p mgo-graphql-rpc --lib config::tests:: -- --nocapture
    ;;
  *)
    printf 'Usage: %s {quick [base-sha]|rpc-regressions|rpc-config}\n' "$0" >&2
    exit 2
    ;;
esac
