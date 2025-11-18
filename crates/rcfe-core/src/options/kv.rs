use tonic::transport::Channel;
use crate::error::Error;

#[derive(Debug, Clone)]
pub struct KVOptions {
    channel: Channel,
}

impl KVOptions {
    pub fn channel(self) -> Channel {
        self.channel
    }

    pub fn builder() -> KVOptionsBuilder {
        KVOptionsBuilder {
            channel: None,
        }
    }
}

pub struct KVOptionsBuilder {
    channel: Option<Channel>,
}

impl KVOptionsBuilder {
    pub fn channel(mut self, channel: Channel) -> Self {
        self.channel = Some(channel);
        self
    }

    pub fn build(self) -> Result<KVOptions, Error> {
        let channel = self.channel.ok_or(Error::IllegalArgument(String::from("channel not specified")))?;
        Ok(KVOptions { channel })
    }
}