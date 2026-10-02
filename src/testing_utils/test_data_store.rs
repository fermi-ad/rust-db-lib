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

    /// Makes both kinds of store query return this error instead of rows.
    /// Configure the store before cloning or executing it.
    pub fn with_query_error(mut self, error: impl Into<DataStoreError>) -> Self {
        Arc::make_mut(&mut self.inner).errors.query = Some(error.into());
        self
    }

    /// Makes both kinds of queries within a transaction return this error.
    pub fn with_transaction_query_error(mut self, error: impl Into<TransactionError>) -> Self {
        Arc::make_mut(&mut self.inner).errors.transaction_query = Some(error.into());
        self
    }

    /// Makes opening a transaction fail with this error.
    pub fn with_begin_error(mut self, error: impl Into<TransactionError>) -> Self {
        Arc::make_mut(&mut self.inner).errors.begin = Some(error.into());
        self
    }

    /// Makes committing a transaction fail with this error.
    pub fn with_commit_error(mut self, error: impl Into<TransactionError>) -> Self {
        Arc::make_mut(&mut self.inner).errors.commit = Some(error.into());
        self
    }

    /// Makes rolling back a transaction fail with this error.
    pub fn with_rollback_error(mut self, error: impl Into<TransactionError>) -> Self {
        Arc::make_mut(&mut self.inner).errors.rollback = Some(error.into());
        self
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
        if let Some(error) = &self.inner.errors.query {
            return Err(error.clone());
        }
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
        if let Some(error) = &self.inner.errors.query {
            return Err(error.clone());
        }
        Ok(self.inner.data.clone())
    }

    async fn begin_transaction(&self) -> Result<Self::Transaction, TransactionError> {
        self.inner
            .operations
            .lock()
            .unwrap()
            .push(Operation::TransactionBegin);
        if let Some(error) = &self.inner.errors.begin {
            return Err(error.clone());
        }
        Ok(TestTransaction {
            inner: self.inner.clone(),
            completed: Cell::new(false),
        })
    }
}
