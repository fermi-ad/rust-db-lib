//! Postgres Module
//!
//! Contains implementations of the core abstractions designed to interact with a PostgreSQL database instance.

use super::{DataRow, DataStore, DataStoreError, DataVal, ParameterizedQuery, QueryParameter};
use chrono::{DateTime, Utc};
use sqlx::{
    Decode, Error, Postgres, Row, Value, ValueRef,
    postgres::{PgConnectOptions, PgPool, PgPoolOptions, PgRow, PgSslMode, PgValue},
};
use std::{borrow::Cow, time::Duration};

impl From<Error> for DataStoreError {
    fn from(value: Error) -> Self {
        DataStoreError {
            details: format!("{value:?}"),
        }
    }
}

/// Controls whether and how TLS is used when connecting to the database.
pub enum SslMode {
    /// Attempt a TLS connection first; fall back to plaintext if the server does not support TLS.
    /// Suitable for local development and environments without TLS configured.
    Prefer,
    /// Require a TLS connection. The connection will fail if the server does not support TLS.
    /// Server certificates are not verified.
    Require,
    /// Require a TLS connection and verify that the server certificate is signed by a trusted CA.
    VerifyCa,
    /// Require a TLS connection, verify the server certificate against a trusted CA,
    /// and verify that the server hostname matches the certificate.
    VerifyFull,
}
impl From<SslMode> for PgSslMode {
    fn from(value: SslMode) -> Self {
        match value {
            SslMode::Prefer => PgSslMode::Prefer,
            SslMode::Require => PgSslMode::Require,
            SslMode::VerifyCa => PgSslMode::VerifyCa,
            SslMode::VerifyFull => PgSslMode::VerifyFull,
        }
    }
}

/// Configuration for establishing a [`PostgresDataStore`] connection pool.
///
/// Fields that are not specified will use the values from [`PostgresConfig::default()`].
///
/// # Example
/// ```rust,ignore
/// use rust_db_lib::postgres::{PostgresConfig, SslMode};
///
/// let config = PostgresConfig {
///     host: "localhost".to_string(),
///     username: "myuser".to_string(),
///     password: "mypassword".to_string(),
///     db_name: "mydb".to_string(),
///     ..PostgresConfig::default()
/// };
/// ```
pub struct PostgresConfig {
    /// The hostname or IP address of the PostgreSQL server.
    pub host: String,
    /// The port the PostgreSQL server is listening on. Defaults to `5432`.
    pub port: u16,
    /// The username to authenticate with.
    pub username: String,
    /// The password to authenticate with.
    pub password: String,
    /// The name of the database to connect to.
    pub db_name: String,
    /// The SSL mode to use for the connection. Defaults to [`SslMode::Require`].
    pub ssl_mode: SslMode,
    /// The maximum number of connections to maintain in the pool. Defaults to `5`.
    pub max_connections: u32,
    /// The maximum time to wait when acquiring a connection from the pool. Defaults to 10 seconds.
    pub connection_timeout: Duration,
}
impl Default for PostgresConfig {
    fn default() -> Self {
        Self {
            host: String::new(),
            port: 5432,
            username: String::new(),
            password: String::new(),
            db_name: String::new(),
            ssl_mode: SslMode::Require,
            max_connections: 5,
            connection_timeout: Duration::from_secs(10),
        }
    }
}

/// Postgres implementation of the [`DataVal`] trait.
pub struct PostgresDataVal {
    column_data: Result<PgValue, DataStoreError>,
}
impl PostgresDataVal {
    fn translate_column_data<'a, T: Decode<'a, Postgres>>(&'a self) -> Result<T, DataStoreError> {
        match self.column_data.as_ref() {
            Ok(value) => T::decode(value.as_ref()).map_err(|e| DataStoreError {
                details: format!("{e:?}"),
            }),
            Err(err) => Err(err.clone()),
        }
    }
}
impl DataVal for PostgresDataVal {
    fn to_bool(self) -> Result<bool, DataStoreError> {
        self.translate_column_data()
    }

    fn to_bool_optional(self) -> Result<Option<bool>, DataStoreError> {
        self.translate_column_data()
    }

    fn to_datetime(self) -> Result<DateTime<Utc>, DataStoreError> {
        self.translate_column_data()
    }

    fn to_datetime_optional(self) -> Result<Option<DateTime<Utc>>, DataStoreError> {
        self.translate_column_data()
    }

    fn to_i8(self) -> Result<i8, DataStoreError> {
        self.translate_column_data()
    }

    fn to_i8_optional(self) -> Result<Option<i8>, DataStoreError> {
        self.translate_column_data()
    }

    fn to_i16(self) -> Result<i16, DataStoreError> {
        self.translate_column_data()
    }

    fn to_i16_optional(self) -> Result<Option<i16>, DataStoreError> {
        self.translate_column_data()
    }

    fn to_i32(self) -> Result<i32, DataStoreError> {
        self.translate_column_data()
    }

    fn to_i32_optional(self) -> Result<Option<i32>, DataStoreError> {
        self.translate_column_data()
    }

    fn to_i64(self) -> Result<i64, DataStoreError> {
        self.translate_column_data()
    }

    fn to_i64_optional(self) -> Result<Option<i64>, DataStoreError> {
        self.translate_column_data()
    }

    fn to_f32(self) -> Result<f32, DataStoreError> {
        self.translate_column_data()
    }

    fn to_f32_optional(self) -> Result<Option<f32>, DataStoreError> {
        self.translate_column_data()
    }

    fn to_f64(self) -> Result<f64, DataStoreError> {
        self.translate_column_data()
    }

    fn to_f64_optional(self) -> Result<Option<f64>, DataStoreError> {
        self.translate_column_data()
    }

    fn to_string(self) -> Result<String, DataStoreError> {
        self.translate_column_data()
    }

    fn to_string_optional(self) -> Result<Option<String>, DataStoreError> {
        self.translate_column_data()
    }
}

/// Represents a single row retrieved from a Postgres database
/// implementing the [`DataRow`] trait. In this case, it wraps [`sqlx::postgres::PgRow`]
/// to provide the necessary methods.
pub struct PostgresDataRow {
    row: PgRow,
}
impl From<PgRow> for PostgresDataRow {
    fn from(row: PgRow) -> Self {
        Self { row }
    }
}
impl DataRow<PostgresDataVal> for PostgresDataRow {
    fn get(&self, column_name: &str) -> PostgresDataVal {
        let column_data = match self.row.try_get_raw(column_name) {
            Ok(val) => Ok(ValueRef::to_owned(&val)),
            Err(err) => Err(DataStoreError::from(err)),
        };
        PostgresDataVal { column_data }
    }
}

/// Postgres implementation of the [`DataStore`] trait.
///
/// Use [`PostgresDataStore::new()`] with a [`PostgresConfig`] to establish a connection pool.
#[derive(Clone)]
pub struct PostgresDataStore {
    db_pool: PgPool,
}
impl PostgresDataStore {
    /// Creates a new instance of [`PostgresDataStore`] with an established connection pool.
    ///
    /// Returns a [`DataStoreError`] if the connection pool cannot be established
    /// (e.g. the server is unreachable, credentials are invalid, or the timeout is exceeded).
    pub async fn new(config: PostgresConfig) -> Result<Self, DataStoreError> {
        let connection_config = PgConnectOptions::new()
            .host(config.host.as_str())
            .port(config.port)
            .username(config.username.as_str())
            .password(config.password.as_str())
            .database(config.db_name.as_str())
            .ssl_mode(PgSslMode::from(config.ssl_mode));

        PgPoolOptions::new()
            .max_connections(config.max_connections)
            .acquire_timeout(config.connection_timeout)
            .connect_with(connection_config)
            .await
            .map(|db_pool| Self { db_pool })
            .map_err(DataStoreError::from)
    }
}
impl DataStore<PostgresDataVal, PostgresDataRow> for PostgresDataStore {
    async fn execute_query(
        &self,
        query: impl Into<Cow<'static, str>> + Send,
    ) -> Result<Vec<PostgresDataRow>, DataStoreError> {
        sqlx::query(sqlx::AssertSqlSafe(query.into()))
            .fetch_all(&self.db_pool)
            .await
            .map(|rows| rows.into_iter().map(PostgresDataRow::from).collect())
            .map_err(DataStoreError::from)
    }

    async fn execute_parameterized_query(
        &self,
        parameterized_query: ParameterizedQuery,
    ) -> Result<Vec<PostgresDataRow>, DataStoreError> {
        let mut query_builder = sqlx::query(sqlx::AssertSqlSafe(parameterized_query.statement));
        for parameter in parameterized_query.bindings {
            query_builder = match parameter {
                QueryParameter::Bool(val) => query_builder.bind(val),
                QueryParameter::DateTime(val) => query_builder.bind(val),
                QueryParameter::I8(val) => query_builder.bind(val),
                QueryParameter::I16(val) => query_builder.bind(val),
                QueryParameter::I32(val) => query_builder.bind(val),
                QueryParameter::I64(val) => query_builder.bind(val),
                QueryParameter::F32(val) => query_builder.bind(val),
                QueryParameter::F64(val) => query_builder.bind(val),
                QueryParameter::Str(val) => query_builder.bind(val),
            }
        }
        query_builder
            .fetch_all(&self.db_pool)
            .await
            .map(|rows| rows.into_iter().map(PostgresDataRow::from).collect())
            .map_err(DataStoreError::from)
    }
}
