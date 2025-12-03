use rcfe::{ByteSequence, Client, Error, FilterType, WatchClient, WatchCreateOptions, WatchRequestType, Watcher};
mod common;

use common::*;

#[tokio::test]
async fn test_watch() -> Result<(), Error> {
    let client = get_client(None).await?;
    let mut watch_client = client.get_watch_client();

    let watch_request = WatchRequestType::Create(
        WatchCreateOptions::builder()
            .key(ByteSequence::from("test_key"))
            .prev_kv(true)
            .add_filter(FilterType::NoDelete)
            .add_filter(FilterType::NoPut)
            .build()?,
    );

    println!("{:?}", watch_request.to_request());

    let mut watcher = watch_client.watch(watch_request).await?;
    watcher.watch().await?;

    let mut response_stream = watcher.into_response().into_inner();

    while let Some(response) = response_stream.message().await? {
        println!("Watch Response: {:?}", response);
        if response.created {
            println!("Watcher created with ID: {}", response.watch_id);
        }
        if response.canceled {
            println!("Watcher canceled, reason: {}", response.cancel_reason);
        }
        if !response.events.is_empty() {
            for event in response.events {
                println!("Received Watch Event: {:?}", event);
            }
        }
    }

    Ok(())
}
