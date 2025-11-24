use crate::{
    ByteSequence,
    etcdserverpb::DeleteRangeRequest
};

/// Options for deleting keys in the key-value store
/// # Fields
/// * `prefix` - A key is treated as a prefix
/// * `prev_kv` - Return the previous key-value pair before deletion
/// # Examples
/// ```rust
/// use rcfe_core::options::kv::DeleteOptions;
/// let delete_options = DeleteOptions {
///     prefix: true,
///     prev_kv: false,
/// };
/// ```
#[derive(Default, Debug, Clone)]
pub struct DeleteOptions {
    /// A key is treated as a prefix
    pub prefix: bool,
    /// Return the previous key-value pair before deletion
    pub prev_kv: bool,
}

impl DeleteOptions {
    /// Creates a builder for DeleteOptions
    /// # Examples
    /// ```rust
    /// use rcfe_core::options::kv::{DeleteOptions, DeleteOptionsBuilder};
    /// let delete_options = DeleteOptions::builder()
    ///     .prefix(true)
    ///     .prev_kv(true)
    ///     .build();
    /// ```
    pub fn builder() -> DeleteOptionsBuilder {
        DeleteOptionsBuilder::default()
    }

    /// Converts DeleteOptions to an etcdserverpb::DeleteRangeRequest
    /// # Examples
    /// ```rust
    /// use rcfe_core::options::kv::DeleteOptions;
    /// use rcfe_core::ByteSequence;
    /// use rcfe_core::etcdserverpb::DeleteRangeRequest;
    /// let delete_options = DeleteOptions::builder()
    ///     .prefix(true)
    ///     .prev_kv(true)
    ///     .build();
    /// let key = ByteSequence::from("my_key");
    /// let request: DeleteRangeRequest = delete_options.to_request(key);
    /// ```
    pub fn to_request(&self, key: &ByteSequence) -> DeleteRangeRequest {
        let mut request = DeleteRangeRequest {
            key: key.to_vec(),
            ..Default::default()
        };

        if self.prefix {
            request.range_end = key.next().to_vec();
        }

        request.prev_kv = self.prev_kv;

        request
    }
}

/// Builder for DeleteOptions
/// # Examples
/// ```rust
/// use rcfe_core::options::kv::DeleteOptionsBuilder;
/// let delete_options = DeleteOptionsBuilder::default()
///     .prefix(true)
///     .prev_kv(true)
///     .build();
/// ```
#[derive(Default, Debug, Clone)]
pub struct DeleteOptionsBuilder {
    prefix: Option<bool>,
    prev_kv: Option<bool>,
}

impl DeleteOptionsBuilder {
    /// Sets the prefix option for DeleteOptions
    /// # Examples
    /// ```rust
    /// use rcfe_core::options::kv::DeleteOptionsBuilder;
    /// let delete_options = DeleteOptionsBuilder::default()
    ///     .prefix(true)
    ///     .build();
    /// ```
    pub fn prefix(mut self, prefix: bool) -> Self {
        self.prefix = Some(prefix);
        self
    }

    /// Sets the prev_kv option for DeleteOptions
    /// # Examples
    /// ```rust
    /// use rcfe_core::options::kv::DeleteOptionsBuilder;
    /// let delete_options = DeleteOptionsBuilder::default()
    ///    .prev_kv(true)
    ///    .build();
    /// ```
    pub fn prev_kv(mut self, prev_kv: bool) -> Self {
        self.prev_kv = Some(prev_kv);
        self
    }

    /// Builds the DeleteOptions from the builder
    /// # Examples
    /// ```rust
    /// use rcfe_core::options::kv::DeleteOptionsBuilder;
    /// let delete_options = DeleteOptionsBuilder::default()
    ///     .prefix(true)
    ///     .prev_kv(true)
    ///     .build();
    /// ```
    pub fn build(self) -> DeleteOptions {
        let mut options = DeleteOptions::default();
        if let Some(prefix) = self.prefix {
            options.prefix = prefix;
        }

        if let Some(prev_kv) = self.prev_kv {
            options.prev_kv = prev_kv;
        }

        options
    }
}