use crate::{
    ByteSequence,
    etcdserverpb::{
        DeleteRangeRequest, PutRequest, RangeRequest, RequestOp as PbRequestOp,
        request_op::Request::{RequestDeleteRange, RequestPut, RequestRange},
    },
    options::{delete::DeleteOptions, get::GetOptions, put::PutOptions},
};

#[derive(Debug, Clone)]
pub enum RequestOp {
    Put {
        key: ByteSequence,
        value: ByteSequence,
        options: Option<PutOptions>,
    },
    Get {
        key: ByteSequence,
        options: Option<GetOptions>,
    },
    Delete {
        key: ByteSequence,
        options: Option<DeleteOptions>,
    },
}

impl RequestOp {
    pub fn into_pb(self) -> PbRequestOp {
        match self {
            RequestOp::Put {
                key,
                value,
                options,
            } => PbRequestOp {
                request: Some(RequestPut(options.map_or_else(
                    || PutRequest {
                        key: key.clone().into(),
                        value: value.clone().into(),
                        ..Default::default()
                    },
                    |opts| opts.to_request(&key, &value),
                ))),
            },
            RequestOp::Get { key, options } => PbRequestOp {
                request: Some(RequestRange(options.map_or_else(
                    || RangeRequest {
                        key: key.clone().into(),
                        ..Default::default()
                    },
                    |opts| opts.to_request(&key),
                ))),
            },
            RequestOp::Delete { key, options } => PbRequestOp {
                request: Some(RequestDeleteRange(options.map_or_else(
                    || DeleteRangeRequest {
                        key: key.clone().into(),
                        ..Default::default()
                    },
                    |opts| opts.to_request(&key),
                ))),
            },
        }
    }
}

impl Into<PbRequestOp> for RequestOp {
    fn into(self) -> PbRequestOp {
        self.into_pb()
    }
}
