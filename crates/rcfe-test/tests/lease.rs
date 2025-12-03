use rcfe::{Client, Error, GrantOptions, KeepAliveHandler, LeaseClient, TimeToLiveOptions};
mod common;

use common::*;
use tokio::test;

#[test]
async fn test_lease_grant() -> Result<(), Error> {
    let client = get_client(None).await?;
    let mut lease_client = client.get_lease_client();

    // Grant a lease with a TTL of 5 seconds
    let ttl = std::time::Duration::new(5, 0);
    let response = lease_client.grant(ttl).await?;

    // Get the lease ID from the response
    let lease_id = response.get_ref().id;

    // Verify that the lease ID is valid (non-zero)
    assert_ne!(lease_id, 0, "Lease ID should be non-zero");

    // Clean up by revoking the lease
    lease_client.revoke(lease_id).await?;

    Ok(())
}

#[test]
async fn test_lease_grant_with_options() -> Result<(), Error> {
    let client = get_client(None).await?;
    let mut lease_client = client.get_lease_client();

    // Grant a lease with a TTL of 10 seconds using custom options
    let ttl = std::time::Duration::new(10, 0);
    let options = GrantOptions::builder()
        .id(12345) // Specify a custom lease ID
        .build();
    let response = lease_client.grant_with_options(ttl, options).await?;

    // Get the lease ID from the response
    let lease_id = response.get_ref().id;

    // Verify that the lease ID
    assert_eq!(lease_id, 12345, "Lease ID should match the specified ID");

    // Clean up by revoking the lease
    lease_client.revoke(lease_id).await?;

    Ok(())
}

#[test]
async fn test_lease_revoke() -> Result<(), Error> {
    let client = get_client(None).await?;
    let mut lease_client = client.get_lease_client();

    // Grant a lease to obtain a lease ID
    let ttl = std::time::Duration::new(5, 0);
    let grant_response = lease_client.grant(ttl).await?;
    let lease_id = grant_response.get_ref().id;

    // Revoke the lease using the obtained lease ID
    let revoke_response = lease_client.revoke(lease_id).await?;

    // Verify that the revoke response contains a header
    assert_eq!(revoke_response.get_ref().header.is_some(), true);

    Ok(())
}

#[test]
async fn test_keep_alive() -> Result<(), Error> {
    let client = get_client(None).await?;
    let mut lease_client = client.get_lease_client();

    let ttl = std::time::Duration::new(5, 0);

    lease_client.revoke(52663944).await.ok(); // Clean up any existing lease with the same ID

    let grant_response = lease_client
        .grant_with_options(ttl, GrantOptions::builder().id(52663944).build())
        .await?;
    let lease_id = grant_response.get_ref().id;

    assert_eq!(lease_id, 52663944, "Lease ID should match the specified ID");

    // 开启租约保活
    let mut keep_alive_handler = lease_client.keep_alive(lease_id).await?;
    let keep_alive_lease_id = keep_alive_handler.lease_id();
    assert_ne!(keep_alive_lease_id, 0, "Lease ID should be non-zero");

    // 使用线程发送保活请求
    for _ in 0..3 {
        keep_alive_handler.keep_alive().await?;
    }

    let mut streaming = keep_alive_handler.into_response().into_inner();

    // 监听保活提示日志
    while let Some(keep_alive) = streaming.message().await? {
        assert_eq!(keep_alive.id, lease_id, "Lease ID should match");
    }

    assert_eq!(keep_alive_lease_id, lease_id, "Lease ID should match");

    // Clean up by revoking the lease
    lease_client.revoke(lease_id).await?;

    Ok(())
}

#[test]
async fn test_lease_time_to_live() -> Result<(), Error> {
    let client = get_client(None).await?;
    let mut lease_client = client.get_lease_client();

    // Grant a lease to obtain a lease ID
    let ttl = std::time::Duration::new(10, 0);
    let grant_response = lease_client.grant(ttl).await?;
    let lease_id = grant_response.get_ref().id;

    // Retrieve the time-to-live information for the granted lease
    let ttl_response = lease_client.time_to_live(lease_id).await?;

    // Verify that the TTL response contains the expected lease ID and TTL value
    let ttl_info = ttl_response.get_ref();
    assert_eq!(ttl_info.id, lease_id, "Lease ID should match");
    assert!(ttl_info.ttl > 0, "TTL should be greater than zero");

    // Clean up by revoking the lease
    lease_client.revoke(lease_id).await?;

    Ok(())
}

#[test]
async fn test_lease_time_to_live_with_options() -> Result<(), Error> {
    let client = get_client(None).await?;
    let mut lease_client = client.get_lease_client();

    // Grant a lease to obtain a lease ID
    let ttl = std::time::Duration::new(15, 0);
    let grant_response = lease_client.grant(ttl).await?;
    let lease_id = grant_response.get_ref().id;

    // Retrieve the time-to-live information for the granted lease with custom options
    let options = TimeToLiveOptions::builder()
        .keys(true) // Example option to include keys
        .build();
    let ttl_response = lease_client.time_to_live_with_options(lease_id, options).await?;

    // Verify that the TTL response contains the expected lease ID and TTL value
    let ttl_info = ttl_response.get_ref();
    assert_eq!(ttl_info.id, lease_id, "Lease ID should match");
    assert!(ttl_info.ttl > 0, "TTL should be greater than zero");

    // Clean up by revoking the lease
    lease_client.revoke(lease_id).await?;

    Ok(())
}
