use crate::{
    ByteSequence,
    error::Error,
    etcdserverpb::{DeleteRangeResponse, PutResponse, RangeResponse},
    options::{delete::DeleteOptions, get::GetOptions, kv::KVOptions, put::PutOptions},
};
use tonic::Response;

/// KVClient defines the interface for interacting with the key-value store.
/// It provides methods to perform range queries with various options.
#[tonic::async_trait]
pub trait KVClient {
    async fn delete(&mut self, key: ByteSequence) -> Result<Response<DeleteRangeResponse>, Error> {
        self.delete_with_options(key, DeleteOptions::default())
            .await
    }

    /// Deletes a key-value pair from the store with the specified options.
    /// # Arguments
    /// * `key` - The key to delete.
    /// * `options` - The options to customize the delete operation.
    /// # Returns
    /// * `Result<Response<etcdserverpb::DeleteRangeResponse>, error::Error>` - The response containing the delete result or an error.
    /// # Examples
    /// ```rust
    /// use rcfe_core::kv::KVClient;
    /// use rcfe_core::ByteSequence;
    /// use rcfe_core::error::Error;
    /// use tonic::Response;
    /// use rcfe_core::etcdserverpb::DeleteRangeResponse;
    /// use rcfe_core::options::kv::DeleteOptions;
    /// async fn example<KV: KVClient>(kv_client: &mut KV, key: ByteSequence, options: DeleteOptions) -> Result<Response<DeleteRangeResponse>, Error> {
    ///     kv_client.delete_with_options(key, options).await
    /// }
    /// ```
    async fn delete_with_options(
        &mut self,
        key: ByteSequence,
        options: DeleteOptions,
    ) -> Result<Response<DeleteRangeResponse>, Error>;

    /// Puts a key-value pair into the store.
    /// # Arguments
    /// * `key` - The key to put.
    /// * `value` - The value to associate with the key.
    /// # Returns
    /// * `Result<Response<etcdserverpb::PutResponse>, error::Error>` - The response containing the put result or an error.
    /// # Examples
    /// ```rust
    /// use rcfe_core::kv::KVClient;
    /// use rcfe_core::ByteSequence;
    /// use rcfe_core::error::Error;
    /// use tonic::Response;
    /// use rcfe_core::etcdserverpb::PutResponse;
    ///
    /// async fn example<KV: KVClient>(kv_client: &mut KV, key: ByteSequence, value: ByteSequence) -> Result<Response<PutResponse>, Error> {
    ///     kv_client.put(key, value).await
    /// }
    /// ```
    async fn put(
        &mut self,
        key: ByteSequence,
        value: ByteSequence,
    ) -> Result<Response<PutResponse>, Error> {
        self.put_with_options(key, value, PutOptions::default())
            .await
    }

    /// Puts a key-value pair into the store with the specified options.
    /// # Arguments
    /// * `key` - The key to put.
    /// * `value` - The value to associate with the key.
    /// * `options` - The options to customize the put operation.
    /// # Returns
    /// * `Result<Response<etcdserverpb::PutResponse>, error::Error>` - The response containing the put result or an error.
    /// # Examples
    /// ```rust
    /// use rcfe_core::kv::KVClient;
    /// use rcfe_core::ByteSequence;
    /// use rcfe_core::error::Error;
    /// use tonic::Response;
    /// use rcfe_core::etcdserverpb::PutResponse;
    /// use rcfe_core::options::kv::PutOptions;
    ///
    /// async fn example<KV: KVClient>(kv_client: &mut KV, key: ByteSequence, value: ByteSequence, options: PutOptions) -> Result<Response<PutResponse>, Error> {
    ///     kv_client.put_with_options(key, value, options).await
    /// }
    /// ```
    async fn put_with_options(
        &mut self,
        key: ByteSequence,
        value: ByteSequence,
        options: PutOptions,
    ) -> Result<Response<PutResponse>, Error>;

    /// Performs a range query with the specified key.
    /// # Arguments
    /// * `key` - The key to query.
    /// # Returns
    /// * `Result<Response<etcdserverpb::RangeResponse>, error::Error>` - The response containing the range results or an error.
    /// # Examples
    /// ```rust
    /// use rcfe_core::kv::KVClient;
    /// use rcfe_core::ByteSequence;
    /// use rcfe_core::error::Error;
    /// use tonic::Response;
    /// use rcfe_core::etcdserverpb::RangeResponse;
    ///
    /// async fn example<KV: KVClient>(kv_client: &mut KV, key: ByteSequence) -> Result<Response<RangeResponse>, Error> {
    ///     kv_client.get(key).await
    /// }
    /// ```
    async fn get(&mut self, key: ByteSequence) -> Result<Response<RangeResponse>, Error> {
        self.get_with_options(key, GetOptions::default()).await
    }

    /// Performs a range query to retrieve all key-value pairs in the store.
    /// # Returns
    /// * `Result<Response<etcdserverpb::RangeResponse>, error::Error>` - The response containing all key-value pairs or an error.
    /// # Examples
    /// ```rust
    /// use rcfe_core::kv::KVClient;
    /// use rcfe_core::error::Error;
    /// use tonic::Response;
    /// use rcfe_core::etcdserverpb::RangeResponse;
    ///
    /// async fn example<KV: KVClient>(kv_client: &mut KV) -> Result<Response<RangeResponse>, Error> {
    ///     kv_client.get_all().await
    /// }
    /// ```
    async fn get_all(&mut self) -> Result<Response<RangeResponse>, Error> {
        let options = GetOptions::builder()
            .end_key(ByteSequence::from("\0"))
            .build();
        self.get_with_options(ByteSequence::from("\0"), options)
            .await
    }

    /// Performs a range query with the specified key and options.
    /// # Arguments
    /// * `key` - The key to query.
    /// * `options` - The options to customize the range query.
    /// # Returns
    /// * `Result<Response<etcdserverpb::RangeResponse>, error::Error>` - The response containing the range results or an error.
    /// # Examples
    /// ```rust
    /// use rcfe_core::kv::KVClient;
    /// use rcfe_core::ByteSequence;
    /// use rcfe_core::error::Error;
    /// use tonic::Response;
    /// use rcfe_core::etcdserverpb::RangeResponse;
    /// use rcfe_core::options::kv::GetOptions;
    ///
    /// async fn example<KV: KVClient>(kv_client: &mut KV, key: ByteSequence, options: GetOptions) -> Result<Response<RangeResponse>, Error> {
    ///     kv_client.get_with_options(key, options).await
    /// }
    /// ```
    async fn get_with_options(
        &mut self,
        key: ByteSequence,
        options: GetOptions,
    ) -> Result<Response<RangeResponse>, Error>;

    /// Retrieves the KV options associated with this client.
    /// # Returns
    /// * `&KVOptions` - A reference to the KVOptions.
    fn get_options(&self) -> &KVOptions;
}
