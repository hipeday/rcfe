use tonic::transport::Channel;
use crate::{
    error::Error,
    etcdserverpb::LeaseTimeToLiveRequest
};

pub mod grant;

pub struct LeaseOptions {
    channel: Channel,
}

impl LeaseOptions {
    pub fn builder() -> LeaseOptionsBuilder {
        LeaseOptionsBuilder::default()
    }

    pub fn channel(&self) -> &Channel {
        &self.channel
    }
}

#[derive(Default)]
pub struct LeaseOptionsBuilder {
    channel: Option<Channel>,
}

impl LeaseOptionsBuilder {
    pub fn channel(mut self, channel: Channel) -> Self {
        self.channel = Some(channel);
        self
    }

    pub fn build(self) -> Result<LeaseOptions, Error> {
        let channel = self.channel.ok_or(Error::IllegalArgument(String::from("channel not specified")))?;
        Ok(LeaseOptions { channel })
    }
}

#[derive(Debug, Default, Clone)]
pub struct TimeToLiveOptions {
    keys: bool,
}

impl TimeToLiveOptions {
    pub fn to_request(&self, lease_id: i64) -> LeaseTimeToLiveRequest {
        LeaseTimeToLiveRequest {
            id: lease_id,
            keys: self.keys,
        }
    }
}

impl TimeToLiveOptions {
    pub fn builder() -> TimeToLiveOptionsBuilder {
        TimeToLiveOptionsBuilder::default()
    }
    
    pub fn keys(&self) -> bool {
        self.keys
    }
}

#[derive(Debug, Default, Clone)]
pub struct TimeToLiveOptionsBuilder {
    keys: Option<bool>,
}

impl TimeToLiveOptionsBuilder {
    pub fn keys(mut self, keys: bool) -> Self {
        self.keys = Some(keys);
        self
    }

    pub fn build(self) -> TimeToLiveOptions {
        TimeToLiveOptions {
            keys: self.keys.unwrap_or(false),
        }
    }
}

