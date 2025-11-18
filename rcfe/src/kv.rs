use rcfe_core::{
    error::Error, etcdserverpb::RangeRequest, etcdserverpb::RangeResponse,
    etcdserverpb::kv_client::KvClient, kv::KVClient, options::kv::KVOptions,
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

    fn build_range_request(
        &self,
        key: Option<&str>,
        range_end: Option<&str>,
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
}

#[tonic::async_trait]
impl KVClient for DefaultKVClient {
    async fn range(&mut self, key: &str) -> Result<Response<RangeResponse>, Error> {
        Ok(self
            .range_with_options(
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
            )
            .await?)
    }

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

        Ok(self.range_with_request(request).await?)
    }

    async fn range_with_request(
        &mut self,
        request: RangeRequest,
    ) -> Result<Response<RangeResponse>, Error> {
        Ok(self.inner.range(request).await?)
    }

    fn get_options(&self) -> &KVOptions {
        &self.options
    }
}
