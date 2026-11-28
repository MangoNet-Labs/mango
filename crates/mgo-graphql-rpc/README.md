# mgo-graphql-rpc

## Dev setup
Note that we use compilation flags to determine the backend for Diesel. If you're using VS Code, make sure to update settings.json with the appropriate features - there should at least be a "pg_backend" (or other backend.)
```
"rust-analyzer.cargo.features": ["pg_backend"]
```
Consequently, you'll also need to specify the backend when running cargo commands:
```cargo run --features "pg_backend" --bin mgo-graphql-rpc start-server --db-url <DB_URL>```

The order is important:
1. --features "pg_backend": This part tells Cargo to enable the pg_backend feature.
2. --bin mgo-graphql-rpc: This specifies which binary to run.
3. start-server --db-url: These are arguments to the binary.

## Spinning up locally

### Setting up local db

Rpc 1.5 backs the graphql schema with a db based on IndexerV2 schema. To spin up a local db.

If you have not created a db yet, you can do so as follows:
```sh
psql -U postgres
CREATE DATABASE mgo_indexer_v2;
```

You should be able to refer to the db url now:
`psql postgres://postgres:postgrespw@localhost:5432/mgo_indexer_v2`

With the new db, run the following commands (also under `mgo/crates/mgo-indexer`):

```sh
diesel setup --database-url="<DATABASE_URL>" --migration-dir=migrations_v2
diesel migration run --database-url="<DATABASE_URL>" --migration-dir=migrations_v2
```

### Validating configuration

From the repository root, build the binary and check either the [TOML service
configuration](config-examples/service.toml) or the [YAML server
configuration](config-examples/server.yaml).

```sh
cargo run --locked -p mgo-graphql-rpc --bin mgo-graphql-rpc -- \
  check-config --format service-toml --path crates/mgo-graphql-rpc/config-examples/service.toml
cargo run --locked -p mgo-graphql-rpc --bin mgo-graphql-rpc -- \
  check-config --format server-yaml --path crates/mgo-graphql-rpc/config-examples/server.yaml
```

`check-config` uses the corresponding startup parser without opening database
or RPC connections. It exits with status 0 on success and status 1 for unreadable
files or parse errors. Diagnostics omit the file contents. The command checks
syntax and value types and follows the startup parser's handling of unknown keys.

Within `limits` and `internal_features`, omitted fields use their built-in
defaults. Explicit values override those defaults, including `false` for the
YAML example's `logger` flag.

### Launching the server

`start-server --config` accepts a TOML service configuration. Use flags such as
`--db-url`, `--host`, `--port`, and `--node-rpc-url` for connection settings.

```sh
cargo run --locked -p mgo-graphql-rpc --bin mgo-graphql-rpc -- \
  start-server --config crates/mgo-graphql-rpc/config-examples/service.toml
```

`from-config --path` accepts a YAML server configuration. It supports server
settings such as `connection` and `internal_features` alongside `service`.

```sh
cargo run --locked -p mgo-graphql-rpc --bin mgo-graphql-rpc -- \
  from-config --path crates/mgo-graphql-rpc/config-examples/server.yaml
```

This will build mgo-graphql-rpc and start an IDE:

```
Starting server...
Launch GraphiQL IDE at: http://127.0.0.1:8000
```

### Launching the server w/ indexer
For local dev, it might be useful to spin up an indexer as well.

## Compatibility with json-rpc

`cargo run --bin mgo-test-validator -- --with-indexer --pg-port 5432 --pg-db-name mgo_indexer_v2 --graphql-host 127.0.0.1 --graphql-port 9125`
