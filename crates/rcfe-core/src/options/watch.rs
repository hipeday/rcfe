use crate::{
    ByteSequence, WatchRequest, etcdserverpb::WatchCancelRequest, etcdserverpb::WatchCreateRequest,
};
use tonic::transport::Channel;

#[derive(Debug, Clone)]
pub struct WatchClientOptions {
    channel: Channel,
}

impl WatchClientOptions {
    pub fn new(channel: Channel) -> Self {
        WatchClientOptions { channel }
    }

    pub fn builder() -> WatchClientOptionsBuilder {
        WatchClientOptionsBuilder { channel: None }
    }

    pub fn channel(self) -> Channel {
        self.channel
    }
}

pub struct WatchClientOptionsBuilder {
    channel: Option<Channel>,
}

impl WatchClientOptionsBuilder {
    pub fn channel(mut self, channel: Channel) -> Self {
        self.channel = Some(channel);
        self
    }

    pub fn build(self) -> Result<WatchClientOptions, crate::error::Error> {
        let channel = self
            .channel
            .ok_or(crate::error::Error::IllegalArgument(String::from(
                "channel not specified",
            )))?;
        Ok(WatchClientOptions { channel })
    }
}

#[derive(Debug, Clone)]
pub enum FilterType {
    NoPut,
    NoDelete,
}

#[derive(Debug, Clone)]
pub struct WatchCreateOptions {
    pub key: ByteSequence,
    pub range_end: ByteSequence,
    pub start_revision: i64,
    pub progress_notify: bool,
    pub filters: Vec<FilterType>,
    pub prev_kv: bool,
}

impl WatchCreateOptions {
    pub fn to_request(&self) -> WatchCreateRequest {
        let filter_types = self
            .filters
            .iter()
            .map(|f| match f {
                FilterType::NoPut => 0,    // Assuming 0 represents NO_PUT
                FilterType::NoDelete => 1, // Assuming 1 represents NO_DELETE
            })
            .collect();

        let key = self.key.clone();
        let range_end = self.range_end.clone();

        WatchCreateRequest {
            key: key.into(),
            range_end: range_end.into(),
            start_revision: self.start_revision,
            progress_notify: self.progress_notify,
            filters: filter_types,
            prev_kv: self.prev_kv,
        }
    }

    pub fn builder() -> WatchCreateOptionsBuilder {
        WatchCreateOptionsBuilder {
            key: None,
            range_end: None,
            start_revision: None,
            progress_notify: None,
            filters: vec![],
            prev_kv: None,
        }
    }
}

pub struct WatchCreateOptionsBuilder {
    key: Option<ByteSequence>,
    range_end: Option<ByteSequence>,
    start_revision: Option<i64>,
    progress_notify: Option<bool>,
    filters: Vec<FilterType>,
    prev_kv: Option<bool>,
}

impl WatchCreateOptionsBuilder {
    pub fn key(mut self, key: ByteSequence) -> Self {
        self.key = Some(key);
        self
    }

    pub fn range_end(mut self, range_end: ByteSequence) -> Self {
        self.range_end = Some(range_end);
        self
    }

    pub fn start_revision(mut self, start_revision: i64) -> Self {
        self.start_revision = Some(start_revision);
        self
    }

    pub fn progress_notify(mut self, progress_notify: bool) -> Self {
        self.progress_notify = Some(progress_notify);
        self
    }

    pub fn add_filter(mut self, filter: FilterType) -> Self {
        self.filters.push(filter);
        self
    }

    pub fn prev_kv(mut self, prev_kv: bool) -> Self {
        self.prev_kv = Some(prev_kv);
        self
    }

    pub fn build(self) -> Result<WatchCreateOptions, crate::error::Error> {
        Ok(WatchCreateOptions {
            key: self
                .key
                .ok_or(crate::error::Error::IllegalArgument(String::from(
                    "key not specified",
                )))?,
            range_end: self.range_end.unwrap_or(ByteSequence::from(vec![])),
            start_revision: self.start_revision.unwrap_or(0),
            progress_notify: self.progress_notify.unwrap_or(false),
            filters: self.filters,
            prev_kv: self.prev_kv.unwrap_or(false),
        })
    }
}

#[derive(Debug, Clone)]
pub enum WatchRequestType {
    Create(WatchCreateOptions),
    Cancel(i64),
    Progress,
}

impl WatchRequestType {
    pub fn to_request(&self) -> WatchRequest {
        match self {
            WatchRequestType::Create(e) => WatchRequest {
                request_union: Some(
                    crate::etcdserverpb::watch_request::RequestUnion::CreateRequest(e.to_request()),
                ),
            },
            WatchRequestType::Cancel(watch_id) => WatchRequest {
                request_union: Some(
                    crate::etcdserverpb::watch_request::RequestUnion::CancelRequest(
                        WatchCancelRequest {
                            watch_id: *watch_id,
                        },
                    ),
                ),
            },
            WatchRequestType::Progress => WatchRequest {
                request_union: Some(
                    crate::etcdserverpb::watch_request::RequestUnion::ProgressRequest(
                        crate::etcdserverpb::WatchProgressRequest {},
                    ),
                ),
            },
        }
    }
}
