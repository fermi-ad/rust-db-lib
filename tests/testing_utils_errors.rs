// The unused dependency checker will flag chrono and sqlx as unused for this
// integration test crate. Unused dependencies in tests are not a big concern,
// as we will still use them in the main binary.
#![allow(unused_crate_dependencies)]

use rust_db_lib::{
    DataStore, DataStoreTransaction, ParameterizedQuery, TransactionError,
    testing_utils::{Operation, test_data_store},
};

#[tokio::test]
async fn consumer_can_configure_query_errors_and_clone_independently() {
    let store = test_data_store!([[("name", "Ada")]]);
    let failing = store.clone().with_query_error("unavailable");
    assert!(
        failing
            .execute_query("SELECT 1")
            .await
            .unwrap_err()
            .to_string()
            .contains("unavailable")
    );
    assert!(
        failing
            .execute_parameterized_query(ParameterizedQuery::new("SELECT 2"))
            .await
            .is_err()
    );
    assert_eq!(store.execute_query("SELECT 3").await.unwrap().len(), 1);
    assert_eq!(
        failing.captured_operations(),
        vec![
            Operation::Query("SELECT 1".into()),
            Operation::ParameterizedQuery(ParameterizedQuery::new("SELECT 2")),
        ]
    );
}

#[tokio::test]
async fn consumer_can_configure_transaction_errors() {
    let store = test_data_store!([]).with_begin_error(TransactionError::Retryable);
    assert!(matches!(
        store.begin_transaction().await,
        Err(TransactionError::Retryable)
    ));
    assert_eq!(
        store.captured_operations(),
        vec![Operation::TransactionBegin]
    );

    let store = test_data_store!([]).with_transaction_query_error(TransactionError::Retryable);
    let mut transaction = store.begin_transaction().await.unwrap();
    assert!(matches!(
        transaction.execute_query("SELECT 1").await,
        Err(TransactionError::Retryable)
    ));
    transaction.rollback().await.unwrap();
    assert_eq!(
        store.captured_operations(),
        vec![
            Operation::TransactionBegin,
            Operation::Query("SELECT 1".into()),
            Operation::TransactionRollback,
        ]
    );

    let store = test_data_store!([]).with_transaction_query_error(TransactionError::Retryable);
    let query = ParameterizedQuery::new("INSERT INTO t VALUES (1)");
    assert!(
        store
            .execute_transaction(vec![query.clone()])
            .await
            .is_err()
    );
    assert_eq!(
        store.captured_operations(),
        vec![
            Operation::TransactionBegin,
            Operation::ParameterizedQuery(query),
            Operation::TransactionRollback,
        ]
    );

    let store = test_data_store!([])
        .with_commit_error(TransactionError::DatabaseError("commit failed".into()));
    let transaction = store.begin_transaction().await.unwrap();
    assert!(matches!(
        transaction.commit().await,
        Err(TransactionError::DatabaseError(_))
    ));
    assert_eq!(
        store.captured_operations(),
        vec![Operation::TransactionBegin, Operation::TransactionCommit,]
    );

    let store = test_data_store!([]).with_rollback_error(TransactionError::Retryable);
    let transaction = store.begin_transaction().await.unwrap();
    assert!(matches!(
        transaction.rollback().await,
        Err(TransactionError::Retryable)
    ));
    assert_eq!(
        store.captured_operations(),
        vec![Operation::TransactionBegin, Operation::TransactionRollback,]
    );
}
