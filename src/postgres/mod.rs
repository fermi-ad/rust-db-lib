//! Postgres Module
//!
//! Contains implementations of the core abstractions designed to interact with a PostgreSQL database instance.

use super::{DataRow, DataStore, DataStoreError, DataVal, ParameterizedQuery, QueryParameter};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use rust_env_var_lib::env_var;
use sqlx::{
    Decode, Error, Postgres, Row, Value, ValueRef,
    postgres::{PgArguments, PgConnectOptions, PgPool, PgPoolOptions, PgRow, PgValue, PgValueRef},
    query::Query,
};
use std::time::Duration;
use tracing::error;

const FAILED_CONVERSION_MSG: &str = "Invalid data conversion. See error log for details";
impl From<Box<dyn std::error::Error + Send + Sync>> for DataStoreError {
    fn from(err: Box<dyn std::error::Error + Send + Sync>) -> Self {
        error!("{:?}", err);
        Self {
            details: String::from(FAILED_CONVERSION_MSG),
        }
    }
}
const GENERIC_ERR_MSG: &str =
    "An error occurred while interacting with the database. See error log for details";
impl From<Error> for DataStoreError {
    fn from(err: Error) -> Self {
        error!("{:?}", err);
        Self {
            details: String::from(GENERIC_ERR_MSG),
        }
    }
}

/// Postgres implementation of the [`DataVal`] trait.
pub struct PostgresDataVal {
    column_data: Result<PgValue, DataStoreError>,
}
impl PostgresDataVal {
    fn decode<'a, T: Decode<'a, Postgres>>(value: PgValueRef<'a>) -> Result<T, DataStoreError> {
        T::decode(value).map_err(DataStoreError::from)
    }
}
impl DataVal for PostgresDataVal {
    fn to_bool(self) -> Result<bool, DataStoreError> {
        match self.column_data {
            Ok(value) => Self::decode(value.as_ref()),
            Err(err) => Err(err),
        }
    }

    fn to_bool_optional(self) -> Result<Option<bool>, DataStoreError> {
        match self.column_data {
            Ok(value) => Self::decode(value.as_ref()),
            Err(err) => Err(err),
        }
    }

    fn to_datetime(self) -> Result<DateTime<Utc>, DataStoreError> {
        match self.column_data {
            Ok(value) => Self::decode(value.as_ref()),
            Err(err) => Err(err),
        }
    }

    fn to_datetime_optional(self) -> Result<Option<DateTime<Utc>>, DataStoreError> {
        match self.column_data {
            Ok(value) => Self::decode(value.as_ref()),
            Err(err) => Err(err),
        }
    }

    fn to_i8(self) -> Result<i8, DataStoreError> {
        match self.column_data {
            Ok(value) => Self::decode(value.as_ref()),
            Err(err) => Err(err),
        }
    }

    fn to_i8_optional(self) -> Result<Option<i8>, DataStoreError> {
        match self.column_data {
            Ok(value) => Self::decode(value.as_ref()),
            Err(err) => Err(err),
        }
    }

    fn to_i16(self) -> Result<i16, DataStoreError> {
        match self.column_data {
            Ok(value) => Self::decode(value.as_ref()),
            Err(err) => Err(err),
        }
    }

    fn to_i16_optional(self) -> Result<Option<i16>, DataStoreError> {
        match self.column_data {
            Ok(value) => Self::decode(value.as_ref()),
            Err(err) => Err(err),
        }
    }

    fn to_i32(self) -> Result<i32, DataStoreError> {
        match self.column_data {
            Ok(value) => Self::decode(value.as_ref()),
            Err(err) => Err(err),
        }
    }

    fn to_i32_optional(self) -> Result<Option<i32>, DataStoreError> {
        match self.column_data {
            Ok(value) => Self::decode(value.as_ref()),
            Err(err) => Err(err),
        }
    }

    fn to_i64(self) -> Result<i64, DataStoreError> {
        match self.column_data {
            Ok(value) => Self::decode(value.as_ref()),
            Err(err) => Err(err),
        }
    }

    fn to_i64_optional(self) -> Result<Option<i64>, DataStoreError> {
        match self.column_data {
            Ok(value) => Self::decode(value.as_ref()),
            Err(err) => Err(err),
        }
    }

    fn to_f32(self) -> Result<f32, DataStoreError> {
        match self.column_data {
            Ok(value) => Self::decode(value.as_ref()),
            Err(err) => Err(err),
        }
    }

    fn to_f32_optional(self) -> Result<Option<f32>, DataStoreError> {
        match self.column_data {
            Ok(value) => Self::decode(value.as_ref()),
            Err(err) => Err(err),
        }
    }

    fn to_f64(self) -> Result<f64, DataStoreError> {
        match self.column_data {
            Ok(value) => Self::decode(value.as_ref()),
            Err(err) => Err(err),
        }
    }

    fn to_f64_optional(self) -> Result<Option<f64>, DataStoreError> {
        match self.column_data {
            Ok(value) => Self::decode(value.as_ref()),
            Err(err) => Err(err),
        }
    }

    fn to_string(self) -> Result<String, DataStoreError> {
        match self.column_data {
            Ok(value) => Self::decode(value.as_ref()),
            Err(err) => Err(err),
        }
    }

    fn to_string_optional(self) -> Result<Option<String>, DataStoreError> {
        match self.column_data {
            Ok(value) => Self::decode(value.as_ref()),
            Err(err) => Err(err),
        }
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

/// Postgres implementation of the [`DataStore`] trait
pub struct PostgresDataStore {
    db_pool: PgPool,
}
impl PostgresDataStore {
    async fn establish_connection_pool() -> PgPool {
        let host: String = env_var::get("DATABASE_HOST")
            .to_option()
            .expect("DATABASE_HOST must be set");

        let port = env_var::get("DATABASE_PORT").or(5432_u16);

        let username: String = env_var::get("DATABASE_USER")
            .to_option()
            .expect("DATABASE_USER must be set");
        let password: String = env_var::get("DATABASE_PASS")
            .to_option()
            .expect("DATABASE_PASS must be set");

        let db_name: String = env_var::get("DATABASE_NAME")
            .to_option()
            .expect("DATABASE_NAME must be set");

        let connection_config = PgConnectOptions::new()
            .host(host.as_str())
            .port(port)
            .username(username.as_str())
            .password(password.as_str())
            .database(db_name.as_str())
            .ssl_mode(sqlx::postgres::PgSslMode::Require);

        PgPoolOptions::new()
            .max_connections(5)
            .acquire_timeout(Duration::from_secs(10))
            .connect_with(connection_config)
            .await
            .expect("Failed to connect to database...")
    }

    /// Creates a new instance of the `PostgresDataStore` with an established connection pool.
    pub async fn new() -> Self {
        Self {
            db_pool: Self::establish_connection_pool().await,
        }
    }

    async fn run_query<'a>(
        &'a self,
        query: Query<'a, Postgres, PgArguments>,
    ) -> Result<Vec<PostgresDataRow>, DataStoreError> {
        let query_result = query.fetch_all(&self.db_pool).await;
        match query_result {
            Ok(rows) => Ok(rows.into_iter().map(PostgresDataRow::from).collect()),
            Err(e) => Err(DataStoreError::from(e)),
        }
    }
}
impl Clone for PostgresDataStore {
    fn clone(&self) -> Self {
        Self {
            db_pool: self.db_pool.clone(),
        }
    }
}
#[async_trait]
impl DataStore<PostgresDataVal, PostgresDataRow> for PostgresDataStore {
    async fn execute_query(&self, query: &str) -> Result<Vec<PostgresDataRow>, DataStoreError> {
        self.run_query(sqlx::query(query)).await
    }

    async fn execute_parameterized_query(
        &self,
        parameterized_query: ParameterizedQuery,
    ) -> Result<Vec<PostgresDataRow>, DataStoreError> {
        let mut query_builder = sqlx::query(parameterized_query.statement.as_str());
        for parameter in parameterized_query.bindings {
            query_builder = match parameter {
                QueryParameter::BOOL(val) => query_builder.bind(val),
                QueryParameter::DATETIME(val) => query_builder.bind(val),
                QueryParameter::I8(val) => query_builder.bind(val),
                QueryParameter::I16(val) => query_builder.bind(val),
                QueryParameter::I32(val) => query_builder.bind(val),
                QueryParameter::I64(val) => query_builder.bind(val),
                QueryParameter::F32(val) => query_builder.bind(val),
                QueryParameter::F64(val) => query_builder.bind(val),
                QueryParameter::STR(val) => query_builder.bind(val),
            }
        }
        self.run_query(query_builder).await
    }
}
