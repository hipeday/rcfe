use crate::ByteSequence;

pub mod client;
pub mod delete;
pub mod get;
pub mod kv;
pub mod put;
pub mod txn;
pub mod compact;
pub mod lease;
pub mod watch;

/// A trait for types that can have an optional namespace.
pub trait Namespaceable {
    fn namespace(&self) -> Option<ByteSequence>;
}

/// A builder trait for setting the namespace on options.
pub trait NamespaceBuilder {
    fn namespace<N>(self, namespace: Option<N>) -> Self
    where
        N: Into<ByteSequence>;
}