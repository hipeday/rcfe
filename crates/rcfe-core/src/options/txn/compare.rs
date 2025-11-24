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

    // 支持自定义 range_end（例如 prefix 查询）
    pub fn with_range_end(mut self, end: impl Into<ByteSequence>) -> Self {
        self.range_end = Some(end.into());
        self
    }

    /// 转换为 protobuf Compare（用于 txn 请求）
    pub fn into_pb(self) -> PbCompare {
        let mut pb_cmp = PbCompare {
            result: (self.result).into(),
            target: (self.target).into(),
            key: self.key.to_vec(),
            range_end: self.range_end.map(|b| b.to_vec()).unwrap_or_default(),
            // oneof target_union -> set appropriate field below
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

impl Into<PbCompare> for Compare {
    fn into(self) -> PbCompare {
        self.into_pb()
    }
}
