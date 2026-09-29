use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

/// Error from a [`DataStore`](crate::DataStore) query or value conversion.
#[derive(Clone, Debug)]
pub struct DataStoreError {
    pub(crate) details: String,
}
impl Display for DataStoreError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "DataStoreError: {}", self.details)
    }
}
impl Error for DataStoreError {}

/// Failure opening or completing a transaction.
#[derive(Clone, Debug)]
pub enum TransactionError {
    /// The transaction conflicted with concurrent work and may succeed if retried.
    Retryable,
    /// Any other transaction failure.
    DatabaseError(DataStoreError),
}
impl Display for TransactionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Retryable => write!(f, "transaction conflict (retryable)"),
            Self::DatabaseError(error) => write!(f, "{error}"),
        }
    }
}
impl Error for TransactionError {}
