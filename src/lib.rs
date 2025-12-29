use async_trait::async_trait;
use chrono::{DateTime, Utc};
use std::{error::Error, fmt::Display};

pub mod postgres;

/// Custom error type for [`DataStore`] operations
#[derive(Debug)]
pub struct DataStoreError {
    details: String,
}
impl Display for DataStoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DataStoreError: {}", self.details)
    }
}
impl Error for DataStoreError {}

/// Represents the value stored in a database column. In this intermediate state,
/// the exact type of the data is unknown. Calling one of the trait methods will attempt to decode
/// the value as the desired type. An error will be returned if the column does not exist or the
/// data cannot be decoded as the requested type.
pub trait DBVal {
    /// Attempts to decode the value as a [`bool`].
    fn to_bool(self) -> Result<bool, DataStoreError>;

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

    /// Attempts to decode the value as a [`DateTime<Utc>`].
    fn to_datetime(self) -> Result<DateTime<Utc>, DataStoreError>;
}

/// Abstraction representing a single row retrieved from a data store
pub trait DataRow<'a, T: DBVal>: Send + Sync {
    /// Generates an instance of [`DBVal`] wrapping the contents of the specified column
    fn get(&'a self, column_name: &str) -> T;
}

/// Abstraction for a data store capable of executing queries
#[async_trait]
pub trait DataStore<'a, T: DBVal, U: DataRow<'a, T>>: Clone + Send + Sync {
    /// Executes a basic SQL statement. If any user input is required, use [`execute_parameterized_query`](Self::execute_parameterized_query)
    async fn execute_query(&self, query: String) -> Result<Vec<U>, DataStoreError>;

    /// Executes a parameterized query. It is expected that the query string will have sequential placeholders
    /// for the paramterized data.
    ///
    /// Example: If passing 5 elements into the query. the query should contain `$1`, `$2`, `$3`, `$4`, and `$5`, and
    /// the bindings should have no fewer than 5 elements (any additional elements beyond 5 will be ignored).
    async fn execute_parameterized_query(
        &self,
        query: String,
        bindings: Vec<String>,
    ) -> Result<Vec<U>, DataStoreError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct DummyVal;
    impl DBVal for DummyVal {
        fn to_bool(self) -> Result<bool, DataStoreError> {
            Ok(true)
        }

        fn to_i8(self) -> Result<i8, DataStoreError> {
            Ok(0_i8)
        }

        fn to_i16(self) -> Result<i16, DataStoreError> {
            Ok(0_i16)
        }

        fn to_i32(self) -> Result<i32, DataStoreError> {
            Ok(0_i32)
        }

        fn to_i64(self) -> Result<i64, DataStoreError> {
            Ok(0_i64)
        }

        fn to_f32(self) -> Result<f32, DataStoreError> {
            Ok(0_f32)
        }

        fn to_f64(self) -> Result<f64, DataStoreError> {
            Ok(0_f64)
        }

        fn to_string(self) -> Result<String, DataStoreError> {
            Ok(String::default())
        }

        fn to_datetime(self) -> Result<DateTime<Utc>, DataStoreError> {
            Ok(Utc::now())
        }
    }

    #[derive(Debug)]
    struct DummyRow {
        data: String,
    }
    impl<'a> DataRow<'a, DummyVal> for DummyRow {
        fn get(&'a self, _: &str) -> DummyVal {
            DummyVal {}
        }
    }
    impl Clone for DummyRow {
        fn clone(&self) -> Self {
            DummyRow {
                data: self.data.clone(),
            }
        }
    }
    impl PartialEq for DummyRow {
        fn eq(&self, other: &Self) -> bool {
            self.data == other.data
        }
    }

    struct DummyDataStore {
        data: Vec<DummyRow>,
    }
    #[async_trait]
    impl<'a> DataStore<'a, DummyVal, DummyRow> for DummyDataStore {
        async fn execute_query(&self, _: String) -> Result<Vec<DummyRow>, DataStoreError> {
            Ok(self.data.clone())
        }
        async fn execute_parameterized_query(
            &self,
            _: String,
            _: Vec<String>,
        ) -> Result<Vec<DummyRow>, DataStoreError> {
            Ok(self.data.clone())
        }
    }
    impl Clone for DummyDataStore {
        fn clone(&self) -> Self {
            Self {
                data: self.data.clone(),
            }
        }
    }

    #[tokio::test]
    async fn test_dummy_data_store() {
        let data1 = DummyRow {
            data: "row1".to_string(),
        };
        let data2 = DummyRow {
            data: "row2".to_string(),
        };
        let store = DummyDataStore {
            data: vec![data1, data2],
        };
        let results = store
            .execute_query("SELECT * FROM dummy".to_string())
            .await
            .unwrap();
        assert_eq!(results.len(), 2);
        let expected = results[0].get("").to_i32();
        assert!(expected.is_ok());
        assert_eq!(store.data, store.clone().data);

        let parameterized_results = store
            .execute_parameterized_query(
                "SELECT * FROM dummy".to_string(),
                vec!["Some binding".to_string()],
            )
            .await
            .unwrap();
        assert_eq!(parameterized_results, results);
    }

    #[test]
    fn test_display_datastore_error() {
        let error = DataStoreError {
            details: "Test error".to_string(),
        };
        assert_eq!(format!("{}", error), "DataStoreError: Test error");
    }
}
