use crate::etcdserverpb::CompactionRequest;

/// Options for compaction operations.
#[derive(Debug, Clone, Default)]
pub struct CompactOptions {
    /// If true, compaction is physical.
    pub physical: bool,
}

/// Builder for CompactOptions
#[derive(Debug, Clone, Default)]
pub struct CompactOptionsBuilder {
    physical: Option<bool>,
}

impl CompactOptions {
    /// Creates a builder for CompactOptions
    /// # Examples
    /// ```rust
    /// use rcfe_core::options::compact::{CompactOptions, CompactOptionsBuilder};
    /// let compact_options = CompactOptions::builder()
    ///     .physical(true)
    ///     .build();
    /// ```
    pub fn builder() -> CompactOptionsBuilder {
        CompactOptionsBuilder::default()
    }

    /// Converts CompactOptions to CompactionRequest
    /// # Examples
    /// ```rust
    /// use rcfe_core::options::compact::{CompactOptions, CompactOptionsBuilder};
    /// use rcfe_core::etcdserverpb::CompactionRequest;
    /// let compact_options = CompactOptions::builder()
    ///     .physical(true)
    ///     .build();
    /// let compaction_request = compact_options.to_compaction_request(42);
    /// ```
    pub fn to_request(&self, revision: i64) -> CompactionRequest {
        CompactionRequest {
            revision,
            physical: self.physical,
        }
    }
}

impl CompactOptionsBuilder {
    /// Sets whether the compaction is physical.
    /// # Examples
    /// ```rust
    /// use rcfe_core::options::compact::{CompactOptions, CompactOptionsBuilder};
    /// let compact_options = CompactOptions::builder()
    ///     .physical(true)
    ///     .build();
    /// ```
    pub fn physical(mut self, physical: bool) -> Self {
        self.physical = Some(physical);
        self
    }

    /// Builds the CompactOptions
    /// # Examples
    /// ```rust
    /// use rcfe_core::options::compact::{CompactOptions, CompactOptionsBuilder};
    /// let compact_options = CompactOptions::builder()
    ///     .physical(true)
    ///     .build();
    /// ```
    pub fn build(self) -> CompactOptions {
        CompactOptions {
            physical: self.physical.unwrap_or(false),
        }
    }
}
