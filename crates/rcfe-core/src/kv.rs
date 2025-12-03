use crate::{
    ByteSequence, NamespaceBuilder, Namespaceable,
    error::Error,
    etcdserverpb::{CompactionResponse, DeleteRangeResponse, PutResponse, RangeResponse},
    options::{delete::DeleteOptions, get::GetOptions, kv::KVOptions, put::PutOptions},
    txn::Txn,
};
use tonic::Response;

/// KVClient defines the interface for interacting with the key-value store.
/// It provides methods to perform range queries with various options.
#[tonic::async_trait]
pub trait KVClient: Send + Sync {
    /// Compacts the key-value store up to the specified revision.
    async fn compact(&mut self, revision: i64) -> Result<Response<CompactionResponse>, Error> {
        self.compact_with_options(revision, crate::CompactOptions::default())
            .await
    }

    /// Compacts the key-value store up to the specified revision with the given options.
    async fn compact_with_options(
        &mut self,
        revision: i64,
        options: crate::CompactOptions,
    ) -> Result<Response<CompactionResponse>, Error>;

    /// Creates a new transaction associated with this KV client.
    fn txn(&mut self) -> impl Txn;

    /// Deletes a key-value pair from the store.
    async fn delete(&mut self, key: ByteSequence) -> Result<Response<DeleteRangeResponse>, Error> {
        self.delete_with_options(key, DeleteOptions::default())
            .await
    }

    /// Deletes a key-value pair from the store with the specified options.
    async fn delete_with_options(
        &mut self,
        key: ByteSequence,
        options: DeleteOptions,
    ) -> Result<Response<DeleteRangeResponse>, Error>;

    /// Puts a key-value pair into the store.
    async fn put<K, V>(&mut self, key: K, value: V) -> Result<Response<PutResponse>, Error>
    where
        K: Into<ByteSequence> + Send,
        V: Into<ByteSequence> + Send,
    {
        self.put_with_options(key.into(), value.into(), PutOptions::default())
            .await
    }

    /// Puts a key-value pair into the store with the specified options.
    async fn put_with_options<K, V>(
        &mut self,
        key: K,
        value: V,
        options: PutOptions,
    ) -> Result<Response<PutResponse>, Error>
    where
        K: Into<ByteSequence> + Send,
        V: Into<ByteSequence> + Send;

    /// Performs a range query with the specified key.
    async fn get<K>(&mut self, key: K) -> Result<Response<RangeResponse>, Error>
    where
        K: Into<ByteSequence> + Send,
    {
        self.get_with_options(
            key.into(),
            GetOptions::builder()
                .namespace(self.options().namespace())
                .build(),
        )
        .await
    }

    /// Performs a range query to retrieve all key-value pairs in the store.
    async fn get_all(
        &mut self,
        options: Option<GetOptions>,
    ) -> Result<Response<RangeResponse>, Error> {
        let options = if let Some(mut options) = options {
            options.end_key = Some(ByteSequence::from("\0"));
            options
        } else {
            GetOptions::builder()
                .end_key(ByteSequence::from("\0"))
                .build()
        };
        self.get_with_options(ByteSequence::from("\0"), options)
            .await
    }

    /// Performs a range query with the specified key and options.
    async fn get_with_options<K>(
        &mut self,
        key: K,
        options: GetOptions,
    ) -> Result<Response<RangeResponse>, Error>
    where
        K: Into<ByteSequence> + Send;

    /// Retrieves the KV options associated with this client.
    /// # Returns
    /// * `&KVOptions` - A reference to the KVOptions.
    fn options(&self) -> &KVOptions;
}
