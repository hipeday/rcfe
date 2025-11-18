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
