use std::collections::HashMap;

use crate::{DataRow, testing_utils::TestVal};

/// A test [`DataRow`] backed by a map of column names to [`TestVal`] values.
///
/// Construct it with [`TestRow::new`] or as part of a [`test_data_store!`](super::test_data_store)
/// fixture. [`get`](Self::get) returns [`TestVal::default`] for a missing column; decoding
/// that default value returns an error even through an optional decoder.
#[derive(Clone, Debug, PartialEq)]
pub struct TestRow {
    cols: HashMap<String, TestVal>,
}
impl TestRow {
    pub fn new(cols: HashMap<String, TestVal>) -> Self {
        TestRow { cols }
    }
}
impl DataRow for TestRow {
    type Val = TestVal;

    fn get(&self, column_name: &str) -> Self::Val {
        self.cols.get(column_name).cloned().unwrap_or_default()
    }
}
