use crate::{
    Client, ClientOptions, Error, KVClient, KVOptions, LeaseClient, LeaseClientOptions,
    WatchClient, WatchClientOptions, kv::DefaultKVClient, lease::DefaultLeaseClient,
    watch::DefaultWatchClient,
};
use tonic::transport::Channel;

#[derive(Clone)]
pub struct DefaultClient {
    options: ClientOptions,
    kv_client: DefaultKVClient,
    lease_client: DefaultLeaseClient,
    watch_client: DefaultWatchClient,
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
                LeaseClientOptions::builder()
                    .channel(channel.clone())
                    .build()?,
            ),
            watch_client: DefaultWatchClient::new(
                WatchClientOptions::builder().channel(channel).build()?,
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

    fn get_watch_client(&self) -> impl WatchClient {
        self.watch_client.clone()
    }
}
