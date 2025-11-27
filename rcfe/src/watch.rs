use crate::{
    Error, GrpcWatchClient, WatchClient, WatchClientOptions, WatchRequest,
    WatchRequestType, WatchResponse, Watcher,
};
use tonic::{async_trait, codegen::tokio_stream::wrappers::ReceiverStream, transport::Channel, Response, Streaming};

pub struct DefaultWatcher {
    id: i64,
    request: WatchRequestType,
    response: Response<Streaming<WatchResponse>>,
    sender: tokio::sync::mpsc::Sender<WatchRequest>,
}

impl DefaultWatcher {
    pub(crate) fn new(
        id: i64,
        request: WatchRequestType,
        response: Response<Streaming<WatchResponse>>,
        sender: tokio::sync::mpsc::Sender<WatchRequest>,
    ) -> Self {
        DefaultWatcher {
            id,
            request,
            response,
            sender,
        }
    }
}

#[async_trait]
impl Watcher for DefaultWatcher {
    fn id(&self) -> i64 {
        self.id
    }

    async fn watch(&mut self) -> Result<(), Error> {
        self.sender
            .send(self.request.to_request())
            .await
            .map_err(|e| Error::WatchError(e.to_string()))
    }

    fn into_response(self) -> Response<Streaming<WatchResponse>> {
        self.response
    }

    fn request(&self) -> &WatchRequestType {
        &self.request
    }

    async fn progress(&mut self) -> Result<(), Error> {
        let request = WatchRequestType::Progress.to_request();
        self.sender
            .send(request)
            .await
            .map_err(|e| Error::WatchError(e.to_string()))
    }

    async fn cancel(&mut self) -> Result<(), Error> {
        let request = WatchRequestType::Cancel(self.id).to_request();
        self.sender
            .send(request)
            .await
            .map_err(|e| Error::WatchError(e.to_string()))
    }
}

#[derive(Clone, Debug)]
pub struct DefaultWatchClient {
    options: WatchClientOptions,
    inner: GrpcWatchClient<Channel>,
}

impl DefaultWatchClient {
    pub fn new(options: WatchClientOptions) -> Self {
        let channel = options.clone().channel();
        DefaultWatchClient {
            options,
            inner: GrpcWatchClient::new(channel),
        }
    }
}

#[async_trait]
impl WatchClient for DefaultWatchClient {
    async fn watch(&mut self, request: WatchRequestType) -> Result<impl Watcher, Error> {
        let watch_request: WatchRequest = request.to_request();

        let (tx, rx) = tokio::sync::mpsc::channel::<WatchRequest>(8);

        tx.send(watch_request)
            .await
            .map_err(|e| Error::WatchError(e.to_string()))?;

        let request_stream = ReceiverStream::new(rx);

        let response = self.inner.watch(request_stream).await?;

        let mut streaming = response.into_inner();
        let watch_id = match streaming.message().await? {
            Some(msg) => msg.watch_id,
            None => {
                return Err(Error::WatchError(
                    "Failed to receive watch ID from server".to_string(),
                ));
            }
        };

        Ok(DefaultWatcher::new(
            watch_id,
            request,
            Response::new(streaming),
            tx,
        ))
    }

    fn options(&self) -> &WatchClientOptions {
        &self.options
    }
}
