use super::{DataRow, DataStore, DataStoreError, DataVal, ParameterizedQuery};
use async_trait::async_trait;
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
#[derive(Debug)]
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
impl Default for TestVal {
    fn default() -> Self {
        Self::new()
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
impl PartialEq for TestVal {
    fn eq(&self, other: &Self) -> bool {
        self.test_bool == other.test_bool
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

/// Implementation of [`ParameterizedQuery`] that can be used in test cases.
/// This implementation does not actually execute any queries, but simply returns the data provided at construction time.
/// All `bind_` methods are no-ops, except for `bind_string`, which records the latest string parameter that was bound.
#[derive(Debug)]
pub struct TestParameterizedQuery<T: DataRow<TestVal>> {
    /// The data to be returned when the query is executed.
    pub data: Vec<T>,
    /// The latest string parameter that was bound to the query.
    pub latest_string_binding: String,
}
impl<T: DataRow<TestVal>> TestParameterizedQuery<T> {
    /// Convenience method for generating an instance of [`TestParameterizedQuery`] with the provided data.
    pub fn new(data: Vec<T>) -> Self {
        Self {
            data,
            latest_string_binding: String::new(),
        }
    }
}
#[async_trait]
impl<'a, T: DataRow<TestVal>> ParameterizedQuery<'a, TestVal, T> for TestParameterizedQuery<T> {
    fn bind_bool(&'a mut self, _: bool) -> Result<(), DataStoreError> {
        Ok(())
    }

    fn bind_datetime(&'a mut self, _: DateTime<Utc>) -> Result<(), DataStoreError> {
        Ok(())
    }

    fn bind_i8(&'a mut self, _: i8) -> Result<(), DataStoreError> {
        Ok(())
    }

    fn bind_i16(&'a mut self, _: i16) -> Result<(), DataStoreError> {
        Ok(())
    }

    fn bind_i32(&'a mut self, _: i32) -> Result<(), DataStoreError> {
        Ok(())
    }

    fn bind_i64(&'a mut self, _: i64) -> Result<(), DataStoreError> {
        Ok(())
    }

    fn bind_f32(&'a mut self, _: f32) -> Result<(), DataStoreError> {
        Ok(())
    }

    fn bind_f64(&'a mut self, _: f64) -> Result<(), DataStoreError> {
        Ok(())
    }

    fn bind_string(&'a mut self, parameter: String) -> Result<(), DataStoreError> {
        self.latest_string_binding = parameter;
        Ok(())
    }

    async fn execute(self) -> Result<Vec<T>, DataStoreError> {
        Ok(self.data)
    }
}

/// Implementation of [`DataStore`] that can be used in test cases.
/// This implementation does not actually connect to any database, but simply returns the data provided at construction time.
/// Calling `init_parameterized_query` will return an instance of [`TestParameterizedQuery`] initialized with the same data.
#[derive(Debug)]
pub struct TestDataStore<T: DataRow<TestVal> + Clone> {
    pub data: Vec<T>,
}
impl<T: DataRow<TestVal> + Clone> TestDataStore<T> {
    /// Convenience method for generating an instance of [`TestDataStore`] with the provided data.
    pub fn new(data: Vec<T>) -> Self {
        Self { data }
    }
}
#[async_trait]
impl<'a, T: DataRow<TestVal> + Clone> DataStore<'a, TestVal, T> for TestDataStore<T> {
    type ParamQueryImpl = TestParameterizedQuery<T>;

    async fn execute_query(&self, _: &str) -> Result<Vec<T>, DataStoreError> {
        Ok(self.data.clone())
    }
    fn init_parameterized_query(&'a self, _: &str) -> Self::ParamQueryImpl {
        TestParameterizedQuery::new(self.data.clone())
    }
}
impl<T: DataRow<TestVal> + Clone> Clone for TestDataStore<T> {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
        }
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

    #[derive(Debug)]
    struct TestRow {
        pub data: String,
    }
    impl DataRow<TestVal> for TestRow {
        fn get(&self, _: &str) -> TestVal {
            let mut val = TestVal::new();
            val.test_string = Some(self.data.clone());
            val
        }
    }
    impl Clone for TestRow {
        fn clone(&self) -> Self {
            Self {
                data: self.data.clone(),
            }
        }
    }
    impl PartialEq for TestRow {
        fn eq(&self, other: &Self) -> bool {
            self.data == other.data
        }
    }

    #[test]
    fn test_parameterized_query_bind_bool() {
        let mut pq: TestParameterizedQuery<TestRow> = TestParameterizedQuery::new(vec![]);
        assert!(pq.bind_bool(true).is_ok());
    }

    #[test]
    fn test_parameterized_query_bind_datetime() {
        let mut pq: TestParameterizedQuery<TestRow> = TestParameterizedQuery::new(vec![]);
        assert!(pq.bind_datetime(Utc::now()).is_ok());
    }

    #[test]
    fn test_parameterized_query_bind_i8() {
        let mut pq: TestParameterizedQuery<TestRow> = TestParameterizedQuery::new(vec![]);
        assert!(pq.bind_i8(0).is_ok());
    }

    #[test]
    fn test_parameterized_query_bind_i16() {
        let mut pq: TestParameterizedQuery<TestRow> = TestParameterizedQuery::new(vec![]);
        assert!(pq.bind_i16(0).is_ok());
    }

    #[test]
    fn test_parameterized_query_bind_i32() {
        let mut pq: TestParameterizedQuery<TestRow> = TestParameterizedQuery::new(vec![]);
        assert!(pq.bind_i32(0).is_ok());
    }

    #[test]
    fn test_parameterized_query_bind_i64() {
        let mut pq: TestParameterizedQuery<TestRow> = TestParameterizedQuery::new(vec![]);
        assert!(pq.bind_i64(0).is_ok());
    }

    #[test]
    fn test_parameterized_query_bind_f32() {
        let mut pq: TestParameterizedQuery<TestRow> = TestParameterizedQuery::new(vec![]);
        assert!(pq.bind_f32(0.0).is_ok());
    }

    #[test]
    fn test_parameterized_query_bind_f64() {
        let mut pq: TestParameterizedQuery<TestRow> = TestParameterizedQuery::new(vec![]);
        assert!(pq.bind_f64(0.0).is_ok());
    }

    #[tokio::test]
    async fn test_data_store() {
        let data1 = TestRow {
            data: "row1".to_string(),
        };
        let data2 = TestRow {
            data: "row2".to_string(),
        };
        let store = TestDataStore::new(vec![data1, data2]);
        assert_eq!(store.data, store.clone().data);
        let results = store.execute_query("SELECT * FROM dummy").await.unwrap();
        assert_eq!(results.len(), 2);
        let expected = results[0].get("").to_string();
        assert_eq!("row1".to_string(), expected.unwrap());

        let mut parameterized_query = store.init_parameterized_query("SELECT * FROM dummy");
        parameterized_query
            .bind_string("Some binding".to_string())
            .unwrap();
        assert_eq!(
            parameterized_query.latest_string_binding,
            "Some binding".to_string()
        );

        let parameterized_results = parameterized_query.execute().await.unwrap();
        assert_eq!(parameterized_results, results);
    }

    #[test]
    fn test_display_test_error() {
        let err = TestError;
        assert_eq!(format!("{}", err), "TestError!");
    }

    #[test]
    fn test_testval_default() {
        let val = TestVal::default();
        assert_eq!(val, TestVal::new());
    }
}
