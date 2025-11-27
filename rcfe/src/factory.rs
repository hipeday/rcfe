use crate::{ClientFactory, ClientOptions, Error, client::DefaultClient};
use tonic::async_trait;

/// A default implementation of the ClientFactory trait for creating DefaultClient instances.
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
