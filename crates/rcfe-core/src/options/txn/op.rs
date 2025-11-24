use crate::{
    ByteSequence,
    etcdserverpb::{
        DeleteRangeRequest, PutRequest, RangeRequest, RequestOp as PbRequestOp,
        request_op::Request::{RequestDeleteRange, RequestPut, RequestRange},
    },
};

#[derive(Debug, Clone)]
pub enum RequestOp {
    Put {
        key: ByteSequence,
        value: ByteSequence,
    },
    Get {
        key: ByteSequence,
        range_end: Option<ByteSequence>,
    },
    Delete {
        key: ByteSequence,
        range_end: Option<ByteSequence>,
    },
}

impl RequestOp {
    pub fn into_pb(self) -> PbRequestOp {
        match self {
            RequestOp::Put { key, value } => PbRequestOp {
                request: Some(RequestPut(PutRequest {
                    key: key.into(),
                    value: value.into(),
                    ..Default::default()
                })),
            },
            RequestOp::Get { key, range_end } => PbRequestOp {
                request: Some(RequestRange(RangeRequest {
                    key: key.into(),
                    range_end: range_end.map_or(ByteSequence::empty().into(), |re| re.into()),
                    ..Default::default()
                })),
            },
            RequestOp::Delete { key, range_end } => PbRequestOp {
                request: Some(RequestDeleteRange(DeleteRangeRequest {
                    key: key.into(),
                    range_end: range_end.map_or(ByteSequence::empty().into(), |re| re.into()),
                    ..Default::default()
                })),
            },
        }
    }
}

impl Into<PbRequestOp> for RequestOp {
    fn into(self) -> PbRequestOp {
        self.into_pb()
    }
}