use dotenvy::dotenv;
use rcfe::{
    ByteSequence, Client, ClientFactory, ClientOptions, DefaultClientFactory, Error, KVClient,
    WatchClient, WatchCreateOptions, WatchRequestType, Watcher,
};
use std::sync::Once;

static INIT: Once = Once::new();

pub fn init() {
    INIT.call_once(|| {
        dotenv().ok();
    })
}

pub fn get_endpoint() -> String {
    std::env::var("TEST_ENDPOINT").unwrap_or_else(|_| "http://localhost:2379".to_string())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    init();

    let client = DefaultClientFactory::new()
        .create(
            ClientOptions::builder()
                .endpoints(vec![get_endpoint()])
                .build(),
        )
        .await?;

    let mut watch_client = client.get_watch_client();

    let watch_request = WatchRequestType::Create(
        WatchCreateOptions::builder()
            .key(ByteSequence::from("test_key"))
            .prev_kv(true)
            .build()?,
    );

    let mut watcher = watch_client.watch(watch_request).await?;
    client
        .get_kv_client()
        .put(ByteSequence::from("test_key"), "test_value")
        .await?;

    watcher.progress().await?;

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
