//! Tests for the Rust DB Lib Testing Utilities Module

use super::*;
use crate::QueryParameter;

#[test]
fn test_val_to_bool() {
    let err_val = TestVal::default();
    assert!(err_val.to_bool().is_err());

    let mut val = TestVal::default();
    val.test_bool = Some(true);
    assert!(val.to_bool().unwrap());
}

#[test]
fn test_val_to_bool_optional() {
    let err_val = TestVal::default();
    assert!(err_val.to_bool_optional().is_err());

    let val = TestVal {
        is_nullable: true,
        ..TestVal::default()
    };
    assert!(val.to_bool_optional().unwrap().is_none());

    let val = TestVal {
        test_bool: Some(true),
        ..TestVal::default()
    };
    assert!(val.to_bool_optional().unwrap().unwrap());
}

#[test]
fn test_val_to_i8() {
    let err_val = TestVal::default();
    assert!(err_val.to_i8().is_err());

    let mut val = TestVal::default();
    val.test_i8 = Some(0_i8);
    assert_eq!(0_i8, val.to_i8().unwrap());
}

#[test]
fn test_val_to_i8_optional() {
    let err_val = TestVal::default();
    assert!(err_val.to_i8_optional().is_err());

    let val = TestVal {
        is_nullable: true,
        ..TestVal::default()
    };
    assert!(val.to_i8_optional().unwrap().is_none());

    let val = TestVal {
        test_i8: Some(0_i8),
        ..TestVal::default()
    };
    assert_eq!(0_i8, val.to_i8_optional().unwrap().unwrap());
}

#[test]
fn test_val_to_i16() {
    let err_val = TestVal::default();
    assert!(err_val.to_i16().is_err());

    let mut val = TestVal::default();
    val.test_i16 = Some(0_i16);
    assert_eq!(0_i16, val.to_i16().unwrap());
}

#[test]
fn test_val_to_i16_optional() {
    let err_val = TestVal::default();
    assert!(err_val.to_i16_optional().is_err());

    let val = TestVal {
        is_nullable: true,
        ..TestVal::default()
    };
    assert!(val.to_i16_optional().unwrap().is_none());

    let val = TestVal {
        test_i16: Some(0_i16),
        ..TestVal::default()
    };
    assert_eq!(0_i16, val.to_i16_optional().unwrap().unwrap());
}

#[test]
fn test_val_to_i32() {
    let err_val = TestVal::default();
    assert!(err_val.to_i32().is_err());

    let mut val = TestVal::default();
    val.test_i32 = Some(0_i32);
    assert_eq!(0_i32, val.to_i32().unwrap());
}

#[test]
fn test_val_to_i32_optional() {
    let err_val = TestVal::default();
    assert!(err_val.to_i32_optional().is_err());

    let val = TestVal {
        is_nullable: true,
        ..TestVal::default()
    };
    assert!(val.to_i32_optional().unwrap().is_none());

    let val = TestVal {
        test_i32: Some(0_i32),
        ..TestVal::default()
    };
    assert_eq!(0_i32, val.to_i32_optional().unwrap().unwrap());
}

#[test]
fn test_val_to_i64() {
    let err_val = TestVal::default();
    assert!(err_val.to_i64().is_err());

    let mut val = TestVal::default();
    val.test_i64 = Some(0_i64);
    assert_eq!(0_i64, val.to_i64().unwrap());
}

#[test]
fn test_val_to_i64_optional() {
    let err_val = TestVal::default();
    assert!(err_val.to_i64_optional().is_err());

    let val = TestVal {
        is_nullable: true,
        ..TestVal::default()
    };
    assert!(val.to_i64_optional().unwrap().is_none());

    let val = TestVal {
        test_i64: Some(0_i64),
        ..TestVal::default()
    };
    assert_eq!(0_i64, val.to_i64_optional().unwrap().unwrap());
}

#[test]
fn test_val_to_f32() {
    let err_val = TestVal::default();
    assert!(err_val.to_f32().is_err());

    let mut val = TestVal::default();
    val.test_f32 = Some(0_f32);
    assert_eq!(0_f32, val.to_f32().unwrap());
}

#[test]
fn test_val_to_f32_optional() {
    let err_val = TestVal::default();
    assert!(err_val.to_f32_optional().is_err());

    let val = TestVal {
        is_nullable: true,
        ..TestVal::default()
    };
    assert!(val.to_f32_optional().unwrap().is_none());

    let val = TestVal {
        test_f32: Some(0_f32),
        ..TestVal::default()
    };
    assert_eq!(0_f32, val.to_f32_optional().unwrap().unwrap());
}

#[test]
fn test_val_to_f64() {
    let err_val = TestVal::default();
    assert!(err_val.to_f64().is_err());

    let mut val = TestVal::default();
    val.test_f64 = Some(0_f64);
    assert_eq!(0_f64, val.to_f64().unwrap());
}

#[test]
fn test_val_to_f64_optional() {
    let err_val = TestVal::default();
    assert!(err_val.to_f64_optional().is_err());

    let val = TestVal {
        is_nullable: true,
        ..TestVal::default()
    };
    assert!(val.to_f64_optional().unwrap().is_none());

    let val = TestVal {
        test_f64: Some(0_f64),
        ..TestVal::default()
    };
    assert_eq!(0_f64, val.to_f64_optional().unwrap().unwrap());
}

#[test]
fn test_val_to_string() {
    let err_val = TestVal::default();
    assert!(err_val.to_string().is_err());

    let mut val = TestVal::default();
    val.test_string = Some(String::default());
    assert_eq!(String::default(), val.to_string().unwrap());
}

#[test]
fn test_val_to_string_optional() {
    let err_val = TestVal::default();
    assert!(err_val.to_string_optional().is_err());

    let val = TestVal {
        is_nullable: true,
        ..TestVal::default()
    };
    assert!(val.to_string_optional().unwrap().is_none());

    let val = TestVal {
        test_string: Some(String::default()),
        ..TestVal::default()
    };
    assert_eq!(
        String::default(),
        val.to_string_optional().unwrap().unwrap()
    );
}

#[test]
fn test_val_to_datetime() {
    let err_val = TestVal::default();
    assert!(err_val.to_datetime().is_err());

    let mut val = TestVal::default();
    let now = Utc::now();
    val.test_datetime = Some(now);
    assert_eq!(now, val.to_datetime().unwrap());
}

#[test]
fn test_val_to_datetime_optional() {
    let err_val = TestVal::default();
    assert!(err_val.to_datetime_optional().is_err());

    let val = TestVal {
        is_nullable: true,
        ..TestVal::default()
    };
    assert!(val.to_datetime_optional().unwrap().is_none());

    let now = Utc::now();
    let val = TestVal {
        test_datetime: Some(now),
        ..TestVal::default()
    };
    assert_eq!(now, val.to_datetime_optional().unwrap().unwrap());
}

#[tokio::test]
async fn test_data_store_returns_stored_values() {
    let mut test_val = TestVal::default();
    test_val.test_string = Some("row1".to_string());
    let data1 = TestRow::new(HashMap::from([("data".to_string(), test_val.clone())]));

    test_val.test_string = Some("row2".to_string());
    let data2 = TestRow::new(HashMap::from([("data".to_string(), test_val)]));

    let store = TestDataStore::new(vec![data1.clone(), data2.clone()]);

    let simple_results = store.execute_query("SELECT * FROM dummy").await.unwrap();
    assert_eq!(simple_results.len(), 2);
    assert_eq!(simple_results[0].get("data"), data1.get("data"));
    assert_eq!(simple_results[1].get("data"), data2.get("data"));

    let mut parameterized_query =
        ParameterizedQuery::new("SELECT * FROM dummy WHERE id = $1 AND name = $2");
    parameterized_query.bind(QueryParameter::I32(42));
    parameterized_query.bind(QueryParameter::Str("row1".to_string()));
    let parameterized_results = store
        .execute_parameterized_query(parameterized_query.clone())
        .await
        .unwrap();
    assert_eq!(parameterized_results, simple_results);
}

#[tokio::test]
async fn test_data_store_captures_queries() {
    let store = TestDataStore::new(vec![]);

    let simple_query = "SELECT * FROM dummy";
    store.execute_query(simple_query).await.unwrap();

    let mut parameterized_query =
        ParameterizedQuery::new("SELECT * FROM dummy WHERE id = $1 AND name = $2");
    parameterized_query.bind(QueryParameter::I32(42));
    parameterized_query.bind(QueryParameter::Str("row1".to_string()));
    store
        .execute_parameterized_query(parameterized_query.clone())
        .await
        .unwrap();

    assert_eq!(
        store.captured_operations(),
        vec![
            Operation::Query(simple_query.into()),
            Operation::ParameterizedQuery(parameterized_query),
        ]
    );
    // Ensure that cloning the store preserves the captured queries
    assert_eq!(
        store.clone().captured_operations(),
        store.captured_operations()
    );
}

#[tokio::test]
async fn test_data_store_transaction_empty_batch() {
    let store = TestDataStore::new(vec![]);
    let result = store.execute_transaction(vec![]).await;
    assert!(result.is_ok());
    assert_eq!(
        store.captured_operations(),
        vec![Operation::TransactionBegin, Operation::TransactionCommit,]
    );
}

#[tokio::test]
async fn test_data_store_transaction_with_queries() {
    let store = TestDataStore::new(vec![]);
    let mut q1 = ParameterizedQuery::new("INSERT INTO t VALUES ($1)");
    q1.bind(QueryParameter::I32(1));
    let mut q2 = ParameterizedQuery::new("INSERT INTO t VALUES ($1)");
    q2.bind(QueryParameter::I32(2));
    let result = store
        .execute_transaction(vec![q1.clone(), q2.clone()])
        .await;
    assert!(result.is_ok());
    assert_eq!(
        store.captured_operations(),
        vec![
            Operation::TransactionBegin,
            Operation::ParameterizedQuery(q1),
            Operation::ParameterizedQuery(q2),
            Operation::TransactionCommit,
        ]
    );
}

#[tokio::test]
async fn test_dropped_transaction_rolls_back() {
    let store = TestDataStore::new(vec![]);
    let transaction = store.begin_transaction().await.unwrap();
    drop(transaction);
    assert_eq!(
        store.captured_operations(),
        vec![Operation::TransactionBegin, Operation::TransactionRollback,]
    );
}
