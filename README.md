# rust-db-lib

[Latest API documentation](https://fermi-ad.github.io/rust-db-lib/rust_db_lib/)

A Rust library with a common interface for executing queries against a data store. It includes a PostgreSQL implementation and optional utilities for testing code that depends on a data store.

## Interface

The primary abstraction is the `DataStore` trait. `DataRow` and `DataVal` provide access to returned columns; `ParameterizedQuery` and `QueryParameter` represent SQL with bound values. Use `DataStore::execute_parameterized_query` when a query contains user input. For multi-statement work, use `DataStore::execute_transaction` for a batch or `DataStore::begin_transaction` for an explicit transaction. See the API documentation for details.

### PostgreSQL

The `postgres` module provides `PostgresDataStore`, `PostgresTransaction`, `PostgresDataRow`, and `PostgresDataVal` implementations of the corresponding traits. Create a connection pool with `PostgresDataStore::new`:

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

Unspecified configuration fields use these defaults:

| Field | Default |
|---|---|
| `port` | `5432` |
| `ssl_mode` | `SslMode::Require` |
| `max_connections` | `5` |
| `connection_timeout` | 10 seconds |

### Testing utilities (optional)

Enable the `testing-utils` feature in the consuming crate:

```toml
[dev-dependencies]
rust-db-lib = { git = "https://github.com/fermi-ad/rust-db-lib", tag = "vX.Y.Z", features = ["testing-utils"] }
```

The `testing_utils` module provides `TestDataStore`, `TestRow`, `TestVal`, `TestTransaction`, and `Operation`. For concise fixtures, import the `test_data_store!` macro from that module:

```rust
use rust_db_lib::{DataRow, DataStore, DataVal, testing_utils::test_data_store};

let store = test_data_store!([
    [("name", "Ada"), ("age", 42), ("active", true), ("nickname", None)],
    [("name", String::from("Grace")), ("age", 43_i64), ("active", false)],
]);

let rows = store.execute_query("SELECT * FROM people").await?;
assert_eq!(rows[0].get("name").to_string()?, "Ada");
assert_eq!(rows[0].get("nickname").to_string_optional()?, None);
assert!(rows[1].get("nickname").to_string_optional().is_err());
```

Each bracketed list becomes a separate row. Supported values are `bool`, `i8`, `i16`, `i32`, `i64`, `f32`, `f64`, `String`, `&str`, and `chrono::DateTime` (converted to UTC). Unsuffixed integers such as `42` use `i32`; suffix values such as `43_i64` to select another type. An explicit `None` makes a null column: optional decoders return `None`, while non-optional decoders return an error. Omitting the column altogether makes both optional and non-optional reads return an error. Value types must match the decoder used by the code under test.

`TestDataStore` returns the same configured rows for **every** query, including queries within transactions; it does not execute SQL or filter results. Check `captured_operations()` to assert on query text, bindings, transaction boundaries, and rollbacks instead of inferring query correctness from returned rows. You can also construct rows directly using `TestRow::new` and `TestVal` for fixtures that need manual configuration.

This repository uses a self dev-dependency to run the feature-enabled integration tests with plain `cargo test`; `testing-utils` is **not** a default feature for downstream users.
