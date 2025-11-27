use crate::{error::Error, etcdserverpb::WatchResponse, WatchRequestType};
use tonic::{async_trait, Response, Streaming};

#[async_trait]
pub trait Watcher {
    /// Retrieves the ID of the watcher.
    fn id(&self) -> i64;

    /// Starts watching for changes.
    async fn watch(&mut self) -> Result<(), Error>;

    /// Converts the watcher into a response stream of `WatchResponse`.
    fn into_response(self) -> Response<Streaming<WatchResponse>>;

    /// Retrieves the original watch request type.
    fn request(&self) -> &WatchRequestType;

    /// Progresses the watcher to fetch the next set of events.
    async fn progress(&mut self) -> Result<(), Error>;

    /// Cancels the watcher.
    async fn cancel(&mut self) -> Result<(), Error>;
}

#[async_trait]
pub trait WatchClient {

    /// Watches a key or range of keys for changes.
    async fn watch(&mut self, request: WatchRequestType) -> Result<impl Watcher, Error>;

    /// Retrieves the options associated with the WatchClient.
    fn options(&self) -> &crate::WatchClientOptions;
}
