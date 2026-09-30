// The unused dependency checker will flag chrono and sqlx as unused for this
// integration test crate. Unused dependencies in tests are not a big concern,
// as we will still use them in the main binary.
#![allow(unused_crate_dependencies)]

use rust_db_lib::{DataRow, DataStore, DataVal, testing_utils::test_data_store};

#[tokio::test]
async fn consumer_can_use_testing_utils_macro() {
    let store = test_data_store!([
        [("name", "Ada"), ("active", true), ("nickname", None)],
        [("name", String::from("Grace")), ("active", false)],
    ]);

    let rows = store.execute_query("SELECT * FROM people").await.unwrap();
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0].get("name").to_string().unwrap(), "Ada");
    assert!(rows[0].get("active").to_bool().unwrap());
    assert_eq!(rows[0].get("nickname").to_string_optional().unwrap(), None);
    assert_eq!(rows[1].get("name").to_string().unwrap(), "Grace");
    assert!(!rows[1].get("active").to_bool().unwrap());
    assert!(rows[1].get("nickname").to_string_optional().is_err());
}
