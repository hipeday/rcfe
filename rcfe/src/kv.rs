use rcfe_core::{
    error::Error, etcdserverpb::RangeRequest, etcdserverpb::RangeResponse,
    etcdserverpb::kv_client::KvClient, kv::KVClient, options::kv::KVOptions,
};

use tonic::{Response, transport::Channel};
use rcfe_core::etcdserverpb::{DeleteRangeRequest, DeleteRangeResponse, PutRequest, PutResponse};

#[derive(Clone)]
pub struct DefaultKVClient {
    options: KVOptions,
    inner: KvClient<Channel>,
}

impl DefaultKVClient {
    pub fn new(opts: KVOptions) -> Self {
        DefaultKVClient {
            options: opts.clone(),
            inner: KvClient::new(opts.channel()),
        }
    }
}

#[tonic::async_trait]
impl KVClient for DefaultKVClient {
    async fn range_with_request(
        &mut self,
        request: RangeRequest,
    ) -> Result<Response<RangeResponse>, Error> {
        Ok(self.inner.range(request).await?)
    }

    async fn put_with_request(&mut self, request: PutRequest) -> Result<Response<PutResponse>, Error> {
        Ok(self.inner.put(request).await?)
    }

    async fn delete_with_request(&mut self, request: DeleteRangeRequest) -> Result<Response<DeleteRangeResponse>, Error> {
        Ok(self.inner.delete_range(request).await?)
    }

    fn get_options(&self) -> &KVOptions {
        &self.options
    }
}
