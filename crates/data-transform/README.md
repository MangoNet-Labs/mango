
data-transform is a separate process used to transform data, by decoding columns into other formats in preparation to upload to Snowflake.

### Running standalone transformer

1. in mango/crates/data-transform:
```sh
$ echo DATABASE_URL=postgres://username:password@localhost/diesel_transform_demo > .env
```