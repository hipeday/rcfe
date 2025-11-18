use rcfe::{Client, ClientFactory, ClientOptions, DefaultClientFactory, KVClient};

#[tokio::test]
async fn test_factory() -> Result<(), rcfe::Error> {
    // Build client options
    let client_options = ClientOptions::builder()
        .endpoints(vec!["http://191.168.0.250:2379"])
        .build();

    // Create a client using the DefaultClientFactory
    let client = DefaultClientFactory::new().create(client_options).await?;

    // Use the client to perform a KV operation
    let mut kv_client = client.get_kv_client();

    // Get the value for a specific key
    let response = kv_client.range("greeting").await?;

    println!("{:#?}", response);

    for item in response.into_inner().kvs {
        println!(
            "Key: {}, Value: {}",
            String::from_utf8_lossy(&item.key),
            String::from_utf8_lossy(&item.value)
        );
    }

    Ok(())
}
