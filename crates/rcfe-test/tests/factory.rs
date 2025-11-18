mod common;

use common::get_client;
use rcfe::{Client, KVClient};

/// Test client factory by performing a KV operation
/// # Test keys
/// - "foo" -> "bar"
/// - "greeting" -> "Hello, etcd"
/// - "greetinh" -> "Hello, etcd"
/// - "greetini" -> "Hello, etcd"
#[tokio::test]
async fn test_factory() -> Result<(), rcfe::Error> {
    let client = get_client().await?;

    // Use the client to perform a KV operation
    let mut kv_client = client.get_kv_client();

    // Get the value for a specific key
    let response = kv_client.range_with_str("greeting").await?;

    let kvs = response.into_inner().kvs;

    let expected = ("greeting".as_bytes(), "Hello, etcd".as_bytes());

    assert_eq!(kvs.len(), 1);

    assert_eq!(kvs[0].key, expected.0);
    assert_eq!(kvs[0].value, expected.1);

    Ok(())
}
