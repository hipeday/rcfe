use rcfe_core::{
    error::Error,
    etcdserverpb::{
        LeaseGrantResponse, LeaseKeepAliveRequest, LeaseKeepAliveResponse, LeaseRevokeRequest,
        LeaseRevokeResponse, lease_client,
    },
    lease::{KeepAliveHandler, LeaseClient},
    options::lease::{LeaseOptions, grant::GrantOptions},
};
use std::time::Duration;
use tonic::{
    Request, Response, Streaming, async_trait, codegen::tokio_stream::wrappers::ReceiverStream,
    transport::Channel,
};
use rcfe_core::etcdserverpb::LeaseTimeToLiveResponse;
use rcfe_core::options::lease::TimeToLiveOptions;

pub struct DefaultKeepAliveHandler {
    lease_id: i64,
    sender: tokio::sync::mpsc::Sender<LeaseKeepAliveRequest>,
    response: Response<Streaming<LeaseKeepAliveResponse>>,
}

impl DefaultKeepAliveHandler {
    pub(crate) fn new(
        lease_id: i64,
        sender: tokio::sync::mpsc::Sender<LeaseKeepAliveRequest>,
        response: Response<Streaming<LeaseKeepAliveResponse>>,
    ) -> Self {
        DefaultKeepAliveHandler {
            lease_id,
            sender,
            response,
        }
    }
}

#[async_trait]
impl KeepAliveHandler for DefaultKeepAliveHandler {
    fn lease_id(&self) -> i64 {
        self.lease_id
    }

    fn into_response(self) -> Response<Streaming<LeaseKeepAliveResponse>> {
        self.response
    }

    async fn keep_alive(&mut self) -> Result<(), Error> {
        Ok(self
            .sender
            .send(LeaseKeepAliveRequest { id: self.lease_id })
            .await
            .map_err(|e| Error::KeepAliveError(e.to_string()))?)
    }
}

#[derive(Clone)]
pub struct DefaultLeaseClient {
    inner: lease_client::LeaseClient<Channel>,
}

impl DefaultLeaseClient {
    pub fn new(options: LeaseOptions) -> Self {
        DefaultLeaseClient {
            inner: lease_client::LeaseClient::new(options.channel().clone()),
        }
    }
}

#[async_trait]
impl LeaseClient for DefaultLeaseClient {
    async fn grant_with_options(
        &mut self,
        ttl: Duration,
        options: GrantOptions,
    ) -> Result<Response<LeaseGrantResponse>, Error> {
        Ok(self.inner.lease_grant(options.to_request(&ttl)).await?)
    }

    async fn revoke(&self, lease_id: i64) -> Result<Response<LeaseRevokeResponse>, Error> {
        let request = Request::new(LeaseRevokeRequest { id: lease_id });
        Ok(self.inner.clone().lease_revoke(request).await?)
    }

    async fn keep_alive(&mut self, lease_id: i64) -> Result<impl KeepAliveHandler, Error> {
        let (tx, rx) = tokio::sync::mpsc::channel::<LeaseKeepAliveRequest>(8);

        // 先尝试发送第一条（如果失败，直接返回错误）
        tx.send(LeaseKeepAliveRequest { id: lease_id })
            .await
            .map_err(|e| Error::KeepAliveError(e.to_string()))?;

        let request_stream = ReceiverStream::new(rx);
        let response = self
            .inner
            .lease_keep_alive(Request::new(request_stream))
            .await?;

        let mut streaming = response.into_inner();

        let id = match streaming.message().await? {
            None => {
                return Err(Error::KeepAliveError(
                    "Failed to create keep-alive stream: no response received".to_string(),
                ));
            }
            Some(resp) => {
                if resp.id != lease_id {
                    return Err(Error::KeepAliveError(
                        "Failed to create keep-alive stream: lease ID mismatch".to_string(),
                    ));
                }
                resp.id
            }
        };

        Ok(DefaultKeepAliveHandler::new(
            id,
            tx,
            Response::new(streaming),
        ))
    }

    async fn time_to_live_with_options(&mut self, lease_id: i64, options: TimeToLiveOptions) -> Result<Response<LeaseTimeToLiveResponse>, Error> {
        let request = options.to_request(lease_id);
        Ok(self.inner.lease_time_to_live(request).await?)
    }
}
