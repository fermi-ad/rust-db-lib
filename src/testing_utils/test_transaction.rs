use std::{borrow::Cow, cell::Cell, sync::Arc};

use crate::{
    DataStoreTransaction, ParameterizedQuery, TransactionError,
    testing_utils::{Operation, TestDataInner, TestRow},
};

/// Transaction handle returned by [`TestDataStore`](super::TestDataStore).
///
/// Queries return the store's configured rows and are recorded as [`Operation`](super::Operation)s.
/// Committing or rolling back records the corresponding operation; dropping an unfinished
/// transaction records a rollback. This is a recording mock, not a database transaction.
pub struct TestTransaction {
    pub(crate) inner: Arc<TestDataInner>,
    pub(crate) completed: Cell<bool>,
}

impl Drop for TestTransaction {
    fn drop(&mut self) {
        if !self.completed.get() {
            self.inner
                .operations
                .lock()
                .unwrap()
                .push(Operation::TransactionRollback);
        }
    }
}

impl DataStoreTransaction for TestTransaction {
    type Row = TestRow;

    async fn execute_query(
        &mut self,
        query: impl Into<Cow<'static, str>> + Send,
    ) -> Result<Vec<Self::Row>, TransactionError> {
        self.inner
            .operations
            .lock()
            .unwrap()
            .push(Operation::Query(query.into()));
        if let Some(error) = &self.inner.errors.transaction_query {
            return Err(error.clone());
        }
        Ok(self.inner.data.clone())
    }

    async fn execute_parameterized_query(
        &mut self,
        parameterized_query: ParameterizedQuery,
    ) -> Result<Vec<Self::Row>, TransactionError> {
        self.inner
            .operations
            .lock()
            .unwrap()
            .push(Operation::ParameterizedQuery(parameterized_query));
        if let Some(error) = &self.inner.errors.transaction_query {
            return Err(error.clone());
        }
        Ok(self.inner.data.clone())
    }

    async fn commit(self) -> Result<(), TransactionError> {
        self.completed.set(true);
        self.inner
            .operations
            .lock()
            .unwrap()
            .push(Operation::TransactionCommit);
        match &self.inner.errors.commit {
            Some(error) => Err(error.clone()),
            None => Ok(()),
        }
    }

    async fn rollback(self) -> Result<(), TransactionError> {
        self.completed.set(true);
        self.inner
            .operations
            .lock()
            .unwrap()
            .push(Operation::TransactionRollback);
        match &self.inner.errors.rollback {
            Some(error) => Err(error.clone()),
            None => Ok(()),
        }
    }
}
