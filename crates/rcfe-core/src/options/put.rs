use crate::{ByteSequence, etcdserverpb};
use etcdserverpb::PutRequest;

#[derive(Debug, Clone, Default)]
pub struct PutOptions {
    /// lease is the lease ID to associate with the key in the key-value store. A lease
    /// value of 0 indicates no lease.
    pub lease: i64,

    /// If true, the previous key-value pair will be returned in the response.
    /// Default is false.
    pub prev_kv: bool,

    /// If true, the lease field in the PutRequest will be ignored.
    /// Default is false.
    pub ignore_lease: bool,

    /// If true, the value field in the PutRequest will be ignored.
    /// Default is false.
    pub ignore_value: bool,
}

impl PutOptions {
    pub fn to_request<K, V>(&self, key: K, value: V) -> PutRequest
    where
        K: Into<ByteSequence>,
        V: Into<ByteSequence>,
    {
        PutRequest {
            key: key.into().to_vec(),
            value: match self.ignore_value {
                true => ByteSequence::empty().to_vec(),
                false => value.into().to_vec(),
            },
            lease: self.lease,
            prev_kv: self.prev_kv,
            ignore_lease: self.ignore_lease,
            ignore_value: self.ignore_value,
        }
    }

    pub fn builder() -> PutOptionsBuilder {
        PutOptionsBuilder::default()
    }
}

#[derive(Debug, Clone, Default)]
pub struct PutOptionsBuilder {
    lease: Option<i64>,
    prev_kv: Option<bool>,
    ignore_lease: Option<bool>,
    ignore_value: Option<bool>,
}

impl PutOptionsBuilder {
    pub fn lease(mut self, lease: i64) -> Self {
        self.lease = Some(lease);
        self
    }

    pub fn prev_kv(mut self, prev_kv: bool) -> Self {
        self.prev_kv = Some(prev_kv);
        self
    }

    pub fn ignore_lease(mut self, ignore_lease: bool) -> Self {
        self.ignore_lease = Some(ignore_lease);
        self
    }

    pub fn ignore_value(mut self, ignore_value: bool) -> Self {
        self.ignore_value = Some(ignore_value);
        self
    }

    pub fn build(self) -> PutOptions {
        let mut options = PutOptions::default();
        if let Some(lease) = self.lease {
            options.lease = lease;
        }

        if let Some(prev_kv) = self.prev_kv {
            options.prev_kv = prev_kv;
        }

        if let Some(ignore_lease) = self.ignore_lease {
            options.ignore_lease = ignore_lease;
        }

        if let Some(ignore_value) = self.ignore_value {
            options.ignore_value = ignore_value;
        }
        options
    }
}
