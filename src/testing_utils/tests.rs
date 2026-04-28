//! Tests for the Rust DB Lib Testing Utilities Module

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
fn test_val_to_bool_optional() {
    let mut err_val = TestVal::new();
    err_val.is_nullable = false;
    assert!(err_val.to_bool_optional().is_err());

    let val = TestVal::new();
    assert!(val.to_bool_optional().unwrap().is_none());

    let mut val = TestVal::new();
    val.test_bool = Some(true);
    assert!(val.to_bool_optional().unwrap().unwrap());
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
fn test_val_to_i8_optional() {
    let mut err_val = TestVal::new();
    err_val.is_nullable = false;
    assert!(err_val.to_i8_optional().is_err());

    let val = TestVal::new();
    assert!(val.to_i8_optional().unwrap().is_none());

    let mut val = TestVal::new();
    val.test_i8 = Some(0_i8);
    assert_eq!(0_i8, val.to_i8_optional().unwrap().unwrap());
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
fn test_val_to_i16_optional() {
    let mut err_val = TestVal::new();
    err_val.is_nullable = false;
    assert!(err_val.to_i16_optional().is_err());

    let val = TestVal::new();
    assert!(val.to_i16_optional().unwrap().is_none());

    let mut val = TestVal::new();
    val.test_i16 = Some(0_i16);
    assert_eq!(0_i16, val.to_i16_optional().unwrap().unwrap());
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
fn test_val_to_i32_optional() {
    let mut err_val = TestVal::new();
    err_val.is_nullable = false;
    assert!(err_val.to_i32_optional().is_err());

    let val = TestVal::new();
    assert!(val.to_i32_optional().unwrap().is_none());

    let mut val = TestVal::new();
    val.test_i32 = Some(0_i32);
    assert_eq!(0_i32, val.to_i32_optional().unwrap().unwrap());
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
fn test_val_to_i64_optional() {
    let mut err_val = TestVal::new();
    err_val.is_nullable = false;
    assert!(err_val.to_i64_optional().is_err());

    let val = TestVal::new();
    assert!(val.to_i64_optional().unwrap().is_none());

    let mut val = TestVal::new();
    val.test_i64 = Some(0_i64);
    assert_eq!(0_i64, val.to_i64_optional().unwrap().unwrap());
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
fn test_val_to_f32_optional() {
    let mut err_val = TestVal::new();
    err_val.is_nullable = false;
    assert!(err_val.to_f32_optional().is_err());

    let val = TestVal::new();
    assert!(val.to_f32_optional().unwrap().is_none());

    let mut val = TestVal::new();
    val.test_f32 = Some(0_f32);
    assert_eq!(0_f32, val.to_f32_optional().unwrap().unwrap());
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
fn test_val_to_f64_optional() {
    let mut err_val = TestVal::new();
    err_val.is_nullable = false;
    assert!(err_val.to_f64_optional().is_err());

    let val = TestVal::new();
    assert!(val.to_f64_optional().unwrap().is_none());

    let mut val = TestVal::new();
    val.test_f64 = Some(0_f64);
    assert_eq!(0_f64, val.to_f64_optional().unwrap().unwrap());
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
fn test_val_to_string_optional() {
    let mut err_val = TestVal::new();
    err_val.is_nullable = false;
    assert!(err_val.to_string_optional().is_err());

    let val = TestVal::new();
    assert!(val.to_string_optional().unwrap().is_none());

    let mut val = TestVal::new();
    val.test_string = Some(String::default());
    assert_eq!(
        String::default(),
        val.to_string_optional().unwrap().unwrap()
    );
}

#[test]
fn test_val_to_datetime() {
    let err_val = TestVal::new();
    assert!(err_val.to_datetime().is_err());

    let mut val = TestVal::new();
    let now = Utc::now();
    val.test_datetime = Some(now);
    assert_eq!(now, val.to_datetime().unwrap());
}

#[test]
fn test_val_to_datetime_optional() {
    let mut err_val = TestVal::new();
    err_val.is_nullable = false;
    assert!(err_val.to_datetime_optional().is_err());

    let val = TestVal::new();
    assert!(val.to_datetime_optional().unwrap().is_none());

    let mut val = TestVal::new();
    let now = Utc::now();
    val.test_datetime = Some(now);
    assert_eq!(now, val.to_datetime_optional().unwrap().unwrap());
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

    let parameterized_query = ParameterizedQuery::new(String::new());

    let parameterized_results = store
        .execute_parameterized_query(parameterized_query)
        .await
        .unwrap();
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
