//! A Rust library providing abstractions for interacting with various data stores in a unified manner.
//! It defines traits for data values, data rows, parameterized queries, and data stores,
//! along with a Postgres implementation and test utilities.

use chrono::{DateTime, Utc};
use std::{
    borrow::Cow,
    convert::Infallible,
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

/// Failure opening, executing, or completing a transaction.
#[derive(Clone, Debug)]
pub enum TransactionError<E = Infallible> {
    /// The transaction conflicted with concurrent work and may succeed if retried.
    Retryable,
    /// The transaction closure rejected the operation.
    OperationError(E),
    /// Any other transaction failure.
    DatabaseError(DataStoreError),
}

impl<E: Display> Display for TransactionError<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Retryable => write!(f, "transaction conflict (retryable)"),
            Self::OperationError(error) => Display::fmt(error, f),
            Self::DatabaseError(error) => Display::fmt(error, f),
        }
    }
}
impl<E: Display + fmt::Debug> Error for TransactionError<E> {}

/// Operations available to a closure while it owns a transaction borrow.
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
}

/// A boxed asynchronous transaction closure.
pub type TransactionFuture<'a, R, E = Infallible> =
    std::pin::Pin<Box<dyn Future<Output = Result<R, E>> + Send + 'a>>;

/// Abstraction for a data store capable of executing queries.
pub trait DataStore<T: DataVal, U: DataRow<T>>: Clone + Send + Sync + 'static {
    /// The backend-specific transaction context.
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

    /// Executes `operation` within a transaction.
    ///
    /// If `operation` returns `Ok`, the transaction is committed and the value returned.
    /// If `operation` returns `Err`, the transaction is rolled back and the error is wrapped
    /// in [`TransactionError::OperationError`]. Backend failures opening, committing, or rolling
    /// back the transaction are classified per [`TransactionError`], including
    /// [`TransactionError::Retryable`] for conflicts that may succeed if `operation` — including
    /// any reads it performed — is retried in full.
    ///
    /// # Examples
    /// ```rust,ignore
    /// let applied = store
    ///     .with_transaction(|tx| {
    ///         Box::pin(async move {
    ///             let readings = tx
    ///                 .execute_query("SELECT temperature, battery_percent FROM device_readings WHERE device_id = 42")
    ///                 .await?;
    ///             if readings.is_empty() {
    ///                 // Automatically causes the transaction to roll back.
    ///                 return Err("device reading was not found".to_string());
    ///             }
    ///
    ///             // Execute update queries within the same transaction. If any fail, the transaction is rolled back.
    ///             tx.execute_query("UPDATE device_settings SET fan_mode = 'cool' WHERE device_id = 42")
    ///                 .await?;
    ///             tx.execute_query("UPDATE device_settings SET low_battery_alert = true WHERE device_id = 42")
    ///                 .await?;
    ///
    ///             // The transaction is automatically committed on success.
    ///             Ok(readings.len())
    ///         })
    ///     })
    ///     .await;
    /// ```
    fn with_transaction<'store, R, E, F>(
        &'store self,
        operation: F,
    ) -> impl Future<Output = Result<R, TransactionError<E>>> + Send + 'store
    where
        R: Send + 'store,
        E: Send + 'store,
        F: for<'tx> FnOnce(&'tx mut Self::Transaction<'store>) -> TransactionFuture<'tx, R, E>
            + Send
            + 'store;

    /// Executes `operation` in a transaction serialized against other operations
    /// using the same named logical resource.
    fn with_serialized_transaction<'store, R, E, F>(
        &'store self,
        resource_name: Cow<'static, str>,
        operation: F,
    ) -> impl Future<Output = Result<R, TransactionError<E>>> + Send + 'store
    where
        R: Send + 'store,
        E: Send + 'store,
        F: for<'tx> FnOnce(&'tx mut Self::Transaction<'store>) -> TransactionFuture<'tx, R, E>
            + Send
            + 'store;

    /// Executes a batch of parameterized queries in a single transaction.
    fn execute_transaction(
        &self,
        queries: Vec<ParameterizedQuery>,
    ) -> impl Future<Output = Result<(), DataStoreError>> + Send {
        async move {
            self.with_transaction::<(), DataStoreError, _>(|transaction| {
                Box::pin(async move {
                    for query in queries {
                        transaction
                            .execute_parameterized_query(query)
                            .await
                            .map_err(transaction_error_to_datastore)?;
                    }
                    Ok(())
                })
            })
            .await
            .map_err(transaction_error_to_datastore)
        }
    }
}

fn transaction_error_to_datastore<E: Display>(error: TransactionError<E>) -> DataStoreError {
    match error {
        TransactionError::Retryable => DataStoreError {
            details: "transaction conflict (retryable)".to_string(),
        },
        TransactionError::OperationError(error) => DataStoreError {
            details: format!("transaction operation failed: {}", error),
        },
        TransactionError::DatabaseError(error) => error,
    }
}
