use async_trait::async_trait;
use chrono::{DateTime, Utc};
use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

pub mod postgres;

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
pub trait DataRow<T: DataVal>: Send + Sync {
    /// Generates an instance of [`DataVal`] wrapping the contents of the specified column
    fn get(&self, column_name: &str) -> T;
}

/// Abstraction for a data store capable of executing queries
#[async_trait]
pub trait DataStore<T: DataVal, U: DataRow<T>>: Clone + Send + Sync {
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

/// A collection of prebuilt implementations of the traits in this library that are useful for unit tests.
pub mod test_utils {
    use super::{DataStoreError, DataVal};
    use chrono::{DateTime, Utc};
    use std::{
        error::Error,
        fmt::{self, Display, Formatter},
    };

    /// A default implementation of [`std::error::Error`] for use in test cases.
    #[derive(Debug)]
    pub struct TestError;
    impl Display for TestError {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "TestError!")
        }
    }
    impl Error for TestError {}

    /// Implementation of [`DataVal`] that can be configured to return mock data.
    /// Each field is optional, and the various implementations of the `DataVal` methods will attempt to read from the corresponding field.
    /// If a field is populated, its value is returned. If it is not, an instance of [`TestError`] is generated and returned.
    pub struct TestVal {
        pub test_bool: Option<bool>,
        pub test_i8: Option<i8>,
        pub test_i16: Option<i16>,
        pub test_i32: Option<i32>,
        pub test_i64: Option<i64>,
        pub test_f32: Option<f32>,
        pub test_f64: Option<f64>,
        pub test_string: Option<String>,
        pub test_datetime: Option<DateTime<Utc>>,
    }
    impl TestVal {
        /// Convenience method for generating an instance of [`TestVal`] with all fields set to [`None`].
        pub fn new() -> Self {
            Self {
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
            op.ok_or(DataStoreError::from(
                Box::new(TestError) as Box<dyn std::error::Error + Send + Sync>
            ))
        }
    }
    impl DataVal for TestVal {
        fn to_bool(self) -> Result<bool, DataStoreError> {
            Self::translate(self.test_bool)
        }

        fn to_i8(self) -> Result<i8, DataStoreError> {
            Self::translate(self.test_i8)
        }

        fn to_i16(self) -> Result<i16, DataStoreError> {
            Self::translate(self.test_i16)
        }

        fn to_i32(self) -> Result<i32, DataStoreError> {
            Self::translate(self.test_i32)
        }

        fn to_i64(self) -> Result<i64, DataStoreError> {
            Self::translate(self.test_i64)
        }

        fn to_f32(self) -> Result<f32, DataStoreError> {
            Self::translate(self.test_f32)
        }

        fn to_f64(self) -> Result<f64, DataStoreError> {
            Self::translate(self.test_f64)
        }

        fn to_string(self) -> Result<String, DataStoreError> {
            Self::translate(self.test_string)
        }

        fn to_datetime(self) -> Result<DateTime<Utc>, DataStoreError> {
            Self::translate(self.test_datetime)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_val_to_bool() {
            let err_val = TestVal::new();
            assert!(err_val.to_bool().is_err());

            let mut val = TestVal::new();
            val.test_bool = Some(true);
            assert!(val.to_bool().unwrap());
        }

        #[test]
        fn test_val_to_i8() {
            let err_val = TestVal::new();
            assert!(err_val.to_i8().is_err());

            let mut val = TestVal::new();
            val.test_i8 = Some(0_i8);
            assert_eq!(0_i8, val.to_i8().unwrap());
        }

        #[test]
        fn test_val_to_i16() {
            let err_val = TestVal::new();
            assert!(err_val.to_i16().is_err());

            let mut val = TestVal::new();
            val.test_i16 = Some(0_i16);
            assert_eq!(0_i16, val.to_i16().unwrap());
        }

        #[test]
        fn test_val_to_i32() {
            let err_val = TestVal::new();
            assert!(err_val.to_i32().is_err());

            let mut val = TestVal::new();
            val.test_i32 = Some(0_i32);
            assert_eq!(0_i32, val.to_i32().unwrap());
        }

        #[test]
        fn test_val_to_i64() {
            let err_val = TestVal::new();
            assert!(err_val.to_i64().is_err());

            let mut val = TestVal::new();
            val.test_i64 = Some(0_i64);
            assert_eq!(0_i64, val.to_i64().unwrap());
        }

        #[test]
        fn test_val_to_f32() {
            let err_val = TestVal::new();
            assert!(err_val.to_f32().is_err());

            let mut val = TestVal::new();
            val.test_f32 = Some(0_f32);
            assert_eq!(0_f32, val.to_f32().unwrap());
        }

        #[test]
        fn test_val_to_f64() {
            let err_val = TestVal::new();
            assert!(err_val.to_f64().is_err());

            let mut val = TestVal::new();
            val.test_f64 = Some(0_f64);
            assert_eq!(0_f64, val.to_f64().unwrap());
        }

        #[test]
        fn test_val_to_string() {
            let err_val = TestVal::new();
            assert!(err_val.to_string().is_err());

            let mut val = TestVal::new();
            val.test_string = Some(String::default());
            assert_eq!(String::default(), val.to_string().unwrap());
        }

        #[test]
        fn test_val_to_datetime() {
            let err_val = TestVal::new();
            assert!(err_val.to_datetime().is_err());

            let mut val = TestVal::new();
            let now = Utc::now();
            val.test_datetime = Some(now.clone());
            assert_eq!(now, val.to_datetime().unwrap());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_utils::TestVal;

    #[derive(Debug)]
    struct DummyRow {
        data: String,
    }
    impl DataRow<TestVal> for DummyRow {
        fn get(&self, _: &str) -> TestVal {
            let mut val = TestVal::new();
            val.test_string = Some(self.data.clone());
            val
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
    impl DataStore<TestVal, DummyRow> for DummyDataStore {
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
        let expected = results[0].get("").to_string();
        assert_eq!("row1".to_string(), expected.unwrap());
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
