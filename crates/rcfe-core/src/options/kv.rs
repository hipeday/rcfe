use crate::{ByteSequence, NamespaceBuilder, Namespaceable, error::Error};
use tonic::transport::Channel;

/// Options for KVClient
#[derive(Debug, Clone)]
pub struct KVOptions {
    channel: Channel,
    namespace: Option<ByteSequence>,
}

/// Builder for KVOptions
#[derive(Debug, Clone)]
pub struct KVOptionsBuilder {
    channel: Option<Channel>,
    namespace: Option<ByteSequence>,
}

impl Namespaceable for KVOptions {
    fn namespace(&self) -> Option<ByteSequence> {
        self.namespace.clone()
    }
}

impl KVOptions {
    pub fn channel(self) -> Channel {
        self.channel
    }

    /// Creates a builder for KVOptions
    pub fn builder() -> KVOptionsBuilder {
        KVOptionsBuilder {
            channel: None,
            namespace: None,
        }
    }
}

impl NamespaceBuilder for KVOptionsBuilder {
    fn namespace<N>(mut self, namespace: Option<N>) -> Self
    where
        N: Into<ByteSequence>
    {
        if let Some(ns) = namespace {
            self.namespace = Some(ns.into());
        };
        self
    }
}

impl KVOptionsBuilder {
    pub fn channel(mut self, channel: Channel) -> Self {
        self.channel = Some(channel);
        self
    }

    /// Builds the KVOptions
    pub fn build(self) -> Result<KVOptions, Error> {
        let channel = self.channel.ok_or(Error::IllegalArgument(String::from(
            "channel not specified",
        )))?;
        Ok(KVOptions { channel, namespace: self.namespace })
    }
}
