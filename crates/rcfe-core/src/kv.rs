use tonic::Response;
use crate::{
    error::Error,
    etcdserverpb::RangeResponse
};

/// KVClient defines the interface for interacting with the key-value store.
/// It provides methods to perform range queries with various options.
#[tonic::async_trait]
pub trait KVClient {
    async fn range(&mut self, key: &str) -> Result<Response<RangeResponse>, Error>;

    async fn range_with_options(
        &mut self,
        key: Option<&str>,
        range_end: Option<&str>,
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
    ) -> Result<Response<RangeResponse>, Error>;

    async fn range_with_request(
        &mut self,
        request: crate::etcdserverpb::RangeRequest,
    ) -> Result<Response<RangeResponse>, Error>;

    fn get_options(&self) -> &crate::options::kv::KVOptions;
}