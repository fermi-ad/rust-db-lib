# Changelog

## v9.0.0

### Breaking changes

- Replaced generic type parameters with associated types across the data-store interfaces: `DataRow::Val` replaces `DataRow<T>`, while `DataStore::Row` and `DataStoreTransaction::Row` replace their row type parameters. Implementations and bounds must specify the appropriate associated types.
- `DataStore::Transaction` is now a non-lifetime-associated type instead of `Transaction<'a>`. `begin_transaction` returns an owned transaction that does not borrow the store; update custom backends accordingly.
- The `testing-utils` feature now provides a concrete `TestRow` backed by column-name-to-`TestVal` mappings. `TestDataStore` and `TestTransaction` are no longer generic over a custom row type; construct a `TestDataStore` from `Vec<TestRow>`.
- Removed `TestVal::new()` and `TestError`. Use `TestVal::default()` and `DataStoreError` instead. `TestVal::default()` now sets `is_nullable` to `false`; explicitly set it to `true` when testing nullable columns.

### Other changes

- PostgreSQL transactions now own their database transaction and share query execution and parameter-binding logic with the store.
- Cloned `TestDataStore` instances and their transactions share stored rows and captured operations.
- Moved the PostgreSQL and testing utility module implementations from `src/postgres/mod.rs` and `src/testing_utils/mod.rs` to `src/postgres.rs` and `src/testing_utils.rs`; public module paths are unchanged.
- Refreshed transitive dependencies in the lockfile.
