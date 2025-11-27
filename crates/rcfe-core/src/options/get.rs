use crate::{
    ByteSequence, etcdserverpb,
    etcdserverpb::range_request::{SortOrder, SortTarget},
    options::{NamespaceBuilder, Namespaceable},
};

/// Options for Get operations
#[derive(Debug, Clone)]
pub struct GetOptions {
    pub end_key: Option<ByteSequence>, // Optional end key for range queries
    pub limit: i64,                    // limit on number of results
    pub revision: i64,                 // revision to read from
    pub sort_order: SortOrder,         // sort order
    pub sort_target: SortTarget,       // sort target
    pub serializable: bool,            // serializable read
    pub keys_only: bool,               // keys only flag
    pub count_only: bool,              // count only flag
    pub min_mod_revision: i64,         // minimum modification revision
    pub max_mod_revision: i64,         // maximum modification revision
    pub min_create_revision: i64,      // minimum creation revision
    pub max_create_revision: i64,      // maximum creation revision
    pub prefix: bool,                  // prefix flag
    namespace: Option<ByteSequence>,
}

/// Builder for GetOptions
#[derive(Debug, Clone, Default)]
pub struct GetOptionsBuilder {
    end_key: Option<ByteSequence>,
    limit: Option<i64>,
    revision: Option<i64>,
    sort_order: Option<SortOrder>,
    sort_target: Option<SortTarget>,
    serializable: Option<bool>,
    keys_only: Option<bool>,
    count_only: Option<bool>,
    min_mod_revision: Option<i64>,
    max_mod_revision: Option<i64>,
    min_create_revision: Option<i64>,
    max_create_revision: Option<i64>,
    prefix: Option<bool>,
    namespace: Option<ByteSequence>,
}

impl GetOptions {
    fn new() -> Self {
        GetOptions {
            end_key: None,
            limit: 0,
            revision: 0,
            sort_order: SortOrder::None,
            sort_target: SortTarget::Key,
            serializable: false,
            keys_only: false,
            count_only: false,
            min_mod_revision: 0,
            max_mod_revision: 0,
            min_create_revision: 0,
            max_create_revision: 0,
            prefix: false,
            namespace: None,
        }
    }

    /// Creates a builder for GetOptions
    pub fn builder() -> GetOptionsBuilder {
        GetOptionsBuilder::default()
    }

    /// Creates a default GetOptions instance
    pub fn default() -> Self {
        Self::new()
    }

    /// Converts GetOptions to an etcdserverpb::RangeRequest
    pub fn to_request(self, key: &ByteSequence) -> etcdserverpb::RangeRequest {
        let (key, end_key) = match self.namespace {
            None => (
                key.clone(),
                match self.prefix {
                    true => key.next(),
                    false => self.end_key.unwrap_or_else(|| ByteSequence::empty()),
                },
            ),
            Some(mut ns) => {
                let namespaced_key = ns.clone().append(&key);
                let namespaced_end_key = match self.prefix {
                    true => namespaced_key.next(),
                    false => match self.end_key {
                        Some(end_key) => ns.append(&end_key),
                        None => ByteSequence::empty(),
                    },
                };
                (namespaced_key, namespaced_end_key)
            }
        };
        etcdserverpb::RangeRequest {
            key: key.as_bytes().to_vec(),
            range_end: end_key.inner,
            limit: self.limit,
            revision: self.revision,
            sort_order: self.sort_order as i32,
            sort_target: self.sort_target as i32,
            serializable: self.serializable,
            keys_only: self.keys_only,
            count_only: self.count_only,
            min_mod_revision: self.min_mod_revision,
            max_mod_revision: self.max_mod_revision,
            min_create_revision: self.min_create_revision,
            max_create_revision: self.max_create_revision,
        }
    }
}

impl Namespaceable for GetOptions {
    fn namespace(&self) -> Option<ByteSequence> {
        self.namespace.clone()
    }
}

impl GetOptionsBuilder {
    /// Sets the end key for range queries.
    pub fn end_key(mut self, end_key: ByteSequence) -> Self {
        self.end_key = Some(end_key);
        self
    }

    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn revision(mut self, revision: i64) -> Self {
        self.revision = Some(revision);
        self
    }

    pub fn sort_order(mut self, sort_order: SortOrder) -> Self {
        self.sort_order = Some(sort_order);
        self
    }

    pub fn sort_target(mut self, sort_target: SortTarget) -> Self {
        self.sort_target = Some(sort_target);
        self
    }

    pub fn serializable(mut self, serializable: bool) -> Self {
        self.serializable = Some(serializable);
        self
    }

    pub fn keys_only(mut self, keys_only: bool) -> Self {
        self.keys_only = Some(keys_only);
        self
    }

    pub fn count_only(mut self, count_only: bool) -> Self {
        self.count_only = Some(count_only);
        self
    }

    pub fn min_mod_revision(mut self, min_mod_revision: i64) -> Self {
        self.min_mod_revision = Some(min_mod_revision);
        self
    }

    pub fn max_mod_revision(mut self, max_mod_revision: i64) -> Self {
        self.max_mod_revision = Some(max_mod_revision);
        self
    }

    pub fn min_create_revision(mut self, min_create_revision: i64) -> Self {
        self.min_create_revision = Some(min_create_revision);
        self
    }

    pub fn max_create_revision(mut self, max_create_revision: i64) -> Self {
        self.max_create_revision = Some(max_create_revision);
        self
    }

    pub fn prefix(mut self, prefix: bool) -> Self {
        self.prefix = Some(prefix);
        self
    }

    pub fn build(self) -> GetOptions {
        let mut options = GetOptions::new();

        if let Some(end_key) = self.end_key {
            options.end_key = Some(end_key);
        }

        if let Some(limit) = self.limit {
            options.limit = limit;
        }

        if let Some(revision) = self.revision {
            options.revision = revision;
        }

        if let Some(sort_order) = self.sort_order {
            options.sort_order = sort_order;
        }

        if let Some(sort_target) = self.sort_target {
            options.sort_target = sort_target;
        }

        if let Some(serializable) = self.serializable {
            options.serializable = serializable;
        }

        if let Some(keys_only) = self.keys_only {
            options.keys_only = keys_only;
        }

        if let Some(count_only) = self.count_only {
            options.count_only = count_only;
        }

        if let Some(min_mod_revision) = self.min_mod_revision {
            options.min_mod_revision = min_mod_revision;
        }

        if let Some(max_mod_revision) = self.max_mod_revision {
            options.max_mod_revision = max_mod_revision;
        }

        if let Some(min_create_revision) = self.min_create_revision {
            options.min_create_revision = min_create_revision;
        }

        if let Some(max_create_revision) = self.max_create_revision {
            options.max_create_revision = max_create_revision;
        }

        if let Some(prefix) = self.prefix {
            options.prefix = prefix;
        }

        if let Some(namespace) = self.namespace {
            options.namespace = Some(namespace);
        }

        options
    }
}

impl NamespaceBuilder for GetOptionsBuilder {
    fn namespace<N>(mut self, namespace: Option<N>) -> Self
    where
        N: Into<ByteSequence>,
    {
        if let Some(ns) = namespace {
            self.namespace = Some(ns.into());
        }
        self
    }
}
