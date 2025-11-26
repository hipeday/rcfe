use crate::{kv::DefaultKVClient, lease::DefaultLeaseClient};
use rcfe_core::{
    client::Client,
    error::Error,
    kv::KVClient,
    lease::LeaseClient,
    options::{client::ClientOptions, kv::KVOptions, lease::LeaseOptions},
};
use tonic::transport::Channel;

#[derive(Clone)]
pub struct DefaultClient {
    options: ClientOptions,
    kv_client: DefaultKVClient,
    lease_client: DefaultLeaseClient,
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
            lease_client: DefaultLeaseClient::new(
                LeaseOptions::builder().channel(channel.clone()).build()?,
            ),
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

    fn get_lease_client(&self) -> impl LeaseClient {
        self.lease_client.clone()
    }
}
