mod common;

use rcfe::{ByteSequence, Client, Compare, Error, KVClient, RequestOp, Txn};
use tokio::test;

use common::get_client;

#[test]
async fn test_txn() -> Result<(), Error> {
    let client = get_client().await?;
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
    }];
    let otherwise_ops = vec![RequestOp::Get {
        key: key.clone(),
        range_end: None,
    }];

    // Start and commit the transaction
    let response = kv_client.txn()
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
    let compares = vec![Compare::version_eq(key.clone(), 0)];
    let then_ops = vec![RequestOp::Put {
        key: key.clone(),
        value: value.clone(),
    }];
    let otherwise_ops = vec![RequestOp::Get {
        key: key.clone(),
        range_end: None,
    }];

    // Start and commit the transaction
    let response = kv_client.txn()
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
