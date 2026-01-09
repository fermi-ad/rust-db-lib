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

/// Abstraction representing a parameterized query. Exposes a method for binding parameters to the query statement.
///
/// It is expected that the query string will have sequential placeholders for the paramterized data.
/// Example: If passing 5 elements into the query. the query should contain `$1`, `$2`, `$3`, `$4`, and `$5`, and
/// the bindings should have no fewer than 5 elements (any additional elements beyond 5 will be ignored).
#[async_trait]
pub trait ParameterizedQuery<'a, T: DataVal, U: DataRow<T>>: Send + Sync {
    /// Binds a [`bool`] to the query.
    fn bind_bool(&'a mut self, parameter: bool) -> Result<(), DataStoreError>;

    /// Binds a [`DateTime<Utc>`] to the query.
    fn bind_datetime(&'a mut self, parameter: DateTime<Utc>) -> Result<(), DataStoreError>;

    /// Binds a [`i8`] to the query.
    fn bind_i8(&'a mut self, parameter: i8) -> Result<(), DataStoreError>;

    /// Binds a [`i16`] to the query.
    fn bind_i16(&'a mut self, parameter: i16) -> Result<(), DataStoreError>;

    /// Binds a [`i32`] to the query.
    fn bind_i32(&'a mut self, parameter: i32) -> Result<(), DataStoreError>;

    /// Binds a [`i64`] to the query.
    fn bind_i64(&'a mut self, parameter: i64) -> Result<(), DataStoreError>;

    /// Binds a [`f32`] to the query.
    fn bind_f32(&'a mut self, parameter: f32) -> Result<(), DataStoreError>;

    /// Binds a [`f64`] to the query.
    fn bind_f64(&'a mut self, parameter: f64) -> Result<(), DataStoreError>;

    /// Binds a String to the query.
    fn bind_string(&'a mut self, parameter: String) -> Result<(), DataStoreError>;

    /// Executes the parameterized query and returns the resulting rows.
    async fn execute(self) -> Result<Vec<U>, DataStoreError>;
}

/// Abstraction representing a single row retrieved from a data store
pub trait DataRow<T: DataVal>: Send + Sync {
    /// Generates an instance of [`DataVal`] wrapping the contents of the specified column
    fn get(&self, column_name: &str) -> T;
}

/// Abstraction for a data store capable of executing queries
#[async_trait]
pub trait DataStore<'a, T: DataVal, U: DataRow<T>>: Clone + Send + Sync {
    type ParamQueryImpl: ParameterizedQuery<'a, T, U>;
    /// Executes a basic SQL statement. If any user input is required, use [`init_parameterized_query`](Self::init_parameterized_query)
    async fn execute_query(&self, query: &str) -> Result<Vec<U>, DataStoreError>;

    /// Initializes a [`ParameterizedQuery`] with the provided query statement.
    ///
    /// Note: It is expected that the query statement will have sequential placeholders for the paramterized data.
    /// Example: If passing 5 elements into the query, the query should contain `$1`, `$2`, `$3`, `$4`, and `$5`, and
    /// the bindings should have no fewer than 5 elements (any additional elements beyond 5 will be ignored).
    fn init_parameterized_query(&'a self, query: &'a str) -> Self::ParamQueryImpl;
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
}
