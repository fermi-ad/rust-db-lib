use super::{DataRow, DataStore, DataStoreError, DataVal, ParameterizedQuery};
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

    fn to_datetime(self) -> Result<DateTime<Utc>, DataStoreError> {
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

    fn to_i16(self) -> Result<i16, DataStoreError> {
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

    fn to_i64(self) -> Result<i64, DataStoreError> {
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

    fn to_f64(self) -> Result<f64, DataStoreError> {
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

pub struct PostgresParameterizedQuery<'a> {
    query_builder: Query<'a, Postgres, PgArguments>,
    db_pool: &'a PgPool,
}
#[async_trait]
impl<'a> ParameterizedQuery<'a, PostgresDataVal, PostgresDataRow>
    for PostgresParameterizedQuery<'a>
{
    fn bind_bool(&'a mut self, parameter: bool) -> Result<(), DataStoreError> {
        self.query_builder
            .try_bind(parameter)
            .map_err(DataStoreError::from)
    }

    fn bind_datetime(&'a mut self, parameter: DateTime<Utc>) -> Result<(), DataStoreError> {
        self.query_builder
            .try_bind(parameter)
            .map_err(DataStoreError::from)
    }

    fn bind_i8(&'a mut self, parameter: i8) -> Result<(), DataStoreError> {
        self.query_builder
            .try_bind(parameter)
            .map_err(DataStoreError::from)
    }

    fn bind_i16(&'a mut self, parameter: i16) -> Result<(), DataStoreError> {
        self.query_builder
            .try_bind(parameter)
            .map_err(DataStoreError::from)
    }

    fn bind_i32(&'a mut self, parameter: i32) -> Result<(), DataStoreError> {
        self.query_builder
            .try_bind(parameter)
            .map_err(DataStoreError::from)
    }

    fn bind_i64(&'a mut self, parameter: i64) -> Result<(), DataStoreError> {
        self.query_builder
            .try_bind(parameter)
            .map_err(DataStoreError::from)
    }

    fn bind_f32(&'a mut self, parameter: f32) -> Result<(), DataStoreError> {
        self.query_builder
            .try_bind(parameter)
            .map_err(DataStoreError::from)
    }

    fn bind_f64(&'a mut self, parameter: f64) -> Result<(), DataStoreError> {
        self.query_builder
            .try_bind(parameter)
            .map_err(DataStoreError::from)
    }

    fn bind_string(&'a mut self, parameter: String) -> Result<(), DataStoreError> {
        self.query_builder
            .try_bind(parameter)
            .map_err(DataStoreError::from)
    }

    async fn execute(self) -> Result<Vec<PostgresDataRow>, DataStoreError> {
        let query_result = self.query_builder.fetch_all(self.db_pool).await;
        match query_result {
            Ok(rows) => Ok(rows.into_iter().map(PostgresDataRow::from).collect()),
            Err(e) => Err(DataStoreError::from(e)),
        }
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
            .database(db_name.as_str());

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
}
impl Clone for PostgresDataStore {
    fn clone(&self) -> Self {
        Self {
            db_pool: self.db_pool.clone(),
        }
    }
}
#[async_trait]
impl<'a> DataStore<'a, PostgresDataVal, PostgresDataRow, PostgresParameterizedQuery<'a>>
    for PostgresDataStore
{
    async fn execute_query(
        &'a self,
        query: &'a str,
    ) -> Result<Vec<PostgresDataRow>, DataStoreError> {
        let query_result = sqlx::query(query).fetch_all(&self.db_pool).await;
        match query_result {
            Ok(rows) => Ok(rows.into_iter().map(PostgresDataRow::from).collect()),
            Err(e) => Err(DataStoreError::from(e)),
        }
    }
    fn init_parameterized_query(&'a self, query: &'a str) -> PostgresParameterizedQuery<'a> {
        let query_builder = sqlx::query(query);
        PostgresParameterizedQuery {
            query_builder,
            db_pool: &self.db_pool,
        }
    }
}
