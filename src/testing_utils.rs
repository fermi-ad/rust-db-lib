//! Mock [`crate::DataStore`] implementations for tests (requires the `testing-utils` feature).
//!
//! Use [`test_data_store!`](crate::test_data_store) to build rows from column/value pairs,
//! or construct [`TestRow`](crate::testing_utils::TestRow) and
//! [`TestVal`](crate::testing_utils::TestVal) directly.
//! [`TestDataStore`](crate::testing_utils::TestDataStore) always returns its configured rows,
//! regardless of the query; inspect
//! [`captured_operations`](crate::testing_utils::TestDataStore::captured_operations) to verify
//! query construction.

pub use crate::test_data_store;
pub use test_data_store::TestDataStore;
pub use test_row::TestRow;
pub use test_transaction::TestTransaction;
pub use test_val::TestVal;

use std::{borrow::Cow, sync::Mutex};

use crate::{DataStoreError, ParameterizedQuery, TransactionError};

mod store_constructor_macro;
mod test_data_store;
mod test_row;
mod test_transaction;
mod test_val;

#[cfg(test)]
mod tests;

/// A single operation captured by a [`TestDataStore`], in the form it was passed to the store.
#[derive(Clone, Debug, PartialEq)]
pub enum Operation {
    /// A query passed to [`execute_query`](crate::DataStore::execute_query).
    Query(Cow<'static, str>),
    /// A query passed to [`execute_parameterized_query`](crate::DataStore::execute_parameterized_query).
    ParameterizedQuery(ParameterizedQuery),
    /// A transaction was opened.
    TransactionBegin,
    /// A transaction was committed.
    TransactionCommit,
    /// A transaction was rolled back.
    TransactionRollback,
}

#[derive(Clone, Debug, Default)]
pub(crate) struct TestErrors {
    pub(crate) query: Option<DataStoreError>,
    pub(crate) transaction_query: Option<TransactionError>,
    pub(crate) begin: Option<TransactionError>,
    pub(crate) commit: Option<TransactionError>,
    pub(crate) rollback: Option<TransactionError>,
}

#[derive(Debug)]
pub(crate) struct TestDataInner {
    pub(crate) data: Vec<TestRow>,
    pub(crate) operations: Mutex<Vec<Operation>>,
    pub(crate) errors: TestErrors,
}
impl TestDataInner {
    pub(crate) fn new(data: Vec<TestRow>) -> Self {
        Self {
            data,
            operations: Mutex::new(Vec::new()),
            errors: TestErrors::default(),
        }
    }
}
impl Clone for TestDataInner {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            operations: Mutex::new(self.operations.lock().unwrap().clone()),
            errors: self.errors.clone(),
        }
    }
}
