use crate::{
    error::Error,
    etcdserverpb::{
        LeaseGrantResponse, LeaseKeepAliveResponse, LeaseRevokeResponse, LeaseTimeToLiveResponse,
    },
    options::{lease::TimeToLiveOptions, lease::grant::GrantOptions},
};
use std::time::Duration;
use tonic::{Response, Streaming, async_trait};

/// Handler for managing lease keep-alive responses.
#[async_trait]
pub trait KeepAliveHandler {
    /// Retrieves the lease ID associated with the keep-alive handler.
    fn lease_id(&self) -> i64;

    /// Converts the keep-alive handler into a response containing the streaming lease keep-alive responses.
    fn into_response(self) -> Response<Streaming<LeaseKeepAliveResponse>>;

    /// Sends a keep-alive request to renew the lease.
    async fn keep_alive(&mut self) -> Result<(), Error>;
}

#[async_trait]
pub trait LeaseClient {
    /// Grants a lease with the specified time-to-live (TTL).
    async fn grant(&mut self, ttl: Duration) -> Result<Response<LeaseGrantResponse>, Error> {
        self.grant_with_options(ttl, GrantOptions::default()).await
    }

    /// Grants a lease with the specified time-to-live (TTL) and options.
    async fn grant_with_options(
        &mut self,
        ttl: Duration,
        options: GrantOptions,
    ) -> Result<Response<LeaseGrantResponse>, Error>;

    /// Revokes a lease with the specified lease ID.
    async fn revoke(&self, lease_id: i64) -> Result<Response<LeaseRevokeResponse>, Error>;

    /// Keeps the lease alive for the specified lease ID.
    async fn keep_alive(&mut self, lease_id: i64) -> Result<impl KeepAliveHandler, Error>;

    /// Retrieves the time-to-live (TTL) information for the specified lease ID.
    async fn time_to_live(&mut self, lease_id: i64)
    -> Result<Response<LeaseTimeToLiveResponse>, Error> {
        self.time_to_live_with_options(
            lease_id,
            TimeToLiveOptions::default(),
        )
        .await
    }

    /// Retrieves the time-to-live (TTL) information for the specified lease ID with options.
    async fn time_to_live_with_options(
        &mut self,
        lease_id: i64,
        options: TimeToLiveOptions,
    ) -> Result<Response<LeaseTimeToLiveResponse>, Error>;
}
