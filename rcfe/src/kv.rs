use crate::{
    ByteSequence, CompactOptions, CompactionResponse, DefaultTxn, DeleteOptions,
    DeleteRangeResponse, Error, GetOptions, GrpcKVClient, KVClient, KVOptions, PutOptions,
    PutResponse, RangeResponse, Txn,
};
use tonic::{Response, transport::Channel};

#[derive(Clone)]
pub struct DefaultKVClient {
    options: KVOptions,
    inner: GrpcKVClient<Channel>,
}

impl DefaultKVClient {
    pub fn new(opts: KVOptions) -> Self {
        DefaultKVClient {
            options: opts.clone(),
            inner: GrpcKVClient::new(opts.channel()),
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

    async fn put_with_options<K, V>(
        &mut self,
        key: K,
        value: V,
        options: PutOptions,
    ) -> Result<Response<PutResponse>, Error>
    where
        K: Into<ByteSequence> + Send,
        V: Into<ByteSequence> + Send,
    {
        let request = options.to_request(key, value);
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
