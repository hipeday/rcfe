use crate::{
    error::Error,
    etcdserverpb::{Compare, RequestOp, TxnResponse},
};
use tonic::Response;

#[tonic::async_trait]
pub trait Txn {
    fn is<I, P>(&mut self, compares: I) -> Result<&mut Self, Error>
    where
        I: IntoIterator<Item = P>,
        P: Into<Compare>;

    fn then<I, P>(&mut self, ops: I) -> Result<&mut Self, Error>
    where
        I: IntoIterator<Item = P>,
        P: Into<RequestOp>;

    fn els<I, P>(&mut self, ops: I) -> Result<&mut Self, Error>
    where
        I: IntoIterator<Item = P>,
        P: Into<RequestOp>;

    async fn commit(&mut self) -> Result<Response<TxnResponse>, Error>;
}
