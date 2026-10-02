/// Builds a [`TestDataStore`](crate::testing_utils::TestDataStore) from rows of named columns.
///
/// Each inner bracketed list is one row. Column names must be string literals; values may be
/// expressions convertible to [`TestVal`](crate::testing_utils::TestVal) (including booleans,
/// integer and floating-point types, strings, and `chrono::DateTime`). An unsuffixed integer
/// literal is `i32`; use a suffix such as `42_i64` for other widths.
///
/// A literal `None` creates a null column: optional [`DataVal`](crate::DataVal) decoders
/// return `None`, and non-optional decoders return an error. Omitting a column instead makes
/// both optional and non-optional reads error. The data is returned for every query; use
/// [`TestDataStore::captured_operations`](crate::testing_utils::TestDataStore::captured_operations)
/// to check which queries were submitted.
///
/// Requires the `testing-utils` feature. Import as
/// `use rust_db_lib::testing_utils::test_data_store;`.
///
/// # Examples
///
/// ```
/// use rust_db_lib::{DataRow, DataStore, DataVal, testing_utils::test_data_store};
///
/// # async fn example() -> Result<(), rust_db_lib::DataStoreError> {
/// let store = test_data_store!([
///     [("name", "Ada"), ("age", 42), ("nickname", None)],
///     [("name", String::from("Grace")), ("age", 43_i64)],
/// ]);
/// let rows = store.execute_query("SELECT * FROM people").await?;
/// assert_eq!(rows[0].get("name").to_string()?, "Ada");
/// assert_eq!(rows[0].get("nickname").to_string_optional()?, None);
/// assert!(rows[1].get("nickname").to_string_optional().is_err());
/// # Ok(())
/// # }
/// ```
#[macro_export]
macro_rules! test_data_store {
    (@column ($column:literal, None)) => {
        (
            $column.to_string(),
            $crate::testing_utils::TestVal {
                is_nullable: true,
                ..Default::default()
            },
        )
    };
    (@column ($column:literal, $value:expr)) => {
        (
            $column.to_string(),
            $crate::testing_utils::TestVal::from($value),
        )
    };
    ( [ $( [ $( $column_entry:tt ),* $(,)? ] ),* $(,)? ] ) => {
        $crate::testing_utils::TestDataStore::new(vec![
            $(
                $crate::testing_utils::TestRow::new(
                    std::collections::HashMap::from([
                        $( $crate::test_data_store!(@column $column_entry), )*
                    ])
                ),
            )*
        ])
    };
}
