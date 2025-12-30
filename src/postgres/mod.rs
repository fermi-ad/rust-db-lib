use super::{DataRow, DataStore, DataStoreError, DataVal};
use async_trait::async_trait;
use sqlx::{
    Decode, Error, Postgres, Row, Value, ValueRef,
    postgres::{PgConnectOptions, PgPool, PgPoolOptions, PgRow, PgValue, PgValueRef},
};
use std::{env, time::Duration};
use tracing::error;

const FAILED_CONVERSION_MSG: &str =
    "Failed to convert the results to the desired type. See error log for details";
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

/// Postgres implementation of the [`DataStore`] trait
pub struct PostgresDataStore {
    db_pool: PgPool,
}
impl PostgresDataStore {
    async fn establish_connection_pool() -> PgPool {
        let host = env::var("DATABASE_HOST").expect("DATABASE_URL must be set");

        let port_str = env::var("DATABASE_PORT").expect("DATABASE_PORT must be set");
        let port: u16 = str::parse(port_str.as_str()).expect("Could not parse DB port into a u16");

        let username = env::var("DATABASE_USER").expect("DATABASE_USER must be set");
        let password = env::var("DATABASE_PASS").expect("DATABASE_PASS must be set");

        let db_name = env::var("DATABASE_NAME").expect("DATABASE_NAME must be set");

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

/// Postgres implementation of the [`DataVal`] trait.
struct PostgresDataVal {
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

    fn to_datetime(self) -> Result<chrono::DateTime<chrono::Utc>, DataStoreError> {
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

/// Encapsulates the execution of queries against a Postgres database.
/// Returns results as [`PostgresDataRow`] instances.
#[async_trait]
impl DataStore<PostgresDataVal, PostgresDataRow> for PostgresDataStore {
    async fn execute_query(&self, query: String) -> Result<Vec<PostgresDataRow>, DataStoreError> {
        let query_result = sqlx::query(query.as_str()).fetch_all(&self.db_pool).await;
        match query_result {
            Ok(rows) => Ok(rows.into_iter().map(PostgresDataRow::from).collect()),
            Err(e) => {
                error!("Query failed: {}", e);
                Err(DataStoreError {
                    details: "Query execution failed. See system logs for details.".to_string(),
                })
            }
        }
    }
    async fn execute_parameterized_query(
        &self,
        query: String,
        bindings: Vec<String>,
    ) -> Result<Vec<PostgresDataRow>, DataStoreError> {
        let mut query_builder = sqlx::query(query.as_str());
        for binding in bindings {
            query_builder = query_builder.bind(binding);
        }
        let query_result = query_builder.fetch_all(&self.db_pool).await;
        match query_result {
            Ok(rows) => Ok(rows.into_iter().map(PostgresDataRow::from).collect()),
            Err(e) => {
                error!("Query failed: {}", e);
                Err(DataStoreError {
                    details: "Query execution failed. See system logs for details.".to_string(),
                })
            }
        }
    }
}
