use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

/// Error from a [`DataStore`](crate::DataStore) query or value conversion.
#[derive(Clone, Debug)]
pub struct DataStoreError {
    pub(crate) details: String,
}
impl DataStoreError {
    /// Creates an error with a message for testing or custom data-store implementations.
    pub fn new(details: impl Into<String>) -> Self {
        Self {
            details: details.into(),
        }
    }
}
impl From<String> for DataStoreError {
    fn from(details: String) -> Self {
        Self::new(details)
    }
}
impl From<&str> for DataStoreError {
    fn from(details: &str) -> Self {
        Self::new(details)
    }
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
impl From<DataStoreError> for TransactionError {
    fn from(error: DataStoreError) -> Self {
        Self::DatabaseError(error)
    }
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
