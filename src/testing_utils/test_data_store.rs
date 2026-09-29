use std::{borrow::Cow, cell::Cell, sync::Arc};

use crate::{
    DataStore, DataStoreError, ParameterizedQuery, TransactionError,
    testing_utils::{Operation, TestDataInner, TestRow, TestTransaction},
};

/// Test [`DataStore`] that returns configured rows and records operations.
///
/// Construct with [`new`](Self::new) or [`test_data_store!`](super::test_data_store).
/// Every query, including one run in a transaction, returns the same configured rows;
/// SQL is not evaluated. Use the returned rows to test mapping logic, not query correctness.
/// Check [`captured_operations`](Self::captured_operations) to assert on statement text,
/// bindings, and transaction begin/commit/rollback events. Clones share the same captured
/// operations.
#[derive(Clone, Debug)]
pub struct TestDataStore {
    inner: Arc<TestDataInner>,
}
impl TestDataStore {
    /// Creates a store that returns the provided rows for every query.
    pub fn new(data: Vec<TestRow>) -> Self {
        Self {
            inner: Arc::new(TestDataInner::new(data)),
        }
    }

    /// Returns the operations captured so far, in the order they were submitted.
    /// Use this to assert that calling code constructed the correct query.
    pub fn captured_operations(&self) -> Vec<Operation> {
        self.inner.operations.lock().unwrap().clone()
    }
}
impl DataStore for TestDataStore {
    type Row = TestRow;
    type Transaction = TestTransaction;

    async fn execute_query(
        &self,
        query: impl Into<Cow<'static, str>> + Send,
    ) -> Result<Vec<Self::Row>, DataStoreError> {
        self.inner
            .operations
            .lock()
            .unwrap()
            .push(Operation::Query(query.into()));
        Ok(self.inner.data.clone())
    }

    async fn execute_parameterized_query(
        &self,
        parameterized_query: ParameterizedQuery,
    ) -> Result<Vec<Self::Row>, DataStoreError> {
        self.inner
            .operations
            .lock()
            .unwrap()
            .push(Operation::ParameterizedQuery(parameterized_query));
        Ok(self.inner.data.clone())
    }

    async fn begin_transaction(&self) -> Result<Self::Transaction, TransactionError> {
        self.inner
            .operations
            .lock()
            .unwrap()
            .push(Operation::TransactionBegin);
        Ok(TestTransaction {
            inner: self.inner.clone(),
            completed: Cell::new(false),
        })
    }
}
