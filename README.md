# rust-db-lib

Check out the [latest documentation](https://fermi-ad.github.io/rust-db-lib/rust_db_lib/)

A library for connecting to a database from a Rust app. It encapsulates DB connection logic and exposes access through a consistent interface, so changes to how services interact with the DB can be managed from one place.


## Interface
The primary abstraction is the `DataStore<T: DataVal, U: DataRow<T>>` trait. See the rustdoc for full details.

Transactions are opened with `begin_transaction(TransactionPolicy)`, returning a backend-specific handle that supports scoped reads and writes followed by `commit` or `rollback`. `TransactionPolicy::SerializeOn` requests serialization for a named logical resource; PostgreSQL implements this with serializable isolation and a transaction-scoped advisory lock. Backends that cannot honor a policy must return `TransactionError::UnsupportedPolicy` rather than silently weakening the guarantee.

Transaction conflicts are returned as `TransactionError::Retryable`. The library does not retry automatically; callers should rerun the complete transaction, including all reads, when handling this classification. The existing `execute_transaction(Vec<ParameterizedQuery>)` convenience method remains available and uses the default policy.

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
`rust-db-lib = { git = "https://github.com/fermi-ad/rust-db-lib", tag = "vX.Y.Z", features = ["testing-utils"] }`

Enables the `testing_utils` module with mock implementations useful for unit testing code that depends on this library.
