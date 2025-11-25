use crate::{
    ByteSequence,
    error::Error,
    etcdserverpb::{
        Compare as PbCompare,
        compare::{
            CompareResult as PbCompareResult, CompareTarget as PbCompareTarget, TargetUnion,
        },
    },
};
use std::{fmt::Display, str::FromStr};

/// The result of a comparison in a transaction.
/// # Examples
/// ```rust
/// use rcfe_core::options::txn::compare::CompareResult;
/// let result = CompareResult::Equal;
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompareResult {
    /// The comparison is equal. as in "==".
    Equal,
    /// The comparison is greater than. as in ">".
    Greater,
    /// The comparison is less than. as in "<".
    Less,
    /// The comparison is not equal. as in "!=".
    NotEqual,
}

impl FromStr for CompareResult {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "==" => Ok(CompareResult::Equal),
            ">" => Ok(CompareResult::Greater),
            "<" => Ok(CompareResult::Less),
            "!=" => Ok(CompareResult::NotEqual),
            _ => Err(Error::IllegalArgument(format!(
                "invalid CompareResult: {}",
                s
            ))),
        }
    }
}

impl Display for CompareResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            CompareResult::Equal => "==".to_string(),
            CompareResult::Greater => ">".to_string(),
            CompareResult::Less => "<".to_string(),
            CompareResult::NotEqual => "!=".to_string(),
        };
        write!(f, "{}", str)
    }
}

impl From<PbCompareResult> for CompareResult {
    fn from(value: PbCompareResult) -> Self {
        match value {
            PbCompareResult::Equal => CompareResult::Equal,
            PbCompareResult::Greater => CompareResult::Greater,
            PbCompareResult::Less => CompareResult::Less,
            PbCompareResult::NotEqual => CompareResult::NotEqual,
        }
    }
}

impl From<CompareResult> for i32 {
    fn from(value: CompareResult) -> Self {
        match value {
            CompareResult::Equal => PbCompareResult::Equal as i32,
            CompareResult::Greater => PbCompareResult::Greater as i32,
            CompareResult::Less => PbCompareResult::Less as i32,
            CompareResult::NotEqual => PbCompareResult::NotEqual as i32,
        }
    }
}

/// The target of the comparison in a transaction.
/// # Examples
/// ```rust
/// use rcfe_core::options::txn::compare::CompareTarget;
/// let target = CompareTarget::Version;
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompareTarget {
    /// Compare based on the version of the key.
    Version,
    /// Compare based on the creation revision of the key.
    Create,
    /// Compare based on the modification revision of the key.
    Mod,
    /// Compare based on the value of the key.
    Value,
}

impl From<PbCompareTarget> for CompareTarget {
    fn from(value: PbCompareTarget) -> Self {
        match value {
            PbCompareTarget::Version => CompareTarget::Version,
            PbCompareTarget::Create => CompareTarget::Create,
            PbCompareTarget::Mod => CompareTarget::Mod,
            PbCompareTarget::Value => CompareTarget::Value,
        }
    }
}

impl From<CompareTarget> for i32 {
    fn from(value: CompareTarget) -> Self {
        match value {
            CompareTarget::Version => PbCompareTarget::Version as i32,
            CompareTarget::Create => PbCompareTarget::Create as i32,
            CompareTarget::Mod => PbCompareTarget::Mod as i32,
            CompareTarget::Value => PbCompareTarget::Value as i32,
        }
    }
}

/// A structure representing a comparison operation in a transaction.
/// It includes the comparison result, target, key, and the specific value to compare against.
/// # Examples
/// ```rust
/// use rcfe_core::options::txn::compare::Compare;
/// let compare = Compare::version_eq("my_key", 5);
/// ```
#[derive(Debug, Clone)]
pub struct Compare {
    pub result: CompareResult,
    pub target: CompareTarget,
    pub key: ByteSequence,
    // oneof content
    pub version: Option<i64>,
    pub create_revision: Option<i64>,
    pub mod_revision: Option<i64>,
    pub value: Option<ByteSequence>,
    pub range_end: Option<ByteSequence>,
}

impl Compare {

    pub fn mod_eq<K: Into<ByteSequence>>(key: K, mv: i64) -> Self {
        Self {
            result: CompareResult::Equal,
            target: CompareTarget::Mod,
            key: key.into(),
            version: None,
            create_revision: None,
            mod_revision: Some(mv),
            value: None,
            range_end: None,
        }
    }

    pub fn version_eq<K: Into<ByteSequence>>(key: K, v: i64) -> Self {
        Self {
            result: CompareResult::Equal,
            target: CompareTarget::Version,
            key: key.into(),
            version: Some(v),
            create_revision: None,
            mod_revision: None,
            value: None,
            range_end: None,
        }
    }

    pub fn value_eq<K, V>(key: K, val: V) -> Self
    where
        K: Into<ByteSequence>,
        V: Into<ByteSequence>,
    {
        Self {
            result: CompareResult::Equal,
            target: CompareTarget::Value,
            key: key.into(),
            version: None,
            create_revision: None,
            mod_revision: None,
            value: Some(val.into()),
            range_end: None,
        }
    }

    pub fn create_eq<K: Into<ByteSequence>>(key: K, rv: i64) -> Self {
        Self {
            result: CompareResult::Equal,
            target: CompareTarget::Create,
            key: key.into(),
            version: None,
            create_revision: Some(rv),
            mod_revision: None,
            value: None,
            range_end: None,
        }
    }

    pub fn builder() -> CompareBuilder {
        CompareBuilder::default()
    }

    /// Set range end for the compare
    /// # Examples
    /// ```rust
    /// use rcfe_core::options::txn::compare::Compare;
    /// use rcfe_core::ByteSequence;
    /// let compare = Compare::version_eq("my_key", 5)
    ///     .with_range_end(ByteSequence::from("my_range_end"));
    /// ```
    pub fn with_range_end(mut self, end: impl Into<ByteSequence>) -> Self {
        self.range_end = Some(end.into());
        self
    }

    /// Convert to protobuf Compare
    /// # Examples
    /// ```rust
    /// use rcfe_core::options::txn::compare::Compare;
    /// let compare = Compare::version_eq("my_key", 5);
    /// let pb_compare = compare.into_pb();
    /// ```
    pub fn into_pb(self) -> PbCompare {
        let mut pb_cmp = PbCompare {
            result: (self.result).into(),
            target: (self.target).into(),
            key: self.key.to_vec(),
            range_end: self.range_end.map(|b| b.to_vec()).unwrap_or_default(),
            ..Default::default()
        };

        if let Some(v) = self.version {
            pb_cmp.target_union = Some(TargetUnion::Version(v));
        } else if let Some(cr) = self.create_revision {
            pb_cmp.target_union = Some(TargetUnion::CreateRevision(cr));
        } else if let Some(mr) = self.mod_revision {
            pb_cmp.target_union = Some(TargetUnion::ModRevision(mr));
        } else if let Some(val) = self.value {
            pb_cmp.target_union = Some(TargetUnion::Value(val.to_vec()));
        }

        pb_cmp
    }
}

/// A builder for creating `Compare` instances.
/// # Examples
/// ```rust
/// use rcfe_core::options::txn::compare::{Compare, CompareBuilder, CompareResult, CompareTarget};
/// use rcfe_core::ByteSequence;
/// let compare = CompareBuilder::default()
///     .result(CompareResult::Equal)
///     .target(CompareTarget::Version)
///     .key("my_key")
///     .version(5)
///     .build();
/// ```
#[derive(Debug, Clone, Default)]
pub struct CompareBuilder {
    result: Option<CompareResult>,
    target: Option<CompareTarget>,
    key: Option<ByteSequence>,
    version: Option<i64>,
    create_revision: Option<i64>,
    mod_revision: Option<i64>,
    value: Option<ByteSequence>,
    range_end: Option<ByteSequence>,
}

impl CompareBuilder {
    pub fn result(mut self, result: CompareResult) -> Self {
        self.result = Some(result);
        self
    }

    pub fn key<K: Into<ByteSequence>>(mut self, key: K) -> Self {
        self.key = Some(key.into());
        self
    }

    pub fn version(mut self, version: i64) -> Self {
        self.version = Some(version);
        self.target = Some(CompareTarget::Version);
        self
    }

    pub fn create_revision(mut self, create_revision: i64) -> Self {
        self.create_revision = Some(create_revision);
        self.target = Some(CompareTarget::Create);
        self
    }

    pub fn mod_revision(mut self, mod_revision: i64) -> Self {
        self.mod_revision = Some(mod_revision);
        self.target = Some(CompareTarget::Mod);
        self
    }

    pub fn value<V: Into<ByteSequence>>(mut self, value: V) -> Self {
        self.value = Some(value.into());
        self.target = Some(CompareTarget::Value);
        self
    }

    pub fn range_end<K: Into<ByteSequence>>(mut self, range_end: K) -> Self {
        self.range_end = Some(range_end.into());
        self
    }

    pub fn build(self) -> Compare {
        Compare {
            result: self.result.expect("CompareResult is required"),
            target: self.target.expect("CompareTarget is required"),
            key: self.key.expect("Key is required"),
            version: self.version,
            create_revision: self.create_revision,
            mod_revision: self.mod_revision,
            value: self.value,
            range_end: self.range_end,
        }
    }
}



impl Into<PbCompare> for Compare {
    fn into(self) -> PbCompare {
        self.into_pb()
    }
}
