# rust-db-lib
A library for connecting to a database from a Rust app. It encapsulates DB connection logic and exposes access through a consistent interface, so changes to how services interact with the DB can be managed from one place.

## Interface
The primary abstraction is the `DataStore<T: DataVal, U: DataRow<T>>` trait. See the rustdoc for full details.

#### Supported implementations
- `postgres::PostgresDataVal` — implements `DataVal`
- `postgres::PostgresDataRow` — implements `DataRow<PostgresDataVal>`
- `postgres::PostgresDataStore` — implements `DataStore<PostgresDataVal, PostgresDataRow>`

#### Connecting to Postgres
Construct a [`postgres::PostgresConfig`](src/postgres/mod.rs) and pass it to `PostgresDataStore::new()`. Only the connection fields are required; pool and TLS settings have sensible defaults.

```rust
use rust_db_lib::postgres::{PostgresConfig, PostgresDataStore};

let config = PostgresConfig {
    host: "localhost".to_string(),
    username: "myuser".to_string(),
    password: "mypassword".to_string(),
    db_name: "mydb".to_string(),
    ..PostgresConfig::default()
};

let store = PostgresDataStore::new(config).await?;
```

`PostgresConfig::default()` values:

| Field | Default |
|---|---|
| `port` | `5432` |
| `ssl_mode` | `SslMode::Require` |
| `max_connections` | `5` |
| `connection_timeout` | `10 seconds` |

## Features

#### `testing-utils`
`rust-db-lib = { version = "6", features = ["testing-utils"] }`

Enables the `testing_utils` module with mock implementations useful for unit testing code that depends on this library.
