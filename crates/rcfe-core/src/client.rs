use crate::{kv::KVClient, lease::LeaseClient, options::client::ClientOptions};
use crate::watch::WatchClient;

/// Client trait defining the interface for a client.
/// Implementors must provide methods to retrieve client options and a key-value client.
pub trait Client {
    /// Get a reference to the client options.
    fn get_options(&self) -> &ClientOptions;

    /// Get the key-value client.
    fn get_kv_client(&self) -> impl KVClient;

    /// Get the lease client.
    fn get_lease_client(&self) -> impl LeaseClient;

    /// Get the watch client.
    fn get_watch_client(&self) -> impl WatchClient;
}
