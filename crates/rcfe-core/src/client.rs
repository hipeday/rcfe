use crate::{kv::KVClient, lease::LeaseClient, options::client::ClientOptions, watch::WatchClient};

/// Client trait defining the interface for a client.
/// Implementors must provide methods to retrieve client options and a key-value client.
#[tonic::async_trait]
pub trait Client {
    /// Ping the server to check connectivity.
    async fn ping(&self) -> Result<(), crate::Error> {
        let mut kv_client = self.get_kv_client();
        kv_client.get("__ping__").await.map(|_| ())?;
        Ok(())
    }

    /// Get a reference to the client options.
    fn get_options(&self) -> &ClientOptions;

    /// Get the key-value client.
    fn get_kv_client(&self) -> impl KVClient;

    /// Get the lease client.
    fn get_lease_client(&self) -> impl LeaseClient;

    /// Get the watch client.
    fn get_watch_client(&self) -> impl WatchClient;
}
