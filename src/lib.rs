//! A Rust library providing abstractions for interacting with various data stores in a unified manner.
//! It defines traits for data values, data rows, parameterized queries, and data stores,
//! along with a Postgres implementation and test utilities.

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

/// Postgres implementation of the traits in this library.
pub mod postgres;

/// A collection of prebuilt implementations of the traits in this library that are useful for unit tests.
pub mod test_utils;

/// Custom error type for [`DataStore`] operations
#[derive(Debug)]
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
    /// Attempts to decode the value as a [`bool`].
    fn to_bool(self) -> Result<bool, DataStoreError>;

    /// Attempts to decode the value as a [`DateTime<Utc>`].
    fn to_datetime(self) -> Result<DateTime<Utc>, DataStoreError>;

    /// Attempts to decode the value as a [`i8`].
    fn to_i8(self) -> Result<i8, DataStoreError>;

    /// Attempts to decode the value as a [`i16`].
    fn to_i16(self) -> Result<i16, DataStoreError>;

    /// Attempts to decode the value as a [`i32`].
    fn to_i32(self) -> Result<i32, DataStoreError>;

    /// Attempts to decode the value as a [`i64`].
    fn to_i64(self) -> Result<i64, DataStoreError>;

    /// Attempts to decode the value as a [`f32`].
    fn to_f32(self) -> Result<f32, DataStoreError>;

    /// Attempts to decode the value as a [`f64`].
    fn to_f64(self) -> Result<f64, DataStoreError>;

    /// Attempts to decode the value as a [`String`].
    fn to_string(self) -> Result<String, DataStoreError>;
}

/// Abstraction representing a single row retrieved from a data store
pub trait DataRow<T: DataVal>: Send + Sync {
    /// Generates an instance of [`DataVal`] wrapping the contents of the specified column
    fn get(&self, column_name: &str) -> T;
}

/// Represents a single parameter to be bound to a parameterized query.
#[derive(Clone, Debug, PartialEq)]
pub enum QueryParameter {
    BOOL(bool),
    DATETIME(DateTime<Utc>),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
    STR(String),
}

/// Abstraction representing a parameterized query. Exposes a method for binding parameters to the query statement.
///
/// It is expected that the query string will have sequential placeholders for the parameterized data.
/// Example: If passing 5 elements into the query. the query should contain `$1`, `$2`, `$3`, `$4`, and `$5`, and
/// the bindings should have no fewer than 5 elements (any additional elements beyond 5 will be ignored).
#[derive(Clone, Debug)]
pub struct ParameterizedQuery {
    /// The SQL query statement with placeholders for parameterized data
    pub statement: String,
    /// The list of [`QueryParameter`]s to bind to the query statement
    pub bindings: Vec<QueryParameter>,
}
impl ParameterizedQuery {
    /// Initializes a [`ParameterizedQuery`] with the provided query statement.
    ///
    /// It is expected that the query statement will have sequential placeholders for the parameterized data.
    /// Example: If passing 5 elements into the query, the query should contain `$1`, `$2`, `$3`, `$4`, and `$5`, and
    /// the bindings should have no fewer than 5 elements (any additional elements beyond 5 will be ignored).
    pub fn new(query_statement: String) -> Self {
        Self {
            statement: query_statement,
            bindings: Vec::new(),
        }
    }

    /// Binds a [`QueryParameter`] to the query. Must be called in the order the parameters appear in the query string.
    pub fn bind(&mut self, parameter: QueryParameter) {
        self.bindings.push(parameter);
    }
}

/// Abstraction for a data store capable of executing queries
#[async_trait]
pub trait DataStore<T: DataVal, U: DataRow<T>>: Clone + Send + Sync {
    /// Executes a basic SQL statement. If any user input is required, use [`init_parameterized_query`](Self::init_parameterized_query)
    async fn execute_query(&self, query: &str) -> Result<Vec<U>, DataStoreError>;

    /// Executes a fully constructed parameterized query.
    /// Values for each of the parameters must have been bound prior to calling this method.
    async fn execute_parameterized_query(
        &self,
        parameterized_query: ParameterizedQuery,
    ) -> Result<Vec<U>, DataStoreError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_datastore_error() {
        let error = DataStoreError {
            details: "Test error".to_string(),
        };
        assert_eq!(format!("{}", error), "DataStoreError: Test error");
    }

    #[test]
    fn test_parameterized_query_new_and_bind() {
        let mut param_query =
            ParameterizedQuery::new("SELECT * FROM table WHERE id = $1".to_string());
        assert_eq!(param_query.statement, "SELECT * FROM table WHERE id = $1");
        assert!(param_query.bindings.is_empty());

        let test_param = QueryParameter::I32(42);
        param_query.bind(test_param.clone());
        assert_eq!(param_query.bindings.len(), 1);
        assert_eq!(param_query.bindings[0], test_param);
    }
}
