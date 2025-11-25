use tonic::async_trait;
use rcfe_core::{
    error::Error,
    factory::ClientFactory,
    options::client::ClientOptions
};
use crate::client::DefaultClient;

/// A default implementation of the ClientFactory trait for creating DefaultClient instances.
/// # Examples
/// ```rust
/// use crate::rcfe::{DefaultClientFactory, ClientFactory, ClientOptions, Error, Client};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Error> {
///     let factory = DefaultClientFactory;
///     let options = ClientOptions::builder()
///         .endpoints(vec!["http://localhost:2379"])
///         .build();
///
///     let client = factory.create(options).await?;
///     println!("Client created with options: {:?}", client.get_options());
///     Ok(())
/// }
/// ```
pub struct DefaultClientFactory;

impl DefaultClientFactory {
    pub fn new() -> Self {
        DefaultClientFactory
    }
}

#[async_trait]
impl ClientFactory<DefaultClient> for DefaultClientFactory {
    async fn create(&self, opts: ClientOptions) -> Result<DefaultClient, Error> {
        Ok(DefaultClient::new(opts)?)
    }
}