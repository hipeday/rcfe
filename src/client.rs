use tonic::Response;
use tonic::transport::Channel;
use crate::error::Error;

mod etcdserverpb {
    tonic::include_proto!("etcdserverpb");
}

mod mvccpb {
    tonic::include_proto!("mvccpb");
}

mod authpb {
    tonic::include_proto!("authpb");
}

mod v3electionpb {
    tonic::include_proto!("v3electionpb");
}

mod v3lockpb {
    tonic::include_proto!("v3lockpb");
}

pub struct Client {
    _options: ClientOptions,
    kv_client: KVClient,
}

pub struct KVClient {
    inner: etcdserverpb::kv_client::KvClient<Channel>,
}

pub struct ClientOptions {
    pub endpoints: Vec<String>,
}

impl Client {
    pub fn new(options: ClientOptions) -> Result<Client, Error> {
        let endpoints = options.endpoints
            .clone()
            .into_iter()
            .map(Channel::from_shared)
            .collect::<Result<Vec<_>, _>>()?
            .into_iter();

        let channel = Channel::balance_list(endpoints);

        Ok(Client { _options: options, kv_client: KVClient::new(channel.clone()) })
    }

    pub fn kv_client(&mut self) -> &mut KVClient {
        &mut self.kv_client
    }
}

impl KVClient {
    fn new(channel: Channel) -> KVClient {
        KVClient {
            inner: etcdserverpb::kv_client::KvClient::new(channel),
        }
    }

    pub async fn range(&mut self, key: &str) -> Result<Response<etcdserverpb::RangeResponse>, Error> {
        let request = etcdserverpb::RangeRequest {
            key: key.as_bytes().to_vec(),
            ..Default::default()
        };
        let response = self.inner.range(request).await?;
        Ok(response)
    }
}
