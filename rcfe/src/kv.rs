use rcfe_core::{
    ByteSequence,
    error::Error,
    etcdserverpb::RangeResponse,
    etcdserverpb::kv_client::KvClient,
    kv::KVClient,
    options::{get::GetOptions, kv::KVOptions},
};
use tonic::{Response, transport::Channel};
use rcfe_core::etcdserverpb::{DeleteRangeResponse, PutResponse};
use rcfe_core::options::delete::DeleteOptions;
use rcfe_core::options::put::PutOptions;

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
    async fn delete_with_options(&mut self, key: ByteSequence, options: DeleteOptions) -> Result<Response<DeleteRangeResponse>, Error> {
        let request = options.to_request(&key);
        Ok(self.inner.delete_range(request).await?)
    }

    async fn put_with_options(&mut self, key: ByteSequence, value: ByteSequence, options: PutOptions) -> Result<Response<PutResponse>, Error> {
        let request = options.to_request(&key, &value);
        Ok(self.inner.put(request).await?)
    }

    async fn get_with_options(
        &mut self,
        key: ByteSequence,
        options: GetOptions,
    ) -> Result<Response<RangeResponse>, Error> {
        let request = options.to_request(&key);
        Ok(self.inner.range(request).await?)
    }

    fn get_options(&self) -> &KVOptions {
        &self.options
    }
}
