use skynet_types::*;

#[test]
fn test_id_hash() {
    let data = b"test data";
    let auth_id = AuthId::hash(data);
    let user_id = UserId::hash(data);

    assert_eq!(auth_id.as_bytes(), user_id.as_bytes());
}

#[test]
fn test_id_hex() {
    let data = b"test data";
    let auth_id = AuthId::hash(data);
    let hex = auth_id.to_hex();
    let decoded = AuthId::from_hex(&hex).unwrap();

    assert_eq!(auth_id, decoded);
}

#[test]
fn test_subnet_mainnet() {
    let mainnet = SubnetId::mainnet();
    assert!(mainnet.is_mainnet());

    let other = SubnetId::hash(b"other");
    assert!(!other.is_mainnet());
}

#[test]
fn test_serde() {
    let auth_id = AuthId::hash(b"test");
    let serialized = serde_json::to_string(&auth_id).unwrap();
    let deserialized: AuthId = serde_json::from_str(&serialized).unwrap();

    assert_eq!(auth_id, deserialized);
}
