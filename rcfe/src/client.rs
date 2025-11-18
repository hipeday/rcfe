use rcfe_core::{
    error::Error,
    options::client::ClientOptions,
    options::kv::KVOptions,
    client::Client,
    kv::KVClient
};
use tonic::transport::Channel;
use crate::kv::DefaultKVClient;

#[derive(Clone)]
pub struct DefaultClient {
    options: ClientOptions,
    kv_client: DefaultKVClient,
}

impl DefaultClient {
    pub fn new(opts: ClientOptions) -> Result<Self, Error> {
        let endpoints = opts
            .endpoints()
            .clone()
            .into_iter()
            .map(Channel::from_shared)
            .collect::<Result<Vec<_>, _>>()?
            .into_iter();

        let channel = Channel::balance_list(endpoints);

        Ok(DefaultClient {
            options: opts,
            kv_client: DefaultKVClient::new(KVOptions::builder().channel(channel.clone()).build()?),
        })
    }
}

impl Client for DefaultClient {
    fn get_options(&self) -> &ClientOptions {
        &self.options
    }

    fn get_kv_client(&self) -> impl KVClient {
        self.kv_client.clone()
    }
}
