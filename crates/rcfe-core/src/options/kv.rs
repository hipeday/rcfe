use crate::error::Error;
use tonic::transport::Channel;

/// Options for KVClient
#[derive(Debug, Clone)]
pub struct KVOptions {
    channel: Channel,
}

/// Builder for KVOptions
#[derive(Debug, Clone)]
pub struct KVOptionsBuilder {
    channel: Option<Channel>,
}

impl KVOptions {
    pub fn channel(self) -> Channel {
        self.channel
    }

    /// Creates a builder for KVOptions
    /// # Examples
    /// ```rust
    /// use rcfe_core::options::kv::{KVOptions, KVOptionsBuilder};
    /// use tonic::transport::Channel;
    /// let channel = Channel::from_static("http://localhost:2379");
    /// let kv_options = KVOptions::builder()
    ///     .channel(channel)
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn builder() -> KVOptionsBuilder {
        KVOptionsBuilder { channel: None }
    }
}

impl KVOptionsBuilder {
    pub fn channel(mut self, channel: Channel) -> Self {
        self.channel = Some(channel);
        self
    }

    /// Builds the KVOptions
    /// # Errors
    /// Returns an Error if the channel is not specified
    /// # Examples
    /// ```rust
    /// use rcfe_core::options::kv::{KVOptions, KVOptionsBuilder};
    /// use tonic::transport::Channel;
    /// let channel = Channel::from_static("http://localhost:2379");
    /// let kv_options = KVOptions::builder()
    ///     .channel(channel)
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn build(self) -> Result<KVOptions, Error> {
        let channel = self.channel.ok_or(Error::IllegalArgument(String::from(
            "channel not specified",
        )))?;
        Ok(KVOptions { channel })
    }
}
