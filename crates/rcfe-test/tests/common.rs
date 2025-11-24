use dotenvy::dotenv;
use rcfe::{ClientFactory, DefaultClient, Error};
use std::sync::Once;

static INIT: Once = Once::new();

/// Initialize environment variables from .env file
/// This function should be called at the beginning of each test module
/// to ensure that environment variables are loaded only once.
/// # Examples
/// ```rust
/// mod common;
/// use common::init;
/// init();
/// ```
pub fn init() {
    INIT.call_once(|| {
        dotenv().ok();
    })
}

/// Get the test endpoint from environment variable or use default
/// # Returns
/// * `String` - The test endpoint URL
/// # Examples
/// ```rust
/// mod common;
/// use common::get_endpoint;
/// let endpoint = get_endpoint();
/// ```
pub fn get_endpoint() -> String {
    std::env::var("TEST_ENDPOINT").unwrap_or_else(|_| "http://localhost:2379".to_string())
}

/// Create and return a RCFE client for testing
/// # Returns
/// * `Result<impl Client, Error>` - The RCFE client or an error
/// # Examples
/// ```rust
/// mod common;
/// use common::get_client;
/// let client = get_client().await.unwrap();
/// ```
pub async fn get_client() -> Result<DefaultClient, Error> {
    init();

    let client_options = rcfe::ClientOptions::builder()
        .endpoints(vec![get_endpoint()])
        .build();

    let client = rcfe::DefaultClientFactory::new()
        .create(client_options)
        .await?;
    Ok(client)
}
