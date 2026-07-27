# rust-db-lib
A library for connecting to a database from within a Rust app. It encapsulates the specifics of the DB connection logic, exposing DB access through a consistent interface. The intention is that all Rust apps import this library as a dependency when they need access to the DB, so necessary changes to how our services interact with the DB can be managed from one place.

## Interface
The primary abstraction provided by this library is the `DataStore<T: DataVal, U: DataRow<T>>` trait. It exposes a predefined set of methods for interacting with a database and the results of querying the database. Details can be found in the rustdoc that accompanies the trait.

#### Supported implementations
The following implementations are provided for connecting to the DB.
- `postgres::PostgresDataVal` - Implements `DataVal`
- `postgres::PostgresDataRow` - Implements `DataRow<PostgresDataVal>`
- `postgres::PostgresDataStore` - Implements `DataStore<PostgresDataVal, PostgresDataRow>`

#### Connecting to Postgres
Construct a [`postgres::PostgresConfig`](src/postgres/mod.rs) and pass it to `PostgresDataStore::new()`. Only the connection fields are required; pool and TLS settings have sensible defaults.

```rust
use rust_db_lib::postgres::{PostgresConfig, PostgresDataStore, SslMode};

let config = PostgresConfig {
    host: "localhost".to_string(),
    port: 5432,
    username: "myuser".to_string(),
    password: "mypassword".to_string(),
    db_name: "mydb".to_string(),
    ..PostgresConfig::default()
};

let store = PostgresDataStore::new(config).await?;
```

`PostgresConfig::default()` provides the following values for unspecified fields:

| Field | Default |
|---|---|
| `port` | `5432` |
| `ssl_mode` | `SslMode::Require` |
| `max_connections` | `5` |
| `connection_timeout` | `10 seconds` |

## Features
The following features may be added when referencing this library in your `Cargo.toml`.

#### `testing-utils`
Example: `rust-db-lib = { version = "6", features = ["testing-utils"] }`

This feature enables the `testing_utils` module, which exposes structures that you may find useful when testing code that depends on this library.

## Docs
The Rust documentation and a getting-started guide can be found [here](https://doc.rust-lang.org/book/title-page.html).
