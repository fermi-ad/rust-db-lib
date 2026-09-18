//! Rust DB Lib Testing Utilities

use super::{DataRow, DataStore, DataStoreError, DataVal, ParameterizedQuery};
use chrono::{DateTime, Utc};
use std::{
    borrow::Cow,
    error::Error,
    fmt::{self, Display, Formatter},
    sync::Mutex,
};

#[cfg(test)]
mod tests;

/// A default implementation of [`std::error::Error`] for use in test cases.
#[derive(Debug)]
pub struct TestError;
impl Display for TestError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "TestError!")
    }
}
impl Error for TestError {}

fn generate_error() -> DataStoreError {
    let err = TestError;
    DataStoreError {
        details: format!("{err:?}"),
    }
}

/// Implementation of [`DataVal`] that can be configured to return mock data.
/// Each field is optional, and the various implementations of the `DataVal` methods will attempt to read from
/// the corresponding field.
///
/// Regular methods:
/// If a field is populated, its value is returned. If it is not, an instance of [`TestError`] is generated
/// and returned.
///
/// Methods ending with `_optional`:
/// If a field is populated, its value is returned. If it is not, the [`is_nullable`](TestVal::is_nullable)
/// field is checked. If the field is `true`, [`None`] is returned. Else, an instance of [`TestError`] is
/// generated and returned.
#[derive(Debug)]
pub struct TestVal {
    pub is_nullable: bool,
    pub test_bool: Option<bool>,
    pub test_datetime: Option<DateTime<Utc>>,
    pub test_i8: Option<i8>,
    pub test_i16: Option<i16>,
    pub test_i32: Option<i32>,
    pub test_i64: Option<i64>,
    pub test_f32: Option<f32>,
    pub test_f64: Option<f64>,
    pub test_string: Option<String>,
}
impl TestVal {
    /// Convenience method for generating an instance of [`TestVal`] with all fields set to [`None`].
    pub fn new() -> Self {
        Self {
            is_nullable: true,
            test_bool: None,
            test_datetime: None,
            test_f32: None,
            test_f64: None,
            test_i16: None,
            test_i32: None,
            test_i64: None,
            test_i8: None,
            test_string: None,
        }
    }

    fn translate<T>(op: Option<T>) -> Result<T, DataStoreError> {
        op.ok_or_else(generate_error)
    }

    fn translate_optional<T>(&self, op: Option<T>) -> Result<Option<T>, DataStoreError> {
        if self.is_nullable || op.is_some() {
            Ok(op)
        } else {
            Err(generate_error())
        }
    }
}
impl Default for TestVal {
    fn default() -> Self {
        Self::new()
    }
}
impl DataVal for TestVal {
    fn to_bool(self) -> Result<bool, DataStoreError> {
        Self::translate(self.test_bool)
    }

    fn to_bool_optional(self) -> Result<Option<bool>, DataStoreError> {
        self.translate_optional(self.test_bool)
    }

    fn to_datetime(self) -> Result<DateTime<Utc>, DataStoreError> {
        Self::translate(self.test_datetime)
    }

    fn to_datetime_optional(self) -> Result<Option<DateTime<Utc>>, DataStoreError> {
        self.translate_optional(self.test_datetime)
    }

    fn to_i8(self) -> Result<i8, DataStoreError> {
        Self::translate(self.test_i8)
    }

    fn to_i8_optional(self) -> Result<Option<i8>, DataStoreError> {
        self.translate_optional(self.test_i8)
    }

    fn to_i16(self) -> Result<i16, DataStoreError> {
        Self::translate(self.test_i16)
    }

    fn to_i16_optional(self) -> Result<Option<i16>, DataStoreError> {
        self.translate_optional(self.test_i16)
    }

    fn to_i32(self) -> Result<i32, DataStoreError> {
        Self::translate(self.test_i32)
    }

    fn to_i32_optional(self) -> Result<Option<i32>, DataStoreError> {
        self.translate_optional(self.test_i32)
    }

    fn to_i64(self) -> Result<i64, DataStoreError> {
        Self::translate(self.test_i64)
    }

    fn to_i64_optional(self) -> Result<Option<i64>, DataStoreError> {
        self.translate_optional(self.test_i64)
    }

    fn to_f32(self) -> Result<f32, DataStoreError> {
        Self::translate(self.test_f32)
    }

    fn to_f32_optional(self) -> Result<Option<f32>, DataStoreError> {
        self.translate_optional(self.test_f32)
    }

    fn to_f64(self) -> Result<f64, DataStoreError> {
        Self::translate(self.test_f64)
    }

    fn to_f64_optional(self) -> Result<Option<f64>, DataStoreError> {
        self.translate_optional(self.test_f64)
    }

    fn to_string(self) -> Result<String, DataStoreError> {
        Self::translate(self.test_string)
    }

    fn to_string_optional(self) -> Result<Option<String>, DataStoreError> {
        let local = self.test_string.clone();
        self.translate_optional(local)
    }
}
impl PartialEq for TestVal {
    fn eq(&self, other: &Self) -> bool {
        self.is_nullable == other.is_nullable
            && self.test_bool == other.test_bool
            && self.test_datetime == other.test_datetime
            && self.test_f32 == other.test_f32
            && self.test_f64 == other.test_f64
            && self.test_i16 == other.test_i16
            && self.test_i32 == other.test_i32
            && self.test_i64 == other.test_i64
            && self.test_i8 == other.test_i8
            && self.test_string == other.test_string
    }
}

/// A single query captured by a [`TestDataStore`], in the form it was passed to the store.
#[derive(Clone, Debug, PartialEq)]
pub enum CapturedQuery {
    /// A query passed to [`execute_query`](DataStore::execute_query).
    Query(Cow<'static, str>),
    /// A query passed to [`execute_parameterized_query`](DataStore::execute_parameterized_query).
    ParameterizedQuery(ParameterizedQuery),
}

/// Implementation of [`DataStore`] that can be used in test cases.
///
/// Note that TestDataStore does not implement a true database. It always returns the `data`
/// provided at construction time, regardless of the query it was given. Do not use the
/// returned rows to assert that a query was built correctly; use them only to test how calling
/// code consumes/maps rows.
///
/// Every query passed to this store is recorded and can be inspected via
/// [`captured_queries`](Self::captured_queries), which is the correct way to assert on query
/// structure (statement text, bindings, etc.).
#[derive(Debug)]
pub struct TestDataStore<T: DataRow<TestVal> + Clone> {
    pub data: Vec<T>,
    queries: Mutex<Vec<CapturedQuery>>,
}
impl<T: DataRow<TestVal> + Clone> TestDataStore<T> {
    /// Convenience method for generating an instance of [`TestDataStore`] with the provided data.
    pub fn new(data: Vec<T>) -> Self {
        Self {
            data,
            queries: Mutex::new(Vec::new()),
        }
    }

    /// Returns the queries captured so far, in the order they were submitted.
    /// This is the intended way to assert that calling code constructed the correct query;
    /// [`data`](Self::data) is unconditional and will not reflect query content.
    pub fn captured_queries(&self) -> Vec<CapturedQuery> {
        self.queries.lock().unwrap().clone()
    }
}
impl<T: DataRow<TestVal> + Clone> DataStore<TestVal, T> for TestDataStore<T> {
    async fn execute_query(
        &self,
        query: impl Into<Cow<'static, str>> + Send,
    ) -> Result<Vec<T>, DataStoreError> {
        self.queries
            .lock()
            .unwrap()
            .push(CapturedQuery::Query(query.into()));
        Ok(self.data.clone())
    }

    async fn execute_parameterized_query(
        &self,
        parameterized_query: ParameterizedQuery,
    ) -> Result<Vec<T>, DataStoreError> {
        self.queries
            .lock()
            .unwrap()
            .push(CapturedQuery::ParameterizedQuery(parameterized_query));
        Ok(self.data.clone())
    }

    async fn execute_transaction(&self, _: Vec<ParameterizedQuery>) -> Result<(), DataStoreError> {
        Ok(())
    }
}
impl<T: DataRow<TestVal> + Clone> Clone for TestDataStore<T> {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            queries: Mutex::new(self.queries.lock().unwrap().clone()),
        }
    }
}
