# rust-db-lib

Check out the [latest documentation](https://fermi-ad.github.io/rust-db-lib/rust_db_lib/)

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

#### Transactions

For a simple write-only batch, use [`DataStore::execute_transaction()`](src/lib.rs:286). Pass it a vector of [`ParameterizedQuery`](src/lib.rs:122) values; it runs them in order and commits them only if the entire batch succeeds.

```rust,ignore
let queries = vec![
    ParameterizedQuery::new("UPDATE device_settings SET fan_mode = 'cool' WHERE device_id = 42"),
    ParameterizedQuery::new("INSERT INTO device_setting_events (device_id, kind) VALUES (42, 'updated')"),
];
store.execute_transaction(queries).await?;
```

For transactions that need to read data before performing related writes, use [`DataStore::with_transaction()`](src/lib.rs:238). The closure receives a transaction-scoped query interface and runs entirely within one transaction. Returning `Ok` commits every mutation; returning an error rolls back the entire operation.

```rust,ignore
let result = store
    .with_transaction(|tx| {
        Box::pin(async move {
            let readings = tx
                .execute_query("SELECT temperature FROM device_readings WHERE device_id = 42")
                .await?;
            if readings.is_empty() {
                // Automatically causes the transaction to roll back.
                return Err("device reading was not found".to_string());
            }

            // Execute mutation queries within the same transaction. If any fail, the transaction is rolled back.
            tx.execute_query("UPDATE device_settings SET fan_mode = 'cool' WHERE device_id = 42")
                .await?;
            tx.execute_query("INSERT INTO device_setting_events (device_id, kind) VALUES (42, 'updated')")
                .await?;

            // The transaction is automatically committed on success.
            Ok(())
        })
    })
    .await;
```

To serialize transactions operating on the same named logical resource, use [`DataStore::with_serialized_transaction()`](src/lib.rs:282) and provide the same `resource_name` for each cooperating caller. If concurrent work causes a transaction conflict, the operation returns [`TransactionError::Retryable`](src/lib.rs:176). The library does not retry automatically, so retry the complete closure, including all of its reads.

```rust,ignore
let result = store.with_serialized_transaction("device:42".into(), |tx| {
    Box::pin(async move {

        // ... read and write operations ...

        Ok(())
    })
})
.await;
```

#### `testing-utils`
`rust-db-lib = { git = "https://github.com/fermi-ad/rust-db-lib", tag = "vX.Y.Z", features = ["testing-utils"] }`

Enables the `testing_utils` module with mock implementations useful for unit testing code that depends on this library.
