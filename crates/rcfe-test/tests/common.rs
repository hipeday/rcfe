use dotenvy::dotenv;
use rcfe::{ClientFactory, DefaultClient, Error, NamespaceBuilder};
use std::sync::Once;

static INIT: Once = Once::new();

/// Initialize environment variables from .env file
/// This function should be called at the beginning of each test module
/// to ensure that environment variables are loaded only once.
pub fn init() {
    INIT.call_once(|| {
        dotenv().ok();
    })
}

/// Get the test endpoint from environment variable or use default
pub fn get_endpoint() -> String {
    std::env::var("TEST_ENDPOINT").unwrap_or_else(|_| "http://localhost:2379".to_string())
}

/// Create and return a RCFE client for testing
pub async fn get_client(namespace: Option<&str>) -> Result<DefaultClient, Error> {
    init();

    let client_options = rcfe::ClientOptions::builder()
        .endpoints(vec![get_endpoint()])
        .namespace(namespace)
        .build();

    let client = rcfe::DefaultClientFactory::new()
        .create(client_options)
        .await?;
    Ok(client)
}
