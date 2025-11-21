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

/// Test delete with prefix
/// # Test keys
/// - "greeting" -> "Hello, etcd"
/// - "greetinh" -> "Hello, etcd"
/// - "greetini" -> "Hello, etcd"
#[tokio::test]
async fn test_delete_with_prefix() -> Result<(), Error> {
    let client = get_client().await?;
    let mut kv_client = client.get_kv_client();
    let delete_response = kv_client
        .delete_with_prefix(ByteSequence::from("greet"))
        .await?;
    let delete_range_response = delete_response.into_inner();
    assert_eq!(delete_range_response.deleted, 3);
    Ok(())
}

/// Test delete all
/// # Test keys
/// - (none)
#[tokio::test]
async fn test_delete_all() -> Result<(), Error> {
    let client = get_client().await?;
    let mut kv_client = client.get_kv_client();
    let delete_response = kv_client.delete_all().await?;
    let delete_range_response = delete_response.into_inner();
    assert!(delete_range_response.deleted >= 0);
    Ok(())
}

/// Test build delete request
#[tokio::test]
async fn test_build_delete_request() -> Result<(), Error> {
    let client = get_client().await?;
    let kv_client = client.get_kv_client();
    let key = ByteSequence::from("test_key");
    let request = kv_client.build_delete_request(key.clone(), None, None);
    assert_eq!(request.key, key.as_bytes());
    assert_eq!(request.range_end, vec![]);
    assert_eq!(request.prev_kv, false);
    Ok(())
}

/// Test build compact request
#[tokio::test]
async fn test_build_compact_request() -> Result<(), Error> {
    let client = get_client().await?;
    let kv_client = client.get_kv_client();
    let revision: i64 = 42;
    let request = kv_client.build_compact_request(revision, None);
    assert_eq!(request.revision, revision);
    assert_eq!(request.physical, false);
    Ok(())
}

/// Test compact with request
/// ### Tips
/// - Get the current revision before compaction to ensure the target revision exists.
/// - Use range any key (can be an empty key) to get the current revision.
#[tokio::test]
async fn test_compact_with_request() -> Result<(), Error> {
    let client = get_client().await?;
    let mut kv_client = client.get_kv_client();

    let response = kv_client.range(ByteSequence::from("rockfe")).await?;
    let current_revision = response.into_inner().header.unwrap().revision;
    let compact_request = kv_client.build_compact_request(current_revision, None);
    let response = kv_client.compact_with_request(compact_request).await?;
    let _compact_response = response.into_inner();

    Ok(())
}

/// Test compact
/// ### Tips
/// - Get the current revision before compaction to ensure the target revision exists.
/// - Use range any key (can be an empty key) to get the current revision.
#[tokio::test]
async fn test_compact() -> Result<(), Error> {
    let client = get_client().await?;
    let mut kv_client = client.get_kv_client();

    // put an key to ensure there is at least one revision
    let put_response = kv_client
        .put(ByteSequence::from("rockfe"), ByteSequence::from("rocks"))
        .await?;
    let _put_resp = put_response.into_inner();

    let response = kv_client.range(ByteSequence::from("rockfe")).await?;
    let current_revision = response.into_inner().header.unwrap().revision;
    let response = kv_client.compact(current_revision).await?;
    let _compact_response = response.into_inner();

    // then delete the key
    let delete_response = kv_client.delete(ByteSequence::from("rockfe")).await?;
    let _delete_resp = delete_response.into_inner();

    Ok(())
}