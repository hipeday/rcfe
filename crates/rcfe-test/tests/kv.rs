use rcfe::{ByteSequence, Client, CompactOptions, Compare, CompareResult, CompareTarget, DeleteOptions, Error, GetOptions, KVClient, PutOptions, RequestOp, Txn};
mod common;

use tokio::test;

use common::get_client;

#[test]
async fn test_put() -> Result<(), Error> {
    let client = get_client(None).await?;
    let mut kv_client = client.get_kv_client();

    let key = ByteSequence::from("test_key");
    let value = ByteSequence::from("test_value");

    // Clean up test key if it exists
    let _ = kv_client
        .delete_with_options(key.clone(), Default::default())
        .await;

    let put_response = kv_client.put(key.clone(), value.clone()).await?;

    assert!(put_response.get_ref().header.is_some());

    // Verify that the key was inserted
    let get_response = kv_client.get(key.clone()).await?;
    let kvs = get_response.into_inner().kvs;
    assert_eq!(kvs.len(), 1);
    assert_eq!(kvs[0].key, key.to_vec());
    assert_eq!(kvs[0].value, value.to_vec());

    // Clean up
    let _ = kv_client
        .delete_with_options(key.clone(), Default::default())
        .await;

    Ok(())
}

#[test]
async fn test_put_with_options() -> Result<(), Error> {
    let client = get_client(None).await?;
    let mut kv_client = client.get_kv_client();

    let key = ByteSequence::from("test_key_options");
    let original_value = ByteSequence::from("test_value_options");

    // Clean up test key if it exists
    let _ = kv_client
        .delete_with_options(key.clone(), Default::default())
        .await;

    // Try prev_kv option
    let put_options = PutOptions::builder().prev_kv(true).build();

    let put_response = kv_client
        .put_with_options(key.clone(), original_value.clone(), put_options)
        .await?;
    assert!(put_response.get_ref().header.is_some());
    assert!(put_response.get_ref().prev_kv.is_none());

    // Test updating the key with prev_kv option
    let new_value = ByteSequence::from("new_test_value_options");
    let put_options = PutOptions::builder().prev_kv(true).build();
    let put_response = kv_client
        .put_with_options(key.clone(), new_value.clone(), put_options)
        .await?;

    // Verify that prev_kv is returned and matches the original value
    assert!(put_response.get_ref().header.is_some());
    assert!(put_response.get_ref().prev_kv.is_some());
    assert_eq!(
        put_response.get_ref().prev_kv.as_ref().unwrap().value,
        original_value.to_vec()
    );

    // Test ignore_value option
    let put_options = PutOptions::builder().ignore_value(true).build();
    let put_response = kv_client
        .put_with_options(
            key.clone(),
            ByteSequence::from("ignored_value"),
            put_options,
        )
        .await?;
    assert!(put_response.get_ref().header.is_some());
    // Verify that the value was not changed
    let get_response = kv_client.get(key.clone()).await?;
    let kvs = get_response.into_inner().kvs;
    assert_eq!(kvs.len(), 1);
    assert_eq!(kvs[0].value, new_value.to_vec());

    // Clean up
    let _ = kv_client
        .delete_with_options(key.clone(), Default::default())
        .await;

    // TODO Test ignore_lease option

    Ok(())
}

#[test]
async fn test_get() -> Result<(), Error> {
    let client = get_client(None).await?;
    let mut kv_client = client.get_kv_client();

    let key = ByteSequence::from("test_get_key");
    let value = ByteSequence::from("test_get_value");

    // Clean up test key if it exists
    let _ = kv_client
        .delete_with_options(key.clone(), Default::default())
        .await;

    // Put a key-value pair
    let _ = kv_client.put(key.clone(), value.clone()).await?;

    // Get the key
    let get_response = kv_client.get(key.clone()).await?;
    let kvs = get_response.into_inner().kvs;

    assert_eq!(kvs.len(), 1);
    assert_eq!(kvs[0].key, key.to_vec());
    assert_eq!(kvs[0].value, value.to_vec());

    // Clean up
    let _ = kv_client
        .delete_with_options(key.clone(), Default::default())
        .await;

    Ok(())
}

#[test]
async fn test_get_all() -> Result<(), Error> {
    let client = get_client(None).await?;
    let mut kv_client = client.get_kv_client();

    let key1 = ByteSequence::from("test_get_all_key1");
    let value1 = ByteSequence::from("test_get_all_value1");
    let key2 = ByteSequence::from("test_get_all_key2");
    let value2 = ByteSequence::from("test_get_all_value2");
    let prefix_key = ByteSequence::from("test_get_all_");

    // Clean up test keys if they exist
    let _ = kv_client
        .delete_with_options(
            prefix_key.clone(),
            DeleteOptions::builder().prefix(true).build(),
        )
        .await;

    // Put key-value pairs
    let _ = kv_client.put(key1.clone(), value1.clone()).await?;
    let _ = kv_client.put(key2.clone(), value2.clone()).await?;

    // Get all keys
    let get_response = kv_client.get_all(None).await?;
    let kvs = get_response.into_inner().kvs;

    // Check that our keys are in the result
    let keys: Vec<Vec<u8>> = kvs.iter().map(|kv| kv.key.clone()).collect();
    assert!(keys.contains(&key1.to_vec()));
    assert!(keys.contains(&key2.to_vec()));

    // Clean up
    let _ = kv_client
        .delete_with_options(
            prefix_key.clone(),
            DeleteOptions::builder().prefix(true).build(),
        )
        .await;
    Ok(())
}

#[test]
async fn test_get_with_options() -> Result<(), Error> {
    // This test can be expanded to include various GetOptions scenarios
    let client = get_client(None).await?;
    let mut kv_client = client.get_kv_client();
    let key = ByteSequence::from("test_get_options_key");
    let value = ByteSequence::from("test_get_options_value");

    // Clean up test key if it exists
    let _ = kv_client
        .delete_with_options(key.clone(), Default::default())
        .await;

    // Put a key-value pair
    let _ = kv_client.put(key.clone(), value.clone()).await?;

    // Get the key with default options
    let get_response = kv_client
        .get_with_options(key.clone(), GetOptions::default())
        .await?;
    let kvs = get_response.into_inner().kvs;
    assert_eq!(kvs.len(), 1);
    assert_eq!(kvs[0].key, key.to_vec());
    assert_eq!(kvs[0].value, value.to_vec());

    // Clean up
    let _ = kv_client
        .delete_with_options(key.clone(), Default::default())
        .await;

    // Test prefix option
    let prefix_key1 = ByteSequence::from("test_get_options_prefix_1");
    let prefix_key2 = ByteSequence::from("test_get_options_prefix_2");
    let prefix_value1 = ByteSequence::from("value1");
    let prefix_value2 = ByteSequence::from("value2");

    let prefix_key = ByteSequence::from("test_get_options_prefix_");

    // Clean up prefix keys if they exist
    let _ = kv_client
        .delete_with_options(
            prefix_key.clone(),
            DeleteOptions::builder().prefix(true).build(),
        )
        .await;

    // Put prefix keys
    let _ = kv_client
        .put(prefix_key1.clone(), prefix_value1.clone())
        .await?;
    let _ = kv_client
        .put(prefix_key2.clone(), prefix_value2.clone())
        .await?;

    let get_options = GetOptions::builder().prefix(true).build();
    let get_response = kv_client
        .get_with_options(prefix_key.clone(), get_options)
        .await?;
    let kvs = get_response.into_inner().kvs;
    assert_eq!(kvs.len(), 2);
    let keys: Vec<Vec<u8>> = kvs.iter().map(|kv| kv.key.clone()).collect();
    assert!(keys.contains(&prefix_key1.to_vec()));
    assert!(keys.contains(&prefix_key2.to_vec()));

    // Clean up prefix keys
    let _ = kv_client
        .delete_with_options(
            prefix_key.clone(),
            DeleteOptions::builder().prefix(true).build(),
        )
        .await;

    // Clean up
    let _ = kv_client
        .delete_with_options(key.clone(), Default::default())
        .await;

    // Test range_end option
    let range_key1 = ByteSequence::from("test_get_options_range_1");
    let range_key2 = ByteSequence::from("test_get_options_range_2");
    let range_value1 = ByteSequence::from("value1");
    let range_value2 = ByteSequence::from("value2");

    let range_start_key = ByteSequence::from("test_get_options_range_");
    let range_end_key = ByteSequence::from("test_get_options_range_2");

    // Clean up range keys if they exist
    let _ = kv_client
        .delete_with_options(
            range_start_key.clone(),
            DeleteOptions::builder().prefix(true).build(),
        )
        .await;

    // Put range keys
    let _ = kv_client
        .put(range_key1.clone(), range_value1.clone())
        .await?;
    let _ = kv_client
        .put(range_key2.clone(), range_value2.clone())
        .await?;
    let get_options = GetOptions::builder().end_key(range_end_key.clone()).build();
    let get_response = kv_client
        .get_with_options(range_start_key.clone(), get_options)
        .await?;
    let kvs = get_response.into_inner().kvs;
    assert_eq!(kvs.len(), 1);
    assert_eq!(kvs[0].key, range_key1.to_vec());
    assert_eq!(kvs[0].value, range_value1.to_vec());

    // Clean up range keys
    let _ = kv_client
        .delete_with_options(
            range_start_key.clone(),
            DeleteOptions::builder().prefix(true).build(),
        )
        .await;

    // Test limit option
    let limit_key1 = ByteSequence::from("test_get_options_limit_1");
    let limit_key2 = ByteSequence::from("test_get_options_limit_2");
    let limit_value1 = ByteSequence::from("value1");
    let limit_value2 = ByteSequence::from("value2");
    let limit_key = ByteSequence::from("test_get_options_limit_");

    // Clean up limit keys if they exist
    let _ = kv_client
        .delete_with_options(
            limit_key.clone(),
            DeleteOptions::builder().prefix(true).build(),
        )
        .await;

    // Put limit keys
    let _ = kv_client
        .put(limit_key1.clone(), limit_value1.clone())
        .await?;
    let _ = kv_client
        .put(limit_key2.clone(), limit_value2.clone())
        .await?;

    let get_options = GetOptions::builder().prefix(true).limit(1).build();
    let get_response = kv_client
        .get_with_options(limit_key.clone(), get_options)
        .await?;
    let kvs = get_response.into_inner().kvs;
    assert_eq!(kvs.len(), 1);
    assert!(kvs[0].key == limit_key1.to_vec() || kvs[0].key == limit_key2.to_vec());

    // Clean up limit keys
    let _ = kv_client
        .delete_with_options(
            limit_key.clone(),
            DeleteOptions::builder().prefix(true).build(),
        )
        .await;

    Ok(())
}

#[test]
async fn test_delete() -> Result<(), Error> {
    let client = get_client(None).await?;
    let mut kv_client = client.get_kv_client();

    let key = ByteSequence::from("test_delete_key");
    let value = ByteSequence::from("test_delete_value");

    // Clean up test key if it exists
    let _ = kv_client
        .delete_with_options(key.clone(), Default::default())
        .await;

    // Put a key-value pair
    let _ = kv_client.put(key.clone(), value.clone()).await?;

    // Delete the key
    let delete_response = kv_client.delete(key.clone()).await?;

    assert!(delete_response.get_ref().header.is_some());
    assert_eq!(delete_response.get_ref().deleted, 1);

    // Verify that the key was deleted
    let get_response = kv_client.get(key.clone()).await?;
    let kvs = get_response.into_inner().kvs;
    assert_eq!(kvs.len(), 0);

    Ok(())
}

#[test]
async fn test_delete_with_options() -> Result<(), Error> {
    let client = get_client(None).await?;
    let mut kv_client = client.get_kv_client();

    let key1 = ByteSequence::from("test_delete_options_key1");
    let key2 = ByteSequence::from("test_delete_options_key2");
    let value1 = ByteSequence::from("test_delete_options_value1");
    let value2 = ByteSequence::from("test_delete_options_value2");

    // Clean up test keys if they exist
    let _ = kv_client
        .delete_with_options(
            ByteSequence::from("test_delete_options_"),
            DeleteOptions::builder().prefix(true).build(),
        )
        .await;

    // Put key-value pairs
    let _ = kv_client.put(key1.clone(), value1.clone()).await?;
    let _ = kv_client.put(key2.clone(), value2.clone()).await?;

    // Delete with prefix option
    let delete_options = DeleteOptions::builder().prefix(true).build();
    let delete_response = kv_client
        .delete_with_options(ByteSequence::from("test_delete_options_"), delete_options)
        .await?;

    assert!(delete_response.get_ref().header.is_some());
    assert_eq!(delete_response.get_ref().deleted, 2);

    // Verify that the keys were deleted
    let get_response1 = kv_client.get(key1.clone()).await?;
    let kvs1 = get_response1.into_inner().kvs;
    assert_eq!(kvs1.len(), 0);

    let get_response2 = kv_client.get(key2.clone()).await?;
    let kvs2 = get_response2.into_inner().kvs;
    assert_eq!(kvs2.len(), 0);

    Ok(())
}

#[test]
async fn test_txn() -> Result<(), Error> {
    let client = get_client(None).await?;
    let mut kv_client = client.get_kv_client();
    let key = ByteSequence::from("foo");
    let value = ByteSequence::from("bar");

    // Clean up before test
    let _ = kv_client.delete(key.clone()).await;

    // Compare key 'foo' not exists(version == 0), if true put key 'foo' 'bar', else get key 'foo'
    let compares = vec![Compare::version_eq(key.clone(), 0)];
    let then_ops = vec![RequestOp::Put {
        key: key.clone(),
        value: value.clone(),
        options: None,
    }];
    let otherwise_ops = vec![RequestOp::Get {
        key: key.clone(),
        options: None,
    }];

    // Start and commit the transaction
    let response = kv_client
        .txn()
        .when(compares)?
        .then(then_ops)?
        .otherwise(otherwise_ops)?
        .commit()
        .await?;

    // Assert the transaction was successful
    assert!(response.get_ref().succeeded);

    // Verify the value was set
    let get_response = kv_client.get(key.clone()).await?;
    let kvs = &get_response.get_ref().kvs;
    assert_eq!(kvs.len(), 1);
    assert_eq!(kvs[0].key, key.clone().to_vec());
    assert_eq!(kvs[0].value, value.clone().to_vec());

    // Compare key 'foo' not exists(version == 0), if true put key 'foo' 'bar', else get key 'foo'
    let compares = vec![
        Compare::builder()
            .key(key.clone())
            .version(0)
            .result(CompareResult::Equal)
            .build(),
    ];
    let then_ops = vec![RequestOp::Put {
        key: key.clone(),
        value: value.clone(),
        options: None,
    }];
    let otherwise_ops = vec![RequestOp::Get {
        key: key.clone(),
        options: None,
    }];

    // Start and commit the transaction
    let response = kv_client
        .txn()
        .when(compares)?
        .then(then_ops)?
        .otherwise(otherwise_ops)?
        .commit()
        .await?;

    // Assert the transaction was successful
    assert!(!response.get_ref().succeeded);

    // Verify the value was set
    let get_response = kv_client.get(key.clone()).await?;
    let kvs = &get_response.get_ref().kvs;
    assert_eq!(kvs.len(), 1);
    assert_eq!(kvs[0].key, key.clone().to_vec());
    assert_eq!(kvs[0].value, value.clone().to_vec());

    Ok(())
}

#[test]
async fn test_txn_compare() -> Result<(), Error> {
    let key = ByteSequence::from("compare_test_key");

    let cmp_version = Compare::version_eq(key.clone(), 5);
    assert_eq!(cmp_version.target, CompareTarget::Version);
    assert_eq!(cmp_version.result, CompareResult::Equal);
    assert_eq!(cmp_version.key, key);
    assert_eq!(cmp_version.version, Some(5));
    assert!(cmp_version.create_revision.is_none());
    assert!(cmp_version.mod_revision.is_none());
    assert!(cmp_version.value.is_none());
    assert!(cmp_version.range_end.is_none());

    let cmp_value = Compare::value_eq(key.clone(), "test_value");
    assert_eq!(cmp_value.target, CompareTarget::Value);
    assert_eq!(cmp_value.result, CompareResult::Equal);
    assert_eq!(cmp_value.key, key);
    assert_eq!(cmp_value.value, Some(ByteSequence::from("test_value")));
    assert!(cmp_value.version.is_none());
    assert!(cmp_value.create_revision.is_none());
    assert!(cmp_value.mod_revision.is_none());
    assert!(cmp_value.range_end.is_none());

    let cmp_create = Compare::create_eq(key.clone(), 10);
    assert_eq!(cmp_create.target, CompareTarget::Create);
    assert_eq!(cmp_create.result, CompareResult::Equal);
    assert_eq!(cmp_create.key, key);
    assert_eq!(cmp_create.create_revision, Some(10));
    assert!(cmp_create.version.is_none());
    assert!(cmp_create.mod_revision.is_none());
    assert!(cmp_create.value.is_none());
    assert!(cmp_create.range_end.is_none());

    Ok(())
}

#[test]
async fn test_txn_compare_with_range_end() -> Result<(), Error> {
    let key = ByteSequence::from("compare_range_key");
    let range_end = ByteSequence::from("compare_range_end");

    let cmp = Compare::builder()
        .key(key.clone())
        .result(CompareResult::Equal)
        .value("test_value")
        .range_end(range_end.clone())
        .build();

    assert_eq!(cmp.target, CompareTarget::Value);
    assert_eq!(cmp.result, CompareResult::Equal);
    assert_eq!(cmp.key, key);
    assert_eq!(cmp.value, Some(ByteSequence::from("test_value")));
    assert_eq!(cmp.range_end, Some(range_end));
    assert!(cmp.version.is_none());
    assert!(cmp.create_revision.is_none());
    assert!(cmp.mod_revision.is_none());

    Ok(())
}

#[test]
async fn text_compact() -> Result<(), Error> {
    let client = get_client(None).await?;
    let mut kv_client = client.get_kv_client();

    let key = ByteSequence::from("compact_test_key");
    let value = ByteSequence::from("compact_test_value");

    // Clean up test key if it exists
    let _ = kv_client.delete(key.clone()).await;

    // Put a key-value pair
    let _ = kv_client.put(key.clone(), value.clone()).await?;

    // Compact the store at the current revision
    let get_response = kv_client.get(key.clone()).await?;
    let revision = get_response.get_ref().header.as_ref().unwrap().revision;

    let compact_response = kv_client.compact(revision).await?;

    assert!(compact_response.get_ref().header.is_some());

    // Clean up
    let _ = kv_client.delete(key.clone()).await;

    Ok(())
}

#[test]
async fn test_compact_with_options() -> Result<(), Error> {
    let client = get_client(None).await?;
    let mut kv_client = client.get_kv_client();

    let key = ByteSequence::from("compact_options_test_key");
    let value = ByteSequence::from("compact_options_test_value");

    // Clean up test key if it exists
    let _ = kv_client.delete(key.clone()).await;

    // Put a key-value pair
    let _ = kv_client.put(key.clone(), value.clone()).await?;

    // Compact the store at the current revision with physical option
    let get_response = kv_client.get(key.clone()).await?;
    let revision = get_response.get_ref().header.as_ref().unwrap().revision;

    let compact_options = CompactOptions::builder().physical(true).build();
    let compact_response = kv_client
        .compact_with_options(revision, compact_options)
        .await?;

    assert!(compact_response.get_ref().header.is_some());

    // Clean up
    let _ = kv_client.delete(key.clone()).await;

    Ok(())
}