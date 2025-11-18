use async_trait::async_trait;
use chrono::{DateTime, Utc};
use std::{error::Error, fmt::Display};

pub mod postgres;

/// Custom error type for DataStore operations
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

/// Abstraction representing a single row retrieved from a data store
///
/// To interact with the data represented by this row, the following methods are exposed:
///   - `get_bool_value(&self, column_name: &str)` -> `Result<bool, DataStoreError>`
///   - `get_datetime_value(&self, column_name: &str)` -> `Result<chrono::DateTime<chrono::Utc, DataStoreError>`
///   - `get_f32_value(&self, column_name: &str)` -> `Result<f32, DataStoreError>`
///   - `get_f64_value(&self, column_name: &str)` -> `Result<f64, DataStoreError>`
///   - `get_i32_value(&self, column_name: &str)` -> `Result<i32, DataStoreError>`
///   - `get_i64_value(&self, column_name: &str)` -> `Result<i64, DataStoreError>`
///   - `get_str_value(&self, column_name: &str)` -> `Result<String, DataStoreError>`
pub trait DataRow: Send + Sync {
    /// Attempts to return the value of the specified column as a `bool`.
    /// Returns an error if the data is not of the correct type.
    fn get_bool_value(&self, column_name: &str) -> Result<bool, DataStoreError>;

    /// Attempts to return the value of the specified column as a `chrono::DateTime<chrono::Utc>`.
    /// Returns an error if the data is not of the correct type.
    fn get_datetime_value(&self, column_name: &str) -> Result<DateTime<Utc>, DataStoreError>;

    /// Attempts to return the value of the specified column as a `f32`.
    /// Returns an error if the data is not of the correct type.
    fn get_f32_value(&self, column_name: &str) -> Result<f32, DataStoreError>;

    /// Attempts to return the value of the specified column as a `f64`.
    /// Returns an error if the data is not of the correct type.
    fn get_f64_value(&self, column_name: &str) -> Result<f64, DataStoreError>;

    /// Attempts to return the value of the specified column as a `i32`.
    /// Returns an error if the data is not of the correct type.
    fn get_i32_value(&self, column_name: &str) -> Result<i32, DataStoreError>;

    /// Attempts to return the value of the specified column as a `i64`.
    /// Returns an error if the data is not of the correct type.
    fn get_i64_value(&self, column_name: &str) -> Result<i64, DataStoreError>;

    /// Attempts to return the value of the specified column as a `String`.
    /// Returns an error if the data is not of the correct type.
    fn get_str_value(&self, column_name: &str) -> Result<String, DataStoreError>;
}

/// Abstraction for a data store capable of executing queries
#[async_trait]
pub trait DataStore<T: DataRow>: Clone + Send + Sync {
    /// Executes a basic SQL statement. If any user input is required, use [`execute_parameterized_query()`]
    async fn execute_query(&self, query: String) -> Result<Vec<T>, DataStoreError>;

    /// Executes a parameterized query. It is expected that the query string will have sequential placeholders
    /// for the paramterized data.
    ///
    /// Example: If passing 5 elements into the query. the query should contain $1, $2, $3, $4, and $5, and
    /// the bindings should have no fewer than 5 elements (any additional elements beyond 5 will be ignored).
    async fn execute_parameterized_query(
        &self,
        query: String,
        bindings: Vec<String>,
    ) -> Result<Vec<T>, DataStoreError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct DummyRow {
        data: String,
    }
    impl DataRow for DummyRow {
        fn get_bool_value(&self, _: &str) -> Result<bool, DataStoreError> {
            Ok(false)
        }
        fn get_datetime_value(&self, _: &str) -> Result<DateTime<Utc>, DataStoreError> {
            Ok(chrono::Utc::now())
        }
        fn get_f32_value(&self, _: &str) -> Result<f32, DataStoreError> {
            Ok(0.0)
        }
        fn get_f64_value(&self, _: &str) -> Result<f64, DataStoreError> {
            Ok(0.0)
        }
        fn get_i32_value(&self, _: &str) -> Result<i32, DataStoreError> {
            Ok(0)
        }
        fn get_i64_value(&self, _: &str) -> Result<i64, DataStoreError> {
            Ok(0)
        }
        fn get_str_value(&self, _: &str) -> Result<String, DataStoreError> {
            Ok(self.data.clone())
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
    impl DataStore<DummyRow> for DummyDataStore {
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
        assert_eq!(results[0].get_str_value("").unwrap(), "row1");
        assert!(
            results[0].get_datetime_value("").unwrap().timestamp()
                <= chrono::Utc::now().timestamp()
        );
        assert_eq!(results[0].get_bool_value("").unwrap(), false);
        assert_eq!(results[0].get_f32_value("").unwrap(), 0.0);
        assert_eq!(results[0].get_f64_value("").unwrap(), 0.0);
        assert_eq!(results[0].get_i32_value("").unwrap(), 0);
        assert_eq!(results[0].get_i64_value("").unwrap(), 0);
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
