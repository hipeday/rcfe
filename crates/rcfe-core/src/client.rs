use crate::{
    kv::KVClient,
    options::client::ClientOptions,
};

/// Client defines the interface for interacting with the etcd server.
/// It provides methods to access various service clients, such as the key-value client.
pub trait Client {
    fn get_options(&self) -> &ClientOptions;
    fn get_kv_client(&self) -> impl KVClient;
}