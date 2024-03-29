Mango indexer is an off-fullnode service to serve data from Mango protocol, including both data directly generated from chain and derivative data.

## Steps to run locally
### Prerequisites
- install local [Postgres server](https://www.postgresql.org/download/). You can also `brew install postgresql@version` and then add the following to your `~/.zshrc` or `~/.zprofile`, etc:
```sh
export LDFLAGS="-L/opt/homebrew/opt/postgresql@15/lib"
export CPPFLAGS="-I/opt/homebrew/opt/postgresql@15/include"
export PATH="/opt/homebrew/opt/postgresql@15/bin:$PATH"
```
- make sure you have libpq installed: `brew install libpq`, and in your profile, add `export PATH="/opt/homebrew/opt/libpq/bin:$PATH"`. If this doesn't work, try `brew link --force libpq`.

- install Diesel CLI with `cargo install diesel_cli --no-default-features --features postgres`, refer to [Diesel Getting Started guide](https://diesel.rs/guides/getting-started) for more details
- [optional but handy] Postgres client like [Postico](https://eggerapps.at/postico2/), for local check, query execution etc.

### Start the Postgres Service

Postgres must run as a service in the background for other tools to communicate with.  If it was installed using homebrew, it can be started as a service with:

``` sh
brew services start postgresql@version
```

### Running standalone indexer
1. DB setup, under `mgo/crates/mgo-indexer` run:
```sh
# an example DATABASE_URL is "postgres://postgres:postgres@localhost/gegao"
diesel setup --database-url="<DATABASE_URL>"
diesel database reset --database-url="<DATABASE_URL>"
```
Note that you'll need an existing database for the above to work. Replace `gegao` with the name of the database created.

2. Checkout to your target branch

For example, if you want to be on the DevNet branch
```sh
git fetch upstream devnet && git reset --hard upstream/devnet
```
3. Start indexer binary, under `mgo/crates/mgo-indexer` run:
- run indexer as a writer, which pulls data from fullnode and writes data to DB
```sh
# Change the RPC_CLIENT_URL to http://0.0.0.0:9000 to run indexer against local validator & fullnode
mgo-indexer --db-url "<DATABASE_URL>" --rpc-client-url "xxxxxx" --client-metric-port 19285 --fullnode-sync-worker --reset-db

Usage: mgo-indexer [OPTIONS]

Options:
      --db-url <DB_URL>                          
      --db-user-name <DB_USER_NAME>              
      --db-password <DB_PASSWORD>                
      --db-host <DB_HOST>                        
      --db-port <DB_PORT>                        
      --db-name <DB_NAME>                        
      --rpc-client-url <RPC_CLIENT_URL>          [default: http://0.0.0.0:9000]
      --client-metric-host <CLIENT_METRIC_HOST>  [default: 0.0.0.0]
      --client-metric-port <CLIENT_METRIC_PORT>  [default: 9184]
      --rpc-server-url <RPC_SERVER_URL>          [default: 0.0.0.0]
      --rpc-server-port <RPC_SERVER_PORT>        [default: 9000]
      --migrated-methods <MIGRATED_METHODS>...   
      --reset-db                                 
      --fullnode-sync-worker                     
      --rpc-server-worker                        
      --analytical-worker                        
      --skip-db-commit                           
      --use-v2                                   
  -h, --help                                     Print help
```
- run indexer as a reader, which is a JSON RPC server with the interface
```
mgo-indexer --db-url "<DATABASE_URL>" --rpc-client-url "xxxxxx" --client-metric-port 19286 --rpc-server-worker
```
More flags info can be found in this file
### DB reset
Run this command under `mgo/crates/mgo-indexer`, which will wipe DB; In case of schema changes in `.sql` files, this will also update corresponding `schema.rs` file.
```sh
diesel database reset --database-url="<DATABASE_URL>"
```

