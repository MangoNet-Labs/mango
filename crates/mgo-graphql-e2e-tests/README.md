End-to-end tests for GraphQL service, built on top of the transactional test
runner.

# Local Set-up

These tests require a running instance of the `postgres` service, with a
database set-up.  The instructions below assume that `postgres` has been
installed using `brew`:

1. See the instructions in the Mango Indexer [indexer](../mgo-indexer/indexer.md)
   for pre-requisites and starting the Postgres service.

2. When postgres is initially installed, it creates a role for your current
   user.  We need to use that role to create the role that will access the
   database:

```sh
$ psql "postgres://postgres:postgres@localhost:5432/postgres" \
    -c "CREATE ROLE postgres WITH SUPERUSER LOGIN PASSWORD 'postgres';"
```

3. Then, create the database that the tests expect, using the `postgres` user and increase the max connections since many tests might run in parallel.

```sh
$ psql "postgres://postgres:postgres@localhost:5432/postgres" \
    -c "CREATE DATABASE mgo_indexer_v2; -c 'ALTER SYSTEM SET max_connections = 600;"
```

4. Finally, restart the `postgres` server so the max connections change takes effect.

Mac
```sh
brew services restart postgres

```

Linux
```sh
sudo systemctl restart postgresql
```