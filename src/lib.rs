//! A Rust library providing abstractions for interacting with various data stores in a unified manner.
//! It defines traits for data values, data rows, parameterized queries, and data stores,
//! along with a Postgres implementation and test utilities.

use chrono::{DateTime, Utc};
use std::{
    borrow::Cow,
    error::Error,
    fmt::{self, Display, Formatter},
};

/// Postgres implementation of the traits in this library.
pub mod postgres;

/// A collection of prebuilt implementations of the traits in this library that are useful for unit tests.
#[cfg(any(feature = "testing-utils", test))]
pub mod testing_utils;

#[cfg(test)]
mod tests;

/// Custom error type for [`DataStore`] operations
#[derive(Clone, Debug)]
pub struct DataStoreError {
    details: String,
}
impl Display for DataStoreError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "DataStoreError: {}", self.details)
    }
}
impl Error for DataStoreError {}

/// Represents the value stored in a database column. In this intermediate state,
/// the exact type of the data is unknown. Calling one of the trait methods will attempt to decode
/// the value as the desired type. An error will be returned if the column does not exist or the
/// data cannot be decoded as the requested type.
pub trait DataVal: Send + Sync + 'static {
    /// For nonnull DB columns. Attempts to decode the value as a [`bool`].
    fn to_bool(self) -> Result<bool, DataStoreError>;

    /// For nullable DB columns. Attempts to decode the value as a [`Option<bool>`].
    fn to_bool_optional(self) -> Result<Option<bool>, DataStoreError>;

    /// For nonnull DB columns. Attempts to decode the value as a [`DateTime<Utc>`].
    fn to_datetime(self) -> Result<DateTime<Utc>, DataStoreError>;

    /// For nullable DB columns. Attempts to decode the value as a [`Option<DateTime<Utc>>`].
    fn to_datetime_optional(self) -> Result<Option<DateTime<Utc>>, DataStoreError>;

    /// For nonnull DB columns. Attempts to decode the value as a [`i8`].
    fn to_i8(self) -> Result<i8, DataStoreError>;

    /// For nullable DB columns. Attempts to decode the value as a [`Option<i8>`].
    fn to_i8_optional(self) -> Result<Option<i8>, DataStoreError>;

    /// For nonnull DB columns. Attempts to decode the value as a [`i16`].
    fn to_i16(self) -> Result<i16, DataStoreError>;

    /// For nullable DB columns. Attempts to decode the value as a [`Option<i16>`].
    fn to_i16_optional(self) -> Result<Option<i16>, DataStoreError>;

    /// For nonnull DB columns. Attempts to decode the value as a [`i32`].
    fn to_i32(self) -> Result<i32, DataStoreError>;

    /// For nullable DB columns. Attempts to decode the value as a [`Option<i32>`].
    fn to_i32_optional(self) -> Result<Option<i32>, DataStoreError>;

    /// For nonnull DB columns. Attempts to decode the value as a [`i64`].
    fn to_i64(self) -> Result<i64, DataStoreError>;

    /// For nullable DB columns. Attempts to decode the value as a [`Option<i64>`].
    fn to_i64_optional(self) -> Result<Option<i64>, DataStoreError>;

    /// For nonnull DB columns. Attempts to decode the value as a [`f32`].
    fn to_f32(self) -> Result<f32, DataStoreError>;

    /// For nullable DB columns. Attempts to decode the value as a [`Option<f32>`].
    fn to_f32_optional(self) -> Result<Option<f32>, DataStoreError>;

    /// For nonnull DB columns. Attempts to decode the value as a [`f64`].
    fn to_f64(self) -> Result<f64, DataStoreError>;

    /// For nullable DB columns. Attempts to decode the value as a [`Option<f64>`].
    fn to_f64_optional(self) -> Result<Option<f64>, DataStoreError>;

    /// For nonnull DB columns. Attempts to decode the value as a [`String`].
    fn to_string(self) -> Result<String, DataStoreError>;

    /// For nullable DB columns. Attempts to decode the value as a [`Option<String>`].
    fn to_string_optional(self) -> Result<Option<String>, DataStoreError>;
}

/// Abstraction representing a single row retrieved from a data store
pub trait DataRow<T: DataVal>: Send + Sync + 'static {
    /// Generates an instance of [`DataVal`] wrapping the contents of the specified column
    fn get(&self, column_name: &str) -> T;
}

/// Represents a single parameter to be bound to a parameterized query.
#[derive(Clone, Debug, PartialEq)]
pub enum QueryParameter {
    Bool(bool),
    DateTime(DateTime<Utc>),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
    Str(String),
}

/// A parameterized SQL query with its bound values.
///
/// The statement must use sequential placeholders (`$1`, `$2`, …) matching the order of [`bind`](Self::bind) calls.
/// Any bindings beyond the number of placeholders in the statement are ignored.
///
/// The statement accepts either a `&'static str` (for SQL literals) or an owned [`String`]
/// (for dynamically constructed queries); both convert via `.into()`.
#[derive(Clone, Debug, PartialEq)]
pub struct ParameterizedQuery {
    /// The SQL statement with sequential `$N` placeholders.
    pub statement: Cow<'static, str>,
    /// The values to bind, in placeholder order.
    pub bindings: Vec<QueryParameter>,
}
impl ParameterizedQuery {
    /// Creates a [`ParameterizedQuery`] from a SQL statement.
    ///
    /// Accepts a `&'static str` literal or an owned [`String`] for dynamically built statements.
    ///
    /// # Examples
    /// ```rust
    /// use rust_db_lib::ParameterizedQuery;
    ///
    /// // Static literal
    /// let q = ParameterizedQuery::new("SELECT * FROM users WHERE active = $1");
    ///
    /// // Dynamically built
    /// let ids = vec![1i32, 2, 3];
    /// let placeholders: String = (1..=ids.len())
    ///     .map(|i| format!("${i}"))
    ///     .collect::<Vec<_>>()
    ///     .join(", ");
    /// let q = ParameterizedQuery::new(
    ///     format!("SELECT * FROM users WHERE id IN ({placeholders})")
    /// );
    /// ```
    pub fn new(query_statement: impl Into<Cow<'static, str>>) -> Self {
        Self {
            statement: query_statement.into(),
            bindings: Vec::new(),
        }
    }

    /// Binds a [`QueryParameter`] to the query in placeholder order.
    pub fn bind(&mut self, parameter: QueryParameter) {
        self.bindings.push(parameter);
    }
}

/// Failure opening or completing a transaction.
#[derive(Clone, Debug)]
pub enum TransactionError {
    /// The transaction conflicted with concurrent work and may succeed if retried.
    Retryable,
    /// Any other transaction failure.
    DatabaseError(DataStoreError),
}
impl Display for TransactionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Retryable => write!(f, "transaction conflict (retryable)"),
            Self::DatabaseError(error) => Display::fmt(error, f),
        }
    }
}
impl Error for TransactionError {}

/// Operations available on an open transaction.
pub trait DataStoreTransaction<T: DataVal, U: DataRow<T>>: Send {
    /// Executes an unparameterized query within this transaction.
    fn execute_query(
        &mut self,
        query: impl Into<Cow<'static, str>> + Send,
    ) -> impl Future<Output = Result<Vec<U>, TransactionError>> + Send;

    /// Executes a parameterized query within this transaction.
    fn execute_parameterized_query(
        &mut self,
        parameterized_query: ParameterizedQuery,
    ) -> impl Future<Output = Result<Vec<U>, TransactionError>> + Send;

    /// Call [`Self::commit`] to persist the transaction or [`Self::rollback`] to discard it.
    /// An open transaction that is dropped without being committed or rolled back is rolled back
    /// automatically by the backend.
    ///
    /// Commits this transaction and persists all changes made within it.
    fn commit(self) -> impl Future<Output = Result<(), TransactionError>> + Send;

    /// Rolls back this transaction and discards all changes made within it.
    fn rollback(self) -> impl Future<Output = Result<(), TransactionError>> + Send;
}

/// Abstraction for a data store capable of executing queries.
pub trait DataStore<T: DataVal, U: DataRow<T>>: Clone + Send + Sync + 'static {
    /// The backend-specific handle for an open transaction.
    type Transaction<'a>: DataStoreTransaction<T, U>
    where
        Self: 'a;

    /// Executes a SQL statement with no bound parameters.
    fn execute_query(
        &self,
        query: impl Into<Cow<'static, str>> + Send,
    ) -> impl Future<Output = Result<Vec<U>, DataStoreError>> + Send;

    /// Executes a fully constructed parameterized query.
    fn execute_parameterized_query(
        &self,
        parameterized_query: ParameterizedQuery,
    ) -> impl Future<Output = Result<Vec<U>, DataStoreError>> + Send;

    /// Executes a batch of parameterized queries in a single transaction.
    ///
    /// This is the simplest transaction API for write-only batches. Queries run in order; the
    /// transaction is committed only when every query succeeds, and is rolled back if a query
    /// fails.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use rust_db_lib::{DataStore, ParameterizedQuery};
    /// # async fn example<S, T, U>(store: S) -> Result<(), rust_db_lib::DataStoreError>
    /// # where
    /// #     S: DataStore<T, U>,
    /// #     T: rust_db_lib::DataVal,
    /// #     U: rust_db_lib::DataRow<T>,
    /// # {
    /// let mut update = ParameterizedQuery::new("UPDATE devices SET enabled = $1 WHERE id = $2");
    /// update.bind(rust_db_lib::QueryParameter::Bool(true));
    /// update.bind(rust_db_lib::QueryParameter::I64(42));
    /// store.execute_transaction(vec![update]).await?;
    /// # Ok(())
    /// # }
    /// ```
    fn execute_transaction(
        &self,
        queries: Vec<ParameterizedQuery>,
    ) -> impl Future<Output = Result<(), DataStoreError>> + Send {
        async move {
            let mut transaction = self
                .begin_transaction()
                .await
                .map_err(transaction_error_to_datastore)?;
            for query in queries {
                if let Err(error) = transaction.execute_parameterized_query(query).await {
                    let _ = transaction.rollback().await;
                    return Err(transaction_error_to_datastore(error));
                }
            }
            transaction
                .commit()
                .await
                .map_err(transaction_error_to_datastore)
        }
    }

    /// Opens a transaction.
    ///
    /// Use this method for multi-step work that requires reads and writes to occur in one transaction.
    ///
    /// Dropping the returned transaction without calling [`DataStoreTransaction::commit`] or
    /// [`DataStoreTransaction::rollback`] rolls it back automatically.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use rust_db_lib::{DataStore, DataStoreTransaction};
    /// # async fn example<S, T, U>(store: S) -> Result<(), rust_db_lib::TransactionError>
    /// # where
    /// #     S: DataStore<T, U>,
    /// #     T: rust_db_lib::DataVal,
    /// #     U: rust_db_lib::DataRow<T>,
    /// # {
    /// let mut transaction = store.begin_transaction().await?;
    /// let rows = transaction.execute_query("SELECT id FROM devices WHERE id = 42").await?;
    /// if rows.is_empty() {
    ///     transaction.rollback().await?;
    /// } else {
    ///     transaction.execute_query("UPDATE devices SET enabled = TRUE WHERE id = 42").await?;
    ///     transaction.commit().await?;
    /// }
    /// # Ok(())
    /// # }
    /// ```
    fn begin_transaction(
        &self,
    ) -> impl Future<Output = Result<Self::Transaction<'_>, TransactionError>> + Send;

    /// Opens a transaction serialized with transactions for the named resource.
    ///
    /// Transactions started with the same resource name are coordinated by the backend. Dropping
    /// the returned transaction without calling [`DataStoreTransaction::commit`] or
    /// [`DataStoreTransaction::rollback`] rolls it back automatically.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use rust_db_lib::{DataStore, DataStoreTransaction};
    /// # async fn example<S, T, U>(store: S) -> Result<(), rust_db_lib::TransactionError>
    /// # where
    /// #     S: DataStore<T, U>,
    /// #     T: rust_db_lib::DataVal,
    /// #     U: rust_db_lib::DataRow<T>,
    /// # {
    /// let mut transaction = store.begin_serialized_transaction("device:42").await?;
    /// transaction.execute_query("UPDATE devices SET enabled = TRUE WHERE id = 42").await?;
    /// transaction.commit().await?;
    /// # Ok(())
    /// # }
    /// ```
    fn begin_serialized_transaction(
        &self,
        resource_name: impl Into<Cow<'static, str>> + Send,
    ) -> impl Future<Output = Result<Self::Transaction<'_>, TransactionError>> + Send;
}

fn transaction_error_to_datastore(error: TransactionError) -> DataStoreError {
    match error {
        TransactionError::DatabaseError(error) => error,
        TransactionError::Retryable => DataStoreError {
            details: "transaction conflict (retryable)".to_string(),
        },
    }
}
