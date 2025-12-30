# rust-db-lib
This is a library for connecting to a database from within a Rust app. It encapsulates the specifics of the DB connection logic, exposing DB access through a consistent interface. The intention is that all Rust apps import this library as a dependency when they need access to the DB, so necessary changes to how our services interact with the DB can be managed from one place.

## Interface 
The primary abstraction provided by this library is the `DataStore<T: DataVal, U: DataRow<T>>` trait. It exposes a predefined set of methods for interacting with a database and the results of querying the database. Details can be found in the rustdoc that accompanies the trait.

#### Supported implementations
The following implementations are provided for connecting to the DB.
- `postgres::PostgresDataVal` - Implements `DataVal`
- `postgres::PostgresDataRow` - Implements `DataRow<PostgresDataVal>`
- `postgres::PostgresDataStore` - Implements `DataStore<PostgresDataVal, PostgresDataRow>`

#### Required environment variables
For this lib to operate successfully, the following environment variables must be set:
- `DATABASE_HOST` - The host of the database, e.g. `localhost`, `10.32.12.53`, `fermi-db.fnal.gov`, etc.
- `DATABASE_PORT` - The port to use on the host. Must parse to an unsigned 16-bit integer.
- `DATABASE_USER` - The username to use when connecting to the database.
- `DATABASE_PASS` - The password for the desired user.
- `DATABASE_NAME` - The name of the database being connected to, e.g. `adbs`

## Docs
The Rust documentation and a getting-started guide can be found [here](https://doc.rust-lang.org/book/title-page.html).