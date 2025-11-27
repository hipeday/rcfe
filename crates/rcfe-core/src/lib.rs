pub(crate) mod client;
pub(crate) mod error;
pub(crate) mod factory;
pub(crate) mod kv;
pub(crate) mod options;
pub(crate) mod txn;
pub(crate) mod lease;
pub(crate) mod watch;
pub(crate) mod prelude;

pub use prelude::*;

pub mod etcdserverpb {
    tonic::include_proto!("etcdserverpb");
}

pub mod mvccpb {
    tonic::include_proto!("mvccpb");
}

pub mod authpb {
    tonic::include_proto!("authpb");
}

pub mod v3electionpb {
    tonic::include_proto!("v3electionpb");
}

pub mod v3lockpb {
    tonic::include_proto!("v3lockpb");
}

/// A struct representing a sequence of bytes.
const EMPTY_BYTE_SEQUENCE_VALUE: Vec<u8> = Vec::new();
const MAX_BYTE: u8 = 0xFF; // Maximum byte value
const INCREMENT_BYTE: u8 = 0x01; // Increment by 1

/// A struct representing a sequence of bytes.
/// It provides methods to create an empty sequence and compute the next lexicographical sequence.
/// # Examples
/// ```rust
/// let seq = ByteSequence::from("abc");
/// let next_seq = seq.next();
/// assert_eq!(next_seq.as_bytes(), b"abd");
/// ```
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ByteSequence {
    inner: Vec<u8>, // A vector to hold the byte sequence
}

impl ByteSequence {
    pub fn as_bytes(&self) -> &[u8] {
        &self.inner
    }

    /// Creates an empty ByteSequence.
    /// # Examples
    /// ```rust
    /// let empty_seq = ByteSequence::empty();
    /// assert_eq!(empty_seq.as_bytes(), b"\0");
    /// ```
    pub fn empty() -> Self {
        ByteSequence {
            inner: EMPTY_BYTE_SEQUENCE_VALUE,
        }
    }

    /// Computes the next lexicographical byte sequence.
    /// If the current sequence is empty, it returns an empty sequence.
    /// If all bytes are at their maximum value, it returns an empty sequence.
    /// If the last byte can be incremented, it increments it and truncates the sequence.
    /// # Examples
    ///
    /// - Basic test case
    ///
    /// ```rust
    /// let from = ByteSequence::from("abc");
    /// let next_seq = from.next();
    /// assert_eq!(next_seq, ByteSequence::from("abd"));
    /// ```
    ///
    /// - Test case with `0xff` bytes
    ///
    /// ```rust
    /// let from = ByteSequence::from(b"ab\xff\xff" as &[u8]); // [0x61,0x62,0xff,0xff]
    /// let next_seq = from.next();
    /// assert_eq!(next_seq.as_bytes(), b"ac");
    /// ```
    ///
    /// - Test case where all bytes are `0xff`
    ///
    /// ```rust
    /// let from = ByteSequence::from(b"\xff\xff" as &[u8]); // [0xff,0xff]
    /// let next_seq = from.next();
    /// assert_eq!(next_seq.as_bytes(), ByteSequence::empty().as_bytes());
    /// ```
    pub fn next(&self) -> Self {
        if self.inner.is_empty() {
            return ByteSequence::empty();
        }

        let mut end = self.inner.clone();
        // Increment the last byte
        for i in (0..end.len()).rev() {
            if end[i] != MAX_BYTE {
                end[i] = end[i].wrapping_add(INCREMENT_BYTE); // +1
                end.truncate(i + 1);
                return ByteSequence { inner: end };
            }
            // If the byte is MAX_BYTE continue to the previous byte
        }
        // If all bytes are MAX_BYTE, return the escape byte
        ByteSequence::empty()
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.inner.clone()
    }
}

impl From<&str> for ByteSequence {
    fn from(value: &str) -> Self {
        ByteSequence {
            inner: value.as_bytes().to_vec(),
        }
    }
}

impl From<String> for ByteSequence {
    fn from(value: String) -> Self {
        ByteSequence {
            inner: value.as_bytes().to_vec(),
        }
    }
}

impl From<Vec<u8>> for ByteSequence {
    fn from(value: Vec<u8>) -> Self {
        ByteSequence { inner: value }
    }
}

impl From<&[u8]> for ByteSequence {
    fn from(value: &[u8]) -> Self {
        ByteSequence {
            inner: value.to_vec(),
        }
    }
}

impl Into<Vec<u8>> for ByteSequence {
    fn into(self) -> Vec<u8> {
        self.inner
    }
}