use crate::DefaultTxn;
use rcfe_core::{
    etcdserverpb::{
        CompactionResponse,
        DeleteRangeResponse,
        PutResponse,
        RangeResponse,
        kv_client::KvClient
    },
    options::{
        compact::CompactOptions,
        delete::DeleteOptions,
        get::GetOptions,
        kv::KVOptions,
        put::PutOptions
    },
    ByteSequence,
    error::Error,
    kv::KVClient,
    txn::Txn
};
use tonic::{Response, transport::Channel};

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
    async fn compact_with_options(
        &mut self,
        revision: i64,
        options: CompactOptions,
    ) -> Result<Response<CompactionResponse>, Error> {
        let request = options.to_request(revision);
        Ok(self.inner.compact(request).await?)
    }

    fn txn(&mut self) -> impl Txn {
        DefaultTxn::new(self.inner.clone())
    }

    async fn delete_with_options(
        &mut self,
        key: ByteSequence,
        options: DeleteOptions,
    ) -> Result<Response<DeleteRangeResponse>, Error> {
        let request = options.to_request(&key);
        Ok(self.inner.delete_range(request).await?)
    }

    async fn put_with_options(
        &mut self,
        key: ByteSequence,
        value: ByteSequence,
        options: PutOptions,
    ) -> Result<Response<PutResponse>, Error> {
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
