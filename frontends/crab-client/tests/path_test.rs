use crab_client::get_database_path;

#[test]
fn test_get_database_path() {
    let path = get_database_path();
    assert!(path.is_some());
    let path = path.unwrap();
    assert!(path.to_str().unwrap().contains("OpenCrab"));
    assert!(path.to_str().unwrap().contains("opencrab.db"));
}
