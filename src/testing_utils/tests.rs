//! Tests for the Rust DB Lib Testing Utilities Module

use chrono::{FixedOffset, TimeZone, Utc};

use super::*;
use crate::{DataRow, DataStore, DataVal, QueryParameter};

#[test]
fn test_val_from_supported_types() {
    assert_eq!(
        TestVal::from(true),
        TestVal {
            test_bool: Some(true),
            ..TestVal::default()
        }
    );

    macro_rules! assert_integer_conversion {
        ($value:expr, $field:ident) => {
            assert_eq!(
                TestVal::from($value),
                TestVal {
                    $field: Some($value),
                    ..TestVal::default()
                }
            );
        };
    }
    assert_integer_conversion!(-8_i8, test_i8);
    assert_integer_conversion!(-16_i16, test_i16);
    assert_integer_conversion!(-32_i32, test_i32);
    assert_integer_conversion!(-64_i64, test_i64);

    assert_eq!(
        TestVal::from(1.25_f32),
        TestVal {
            test_f32: Some(1.25),
            ..TestVal::default()
        }
    );
    assert_eq!(
        TestVal::from(2.5_f64),
        TestVal {
            test_f64: Some(2.5),
            ..TestVal::default()
        }
    );
    for val in [
        TestVal::from("borrowed"),
        TestVal::from(String::from("borrowed")),
    ] {
        assert_eq!(
            val,
            TestVal {
                test_string: Some(String::from("borrowed")),
                ..TestVal::default()
            }
        );
    }
}

#[test]
fn test_val_from_datetime_normalizes_time_zone() {
    let utc = Utc.with_ymd_and_hms(2024, 2, 3, 4, 5, 6).unwrap();
    let offset = FixedOffset::east_opt(5 * 3600).unwrap();
    let local = utc.with_timezone(&offset);
    let expected = TestVal {
        test_datetime: Some(utc),
        ..TestVal::default()
    };

    assert_eq!(TestVal::from(utc), expected);
    assert_eq!(TestVal::from(local), expected);
    assert_eq!(TestVal::from(local).to_datetime().unwrap(), utc);
}

#[tokio::test]
async fn test_data_store_macro_builds_independent_rows_with_nulls_and_expressions() {
    let created = Utc.with_ymd_and_hms(2024, 2, 3, 4, 5, 6).unwrap();
    let store = test_data_store!([
        [
            ("name", "first"),
            ("count", 42),
            ("active", true),
            ("optional", None),
            ("created", created),
        ],
        [
            ("name", String::from("second")),
            ("count", 43_i64),
            ("active", false),
            (
                "created",
                created.with_timezone(&FixedOffset::east_opt(3600).unwrap())
            ),
        ],
    ]);

    let rows = store.execute_query("SELECT * FROM mock").await.unwrap();
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0].get("name").to_string().unwrap(), "first");
    assert_eq!(rows[0].get("count").to_i32().unwrap(), 42);
    assert!(rows[0].get("count").to_i64().is_err());
    assert!(rows[0].get("active").to_bool().unwrap());
    assert_eq!(rows[0].get("created").to_datetime().unwrap(), created);
    assert_eq!(rows[0].get("optional").to_string_optional().unwrap(), None);
    assert!(rows[0].get("optional").to_string().is_err());
    assert_eq!(rows[1].get("name").to_string().unwrap(), "second");
    assert_eq!(rows[1].get("count").to_i64().unwrap(), 43);
    assert!(!rows[1].get("active").to_bool().unwrap());
    assert_eq!(rows[1].get("created").to_datetime().unwrap(), created);
    assert!(rows[1].get("optional").to_string_optional().is_err());
    assert!(rows[0].get("missing").to_string_optional().is_err());
}

#[tokio::test]
async fn test_data_store_macro_accepts_empty_rows_and_store() {
    let empty_store = test_data_store!([]);
    assert!(
        empty_store
            .execute_query("SELECT 1")
            .await
            .unwrap()
            .is_empty()
    );

    let store = test_data_store!([[], [("flag", None)]]);
    let rows = store.execute_query("SELECT 1").await.unwrap();
    assert_eq!(rows.len(), 2);
    assert!(rows[0].get("flag").to_bool_optional().is_err());
    assert_eq!(rows[1].get("flag").to_bool_optional().unwrap(), None);
}

#[test]
fn test_val_to_bool() {
    let err_val = TestVal::default();
    assert!(err_val.to_bool().is_err());

    let val = TestVal {
        test_bool: Some(true),
        ..TestVal::default()
    };
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

    let val = TestVal {
        test_i8: Some(0_i8),
        ..TestVal::default()
    };
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

    let val = TestVal {
        test_i16: Some(0_i16),
        ..TestVal::default()
    };
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

    let val = TestVal {
        test_i32: Some(0_i32),
        ..TestVal::default()
    };
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

    let val = TestVal {
        test_i64: Some(0_i64),
        ..TestVal::default()
    };
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

    let val = TestVal {
        test_f32: Some(0_f32),
        ..TestVal::default()
    };
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

    let val = TestVal {
        test_f64: Some(0_f64),
        ..TestVal::default()
    };
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

    let val = TestVal {
        test_string: Some(String::default()),
        ..TestVal::default()
    };
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

    let now = Utc::now();
    let val = TestVal {
        test_datetime: Some(now),
        ..TestVal::default()
    };
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
    let store = test_data_store!([[("data", "row1")], [("data", "row2")]]);

    let simple_results = store.execute_query("SELECT * FROM dummy").await.unwrap();
    assert_eq!(simple_results.len(), 2);
    assert_eq!(simple_results[0].get("data"), TestVal::from("row1"));
    assert_eq!(simple_results[1].get("data"), TestVal::from("row2"));

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
    let store = test_data_store!([]);

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
    let store = test_data_store!([]);
    let result = store.execute_transaction(vec![]).await;
    assert!(result.is_ok());
    assert_eq!(
        store.captured_operations(),
        vec![Operation::TransactionBegin, Operation::TransactionCommit,]
    );
}

#[tokio::test]
async fn test_data_store_transaction_with_queries() {
    let store = test_data_store!([]);
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
    let store = test_data_store!([]);
    let transaction = store.begin_transaction().await.unwrap();
    drop(transaction);
    assert_eq!(
        store.captured_operations(),
        vec![Operation::TransactionBegin, Operation::TransactionRollback,]
    );
}
