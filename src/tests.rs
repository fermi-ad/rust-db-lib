use super::*;

#[test]
fn test_display_datastore_error() {
    let error = DataStoreError {
        details: "Test error".to_string(),
    };
    assert_eq!(format!("{}", error), "DataStoreError: Test error");
}

#[test]
fn test_parameterized_query_new_and_bind() {
    let mut param_query = ParameterizedQuery::new("SELECT * FROM table WHERE id = $1");
    assert_eq!(param_query.statement, "SELECT * FROM table WHERE id = $1");
    assert!(param_query.bindings.is_empty());

    let test_param = QueryParameter::I32(42);
    param_query.bind(test_param.clone());
    assert_eq!(param_query.bindings.len(), 1);
    assert_eq!(param_query.bindings[0], test_param);
}
