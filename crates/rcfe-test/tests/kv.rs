mod common;

use rcfe::{ByteSequence, Client, Error, KVClient};

use common::get_client;

/// Test range query
/// # Test keys
/// - "foo" -> "bar"
/// - "greeting" -> "Hello, etcd"
/// - "greetinh" -> "Hello, etcd"
/// - "greetini" -> "Hello, etcd"
#[tokio::test]
async fn test_range() -> Result<(), Error> {
    let client = get_client().await?;
    let mut kv_client = client.get_kv_client();
    let response = kv_client.range(ByteSequence::from("greeting")).await?;

    let expected = ("greeting".as_bytes(), "Hello, etcd".as_bytes());

    let kvs = response.into_inner().kvs;
    assert_eq!(kvs.len(), 1);
    assert_eq!(kvs[0].key, expected.0);
    assert_eq!(kvs[0].value, expected.1);
    Ok(())
}

/// Test range query with prefix
/// # Test keys
/// - "foo" -> "bar"
/// - "greeting" -> "Hello, etcd"
/// - "greetinh" -> "Hello, etcd"
/// - "greetini" -> "Hello, etcd"
#[tokio::test]
async fn test_range_with_prefix() -> Result<(), Error> {
    let client = get_client().await?;
    let mut kv_client = client.get_kv_client();
    let response = kv_client
        .range_with_prefix(ByteSequence::from("greet"))
        .await?;
    let kvs = response.into_inner().kvs;
    assert_eq!(kvs.len(), 3);

    let expected = vec![
        ("greeting".as_bytes(), "Hello, etcd".as_bytes()),
        ("greetinh".as_bytes(), "Hello, etcd".as_bytes()),
        ("greetini".as_bytes(), "Hello, etcd".as_bytes()),
    ];

    for (i, kv) in kvs.iter().enumerate() {
        assert_eq!(kv.key, expected[i].0);
        assert_eq!(kv.value, expected[i].1);
    }

    Ok(())
}

/// Test range query with string key
/// # Test keys
/// - "foo" -> "bar"
/// - "greeting" -> "Hello, etcd"
/// - "greetinh" -> "Hello, etcd"
/// - "greetini" -> "Hello, etcd"
#[tokio::test]
async fn test_range_with_str() -> Result<(), Error> {
    let client = get_client().await?;

    let mut kv_client = client.get_kv_client();
    let response = kv_client.range_with_str("greeting").await?;
    let kvs = response.into_inner().kvs;

    let expected = ("greeting".as_bytes(), "Hello, etcd".as_bytes());

    assert_eq!(kvs.len(), 1);
    assert_eq!(kvs[0].key, expected.0);
    assert_eq!(kvs[0].value, expected.1);
    Ok(())
}

/// Test range query with explicit end key
///
/// # Test keys
///
/// - "foo" -> "bar"
/// - "greeting" -> "Hello, etcd"
/// - "greetinh" -> "Hello, etcd"
/// - "greetini" -> "Hello, etcd"
#[tokio::test]
async fn test_range_with_end() -> Result<(), Error> {
    let client = get_client().await?;

    let mut kv_client = client.get_kv_client();
    let response = kv_client
        .range_with_end(
            ByteSequence::from("greeting"),
            ByteSequence::from("greetini"),
        )
        .await?;

    let kvs = response.into_inner().kvs;
    assert_eq!(kvs.len(), 2);

    let expected = vec![
        ("greeting".as_bytes(), "Hello, etcd".as_bytes()),
        ("greetinh".as_bytes(), "Hello, etcd".as_bytes()),
    ];

    for (i, kv) in kvs.iter().enumerate() {
        assert_eq!(kv.key, expected[i].0);
        assert_eq!(kv.value, expected[i].1);
    }

    Ok(())
}

/// Test range query for all keys
/// # Test keys
/// - "foo" -> "bar"
/// - "greeting" -> "Hello, etcd"
/// - "greetinh" -> "Hello, etcd"
/// - "greetini" -> "Hello, etcd"
#[tokio::test]
async fn test_range_all() -> Result<(), Error> {
    let client = get_client().await?;

    let mut kv_client = client.get_kv_client();

    let response = kv_client.range_all().await?;

    let range_response = response.into_inner();
    let kvs = range_response.kvs;

    let expected = vec![
        ("foo".as_bytes(), "bar".as_bytes()),
        ("greeting".as_bytes(), "Hello, etcd".as_bytes()),
        ("greetinh".as_bytes(), "Hello, etcd".as_bytes()),
        ("greetini".as_bytes(), "Hello, etcd".as_bytes()),
    ];

    assert_eq!(kvs.len(), 4);

    for (i, kv) in kvs.iter().enumerate() {
        assert_eq!(kv.key, expected[i].0);
        assert_eq!(kv.value, expected[i].1);
    }

    Ok(())
}

/// Test range query with options
/// # Test keys
/// - "foo" -> "bar"
/// - "greeting" -> "Hello, etcd"
/// - "greetinh" -> "Hello, etcd"
/// - "greetini" -> "Hello, etcd"
#[tokio::test]
async fn test_range_with_options() -> Result<(), Error> {
    let client = get_client().await?;

    let mut kv_client = client.get_kv_client();

    let response = kv_client
        .range_with_options(
            Some("greeting".into()),
            Some("greetinh".into()),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .await?;

    let kvs = response.into_inner().kvs;
    assert_eq!(kvs.len(), 1);

    let expected = vec![("greeting".as_bytes(), "Hello, etcd".as_bytes())];

    for (i, kv) in kvs.iter().enumerate() {
        assert_eq!(kv.key, expected[i].0);
        assert_eq!(kv.value, expected[i].1);
    }
    Ok(())
}

#[tokio::test]
async fn test_range_with_request() -> Result<(), Error> {
    let client = get_client().await?;

    let mut kv_client = client.get_kv_client();

    let request = kv_client.build_range_request(
        Some(ByteSequence::from("greeting")),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    );

    let response = kv_client.range_with_request(request).await?;

    let kvs = response.into_inner().kvs;
    assert_eq!(kvs.len(), 1);
    let expected = ("greeting".as_bytes(), "Hello, etcd".as_bytes());
    assert_eq!(kvs[0].key, expected.0);
    assert_eq!(kvs[0].value, expected.1);
    Ok(())
}

#[tokio::test]
async fn test_build_put_request() -> Result<(), Error> {
    let client = get_client().await?;

    let kv_client = client.get_kv_client();

    let key = ByteSequence::from("test_key");
    let value = ByteSequence::from("test_value");

    let request = kv_client.build_put_request(key.clone(), value.clone(), None, None, None, None);

    assert_eq!(request.key, key.as_bytes());
    assert_eq!(request.value, value.as_bytes());
    assert_eq!(request.lease, 0);
    assert_eq!(request.prev_kv, false);

    Ok(())
}

/// Test range query with options
/// # Test keys
/// - "foo" -> "bar"
/// - "greeting" -> "Hello, etcd"
/// - "greetinh" -> "Hello, etcd"
/// - "greetini" -> "Hello, etcd"
#[tokio::test]
async fn test_put_with_request() -> Result<(), Error> {
    let client = get_client().await?;

    let mut kv_client = client.get_kv_client();

    let key = ByteSequence::from("rcfe");
    let value = ByteSequence::from("rocks");

    let request = kv_client.build_put_request(key.clone(), value.clone(), None, None, None, None);

    let response = kv_client.put_with_request(request).await?;

    let put_response = response.into_inner();

    // Since the key is new, the previous_kv should be None
    assert!(put_response.prev_kv.is_none());

    // Clean up by deleting the test key
    let response = kv_client.range(key.clone()).await?;

    let kvs = response.into_inner().kvs;
    assert_eq!(kvs.len(), 1);
    assert_eq!(kvs[0].key, key.as_bytes());
    assert_eq!(kvs[0].value, value.as_bytes());
    Ok(())
}

/// Test delete with request
/// # Test keys
/// - "foo" -> "bar"
/// - "greeting" -> "Hello, etcd"
/// - "greetinh" -> "Hello, etcd"
/// - "greetini" -> "Hello, etcd"
/// - "rcfe" -> "rocks"
#[tokio::test]
async fn test_delete_with_request() -> Result<(), Error> {
    let client = get_client().await?;
    let mut kv_client = client.get_kv_client();
    let key = ByteSequence::from("rcfe");
    let delete_request = kv_client.build_delete_request(key.clone(), None, None);
    let delete_response = kv_client.delete_with_request(delete_request).await?;
    let delete_range_response = delete_response.into_inner();
    assert_eq!(delete_range_response.deleted, 1);
    Ok(())
}

/// Test delete
/// # Test keys
/// - "foo" -> "bar"
/// - "greeting" -> "Hello, etcd"
/// - "greetinh" -> "Hello, etcd"
/// - "greetini" -> "Hello, etcd"
#[tokio::test]
async fn test_delete() -> Result<(), Error> {
    let client = get_client().await?;
    let mut kv_client = client.get_kv_client();
    let key = ByteSequence::from("foo");
    let delete_response = kv_client.delete(key.clone()).await?;
    let delete_range_response = delete_response.into_inner();
    assert_eq!(delete_range_response.deleted, 1);
    Ok(())
}