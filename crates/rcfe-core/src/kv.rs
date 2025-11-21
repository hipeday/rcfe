use tonic::Response;
use crate::{error::Error, etcdserverpb::RangeResponse, ByteSequence};
use crate::etcdserverpb::{DeleteRangeResponse, RangeRequest};

/// KVClient defines the interface for interacting with the key-value store.
/// It provides methods to perform range queries with various options.
#[tonic::async_trait]
pub trait KVClient {

    /// Performs a range query for the specified key.
    /// # Arguments
    /// * `key` - The key to query.
    /// # Returns
    /// * `Result<Response<RangeResponse>, Error>` - The response containing the range results or an error if the operation fails.
    /// # Errors
    /// * Returns an `Error` if the range operation fails.
    async fn range(&mut self, key: ByteSequence) -> Result<Response<RangeResponse>, Error> {
        self.range_with_options(
            Some(key),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        ).await
    }

    /// Performs a range query for all keys with the specified prefix.
    /// # Arguments
    /// * `prefix` - The prefix to query.
    /// # Returns
    /// * `Result<Response<RangeResponse>, Error>` - The response containing the range results or an error if the operation fails.
    /// # Errors
    /// * Returns an `Error` if the range operation fails.
    async fn range_with_prefix(&mut self, prefix: ByteSequence) -> Result<Response<RangeResponse>, Error> {
        let range_end = prefix.next();
        self.range_with_options(
            Some(prefix),
            Some(range_end),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        ).await
    }

    /// Performs a range query for the specified key as a string.
    /// # Arguments
    /// * `key` - The key to query as a string.
    /// # Returns
    /// * `Result<Response<RangeResponse>, Error>` - The response containing the range results or an error if the operation fails.
    /// # Errors
    /// * Returns an `Error` if the range operation fails.
    async fn range_with_str(&mut self, key: &str) -> Result<Response<RangeResponse>, Error> {
        self.range(ByteSequence::from(key)).await
    }

    /// Performs a range query for the specified key and range end.
    /// # Arguments
    /// * `key` - The key to query as a string.
    /// * `range_end` - The end of the range to query.
    /// # Returns
    /// * `Result<Response<RangeResponse>, Error>` - The response containing the range results or an error if the operation fails.
    /// # Errors
    /// * Returns an `Error` if the range operation fails.
    async fn range_with_end(&mut self, key: ByteSequence, range_end: ByteSequence) -> Result<Response<RangeResponse>, Error> {
        self.range_with_options(
            Some(key),
            Some(range_end),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        ).await
    }

    /// Performs a range query for all keys.
    /// # Returns
    /// * `Result<Response<RangeResponse>, Error>` - The response containing the range results or an error if the operation fails.
    /// # Errors
    /// * Returns an `Error` if the range operation fails.
    async fn range_all(&mut self) -> Result<Response<RangeResponse>, Error> {
        self.range_with_options(
            Some(ByteSequence::empty()),
            Some(ByteSequence::empty()),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        ).await
    }

    /// Performs a range query with various options.
    /// # Arguments
    /// * `key` - The key to query (optional).
    /// * `range_end` - The end of the range to query (optional).
    /// * `revision` - The revision to query at (optional).
    /// * `sort_order` - The order to sort the results (optional).
    /// * `sort_target` - The target to sort the results by (optional).
    /// * `serializable` - Whether to perform a serializable read (optional).
    /// * `keys_only` - Whether to return only keys (optional).
    /// * `count_only` - Whether to return only the count of keys (optional).
    /// * `min_mod_revision` - Minimum modification revision to filter results (optional).
    /// * `max_mod_revision` - Maximum modification revision to filter results (optional).
    /// * `min_create_revision` - Minimum creation revision to filter results (optional).
    /// * `max_create_revision` - Maximum creation revision to filter results (optional).
    /// # Returns
    /// * `Result<Response<RangeResponse>, Error>` - The response containing the range results or an error if the operation fails.
    /// # Errors
    /// * Returns an `Error` if the range operation fails.
    async fn range_with_options(
        &mut self,
        key: Option<ByteSequence>,
        range_end: Option<ByteSequence>,
        revision: Option<i64>,
        sort_order: Option<i32>,
        sort_target: Option<i32>,
        serializable: Option<bool>,
        keys_only: Option<bool>,
        count_only: Option<bool>,
        min_mod_revision: Option<i64>,
        max_mod_revision: Option<i64>,
        min_create_revision: Option<i64>,
        max_create_revision: Option<i64>,
    ) -> Result<Response<RangeResponse>, Error> {
        let request = self.build_range_request(
            key,
            range_end,
            None,
            revision,
            sort_order,
            sort_target,
            serializable,
            keys_only,
            count_only,
            min_mod_revision,
            max_mod_revision,
            min_create_revision,
            max_create_revision,
        );
        self.range_with_request(request).await
    }

    /// Performs a range query using a pre-constructed RangeRequest.
    /// # Arguments
    /// * `request` - The RangeRequest to use for the query.
    /// # Returns
    /// * `Result<Response<RangeResponse>, Error>` - The response containing the range results or an error if the operation fails.
    /// # Errors
    /// * Returns an `Error` if the range operation fails.
    async fn range_with_request(
        &mut self,
        request: RangeRequest,
    ) -> Result<Response<RangeResponse>, Error>;

    /// Performs a put operation for the specified key and value.
    /// # Arguments
    /// * `key` - The key to put.
    /// * `value` - The value to put.
    /// # Returns
    /// * `Result<Response<PutResponse>, Error>` - The response containing the put results or an error if the operation fails.
    /// # Errors
    /// * Returns an `Error` if the put operation fails.
    async fn put(
        &mut self,
        key: ByteSequence,
        value: ByteSequence,
    ) -> Result<Response<crate::etcdserverpb::PutResponse>, Error> {
        let request = self.build_put_request(key, value, None, None, None, None);
        self.put_with_request(request).await
    }

    /// Performs a put operation using a pre-constructed PutRequest.
    /// # Arguments
    /// * `request` - The PutRequest to use for the operation.
    /// # Returns
    /// * `Result<Response<PutResponse>, Error>` - The response containing the put results or an error if the operation fails.
    /// # Errors
    /// * Returns an `Error` if the put operation fails.
    async fn put_with_request(
        &mut self,
        request: crate::etcdserverpb::PutRequest,
    ) -> Result<Response<crate::etcdserverpb::PutResponse>, Error>;

    /// Performs a delete operation for the specified key.
    /// # Arguments
    /// * `key` - The key to delete.
    /// # Returns
    /// * `Result<Response<DeleteRangeResponse>, Error>` - The response containing the delete results or an error if the operation fails.
    /// # Errors
    /// * Returns an `Error` if the delete operation fails.
    async fn delete(&mut self, key: ByteSequence) -> Result<Response<DeleteRangeResponse>, Error> {
        let request = self.build_delete_request(key, None, None);
        self.delete_with_request(request).await
    }

    /// Performs a delete operation for the specified key.
    /// # Arguments
    /// * `key` - The key to delete.
    /// # Returns
    /// * `Result<Response<DeleteRangeResponse>, Error>` - The response containing the delete results or an error if the operation fails.
    /// # Errors
    /// * Returns an `Error` if the delete operation fails.
    async fn delete_with_request(
        &mut self,
        request: crate::etcdserverpb::DeleteRangeRequest,
    ) -> Result<Response<DeleteRangeResponse>, Error>;

    /// Builds a RangeRequest with the specified parameters.
    /// # Arguments
    /// * `key` - The key to query (optional).
    /// * `range_end` - The end of the range to query (optional).
    /// * `limit` - The maximum number of results to return (optional).
    /// * `revision` - The revision to query at (optional).
    /// * `sort_order` - The order to sort the results (optional).
    /// * `sort_target` - The target to sort the results by (optional).
    /// * `serializable` - Whether to perform a serializable read (optional).
    /// * `keys_only` - Whether to return only keys (optional).
    /// * `count_only` - Whether to return only the count of keys (optional).
    /// * `min_mod_revision` - Minimum modification revision to filter results (optional).
    /// * `max_mod_revision` - Maximum modification revision to filter results (optional).
    /// * `min_create_revision` - Minimum creation revision to filter results (optional).
    /// * `max_create_revision` - Maximum creation revision to filter results (optional).
    /// # Returns
    /// * `RangeRequest` - The constructed RangeRequest.
    fn build_range_request(
        &self,
        key: Option<ByteSequence>,
        range_end: Option<ByteSequence>,
        limit: Option<i64>,
        revision: Option<i64>,
        sort_order: Option<i32>,
        sort_target: Option<i32>,
        serializable: Option<bool>,
        keys_only: Option<bool>,
        count_only: Option<bool>,
        min_mod_revision: Option<i64>,
        max_mod_revision: Option<i64>,
        min_create_revision: Option<i64>,
        max_create_revision: Option<i64>,
    ) -> RangeRequest {
        let mut request = RangeRequest {
            ..Default::default()
        };

        if let Some(k) = key {
            request.key = k.as_bytes().to_vec();
        }

        if let Some(re) = range_end {
            request.range_end = re.as_bytes().to_vec();
        }

        if let Some(l) = limit {
            request.limit = l;
        }

        if let Some(r) = revision {
            request.revision = r;
        }

        if let Some(so) = sort_order {
            request.sort_order = so;
        }

        if let Some(st) = sort_target {
            request.sort_target = st;
        }

        if let Some(s) = serializable {
            request.serializable = s;
        }

        if let Some(ko) = keys_only {
            request.keys_only = ko;
        }

        if let Some(co) = count_only {
            request.count_only = co;
        }

        if let Some(mmr) = min_mod_revision {
            request.min_mod_revision = mmr;
        }

        if let Some(xmr) = max_mod_revision {
            request.max_mod_revision = xmr;
        }

        if let Some(mcr) = min_create_revision {
            request.min_create_revision = mcr;
        }

        if let Some(xcr) = max_create_revision {
            request.max_create_revision = xcr;
        }

        request
    }

    /// Builds a PutRequest with the specified parameters.
    /// # Arguments
    /// * `key` - The key to put.
    /// * `value` - The value to put.
    /// * `lease` - The lease ID to associate with the key (optional).
    /// * `prev_kv` - Whether to return the previous key-value pair (optional).
    /// * `ignore_value` - Whether to ignore the value in the put operation (optional).
    /// * `ignore_lease` - Whether to ignore the lease in the put operation (optional).
    /// # Returns
    /// * `PutRequest` - The constructed PutRequest.
    /// Returns a PutRequest constructed with the provided parameters.
    fn build_put_request(
        &self,
        key: ByteSequence,
        value: ByteSequence,
        lease: Option<i64>,
        prev_kv: Option<bool>,
        ignore_value: Option<bool>,
        ignore_lease: Option<bool>,
    ) -> crate::etcdserverpb::PutRequest {
        let mut request = crate::etcdserverpb::PutRequest {
            key: key.as_bytes().to_vec(),
            value: value.as_bytes().to_vec(),
            ..Default::default()
        };

        if let Some(l) = lease {
            request.lease = l;
        }

        if let Some(pk) = prev_kv {
            request.prev_kv = pk;
        }

        if let Some(iv) = ignore_value {
            request.ignore_value = iv;
        }

        if let Some(il) = ignore_lease {
            request.ignore_lease = il;
        }

        request
    }

    /// Builds a DeleteRangeRequest with the specified parameters.
    /// # Arguments
    /// * `key` - The key to delete.
    /// * `range_end` - The end of the range to delete (optional).
    /// * `prev_kv` - Whether to return the previous key-value pairs (optional).
    /// # Returns
    /// * `DeleteRangeRequest` - The constructed DeleteRangeRequest.
    /// Returns a DeleteRangeRequest constructed with the provided parameters.
    fn build_delete_request(
        &self,
        key: ByteSequence,
        range_end: Option<ByteSequence>,
        prev_kv: Option<bool>,
    ) -> crate::etcdserverpb::DeleteRangeRequest {
        let mut request = crate::etcdserverpb::DeleteRangeRequest {
            key: key.as_bytes().to_vec(),
            ..Default::default()
        };

        if let Some(re) = range_end {
            request.range_end = re.as_bytes().to_vec();
        }

        if let Some(pk) = prev_kv {
            request.prev_kv = pk;
        }

        request
    }

    /// Retrieves the KV options associated with this client.
    /// # Returns
    /// * `&KVOptions` - A reference to the KVOptions.
    fn get_options(&self) -> &crate::options::kv::KVOptions;
}