use std::time::Duration;
use crate::etcdserverpb::LeaseGrantRequest;

#[derive(Debug, Clone, Default)]
pub struct GrantOptions {
    /// The lease ID to grant.
    pub id: i64,
}

#[derive(Debug, Clone, Default)]
pub struct GrantOptionsBuilder {
    id: Option<i64>,
}

impl GrantOptionsBuilder {
    /// Sets the lease ID to grant.
    pub fn id(mut self, id: i64) -> Self {
        self.id = Some(id);
        self
    }

    /// Builds the `GrantOptions` instance.
    pub fn build(self) -> GrantOptions {
        GrantOptions {
            id: self.id.unwrap_or(0),
        }
    }
}

impl GrantOptions {
    pub fn to_request(&self, ttl: &Duration) -> LeaseGrantRequest {
        LeaseGrantRequest {
            id: self.id,
            ttl: ttl.as_secs() as i64,
        }
    }

    /// Creates a new `GrantOptionsBuilder`.
    pub fn builder() -> GrantOptionsBuilder {
        GrantOptionsBuilder::default()
    }
}