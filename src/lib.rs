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
pub trait DataVal: Send + Sync {
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
pub trait DataRow<T: DataVal>: Send + Sync {
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
#[derive(Clone, Debug)]
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

/// Abstraction for a data store capable of executing queries
pub trait DataStore<T: DataVal, U: DataRow<T>>: Clone + Send + Sync {
    /// Executes a SQL statement with no bound parameters.
    /// For queries with user input, use [`execute_parameterized_query`](Self::execute_parameterized_query).
    fn execute_query(
        &self,
        query: impl Into<Cow<'static, str>> + Send,
    ) -> impl Future<Output = Result<Vec<U>, DataStoreError>>;

    /// Executes a fully constructed parameterized query.
    /// Values for each of the parameters must have been bound prior to calling this method.
    fn execute_parameterized_query(
        &self,
        parameterized_query: ParameterizedQuery,
    ) -> impl Future<Output = Result<Vec<U>, DataStoreError>>;
}
