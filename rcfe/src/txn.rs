use rcfe_core::{
    error::Error,
    etcdserverpb::kv_client::KvClient,
    etcdserverpb::{Compare, RequestOp},
    etcdserverpb::{TxnRequest, TxnResponse},
    txn::Txn,
};
use tonic::{Response, async_trait, transport::Channel};

pub struct DefaultTxn {
    /// The list of comparisons to evaluate. like Txn.is
    compares: Vec<Compare>,

    /// The list of operations to execute if all comparisons succeed. like Txn.then
    then_ops: Vec<RequestOp>,

    /// The list of operations to execute if any comparison fails. like Txn.els
    else_ops: Vec<RequestOp>,

    seen_then: bool,
    seen_else: bool,
    kv_client: KvClient<Channel>,
}

impl DefaultTxn {
    pub fn new(kv_client: KvClient<Channel>) -> Self {
        DefaultTxn {
            kv_client,
            compares: Vec::new(),
            then_ops: Vec::new(),
            else_ops: Vec::new(),
            seen_then: false,
            seen_else: false,
        }
    }
}

#[async_trait]
impl Txn for DefaultTxn {
    fn is<I, P>(&mut self, compares: I) -> Result<&mut Self, Error>
    where
        I: IntoIterator<Item = P>,
        P: Into<Compare>,
    {
        if self.seen_then {
            return Err(Error::InvalidTxnSequence(
                "Cannot add comparisons after 'then' clause!".into(),
            ));
        }

        if self.seen_else {
            return Err(Error::InvalidTxnSequence(
                "Cannot add comparisons after 'else' clause!".into(),
            ));
        }
        for compare in compares {
            self.compares.push(compare.into());
        }
        Ok(self)
    }

    fn then<I, P>(&mut self, ops: I) -> Result<&mut Self, Error>
    where
        I: IntoIterator<Item = P>,
        P: Into<RequestOp>,
    {
        if self.seen_else {
            Err(Error::InvalidTxnSequence(
                "Cannot add 'then' clause after 'else' clause!".into(),
            ))
        } else {
            self.seen_then = true;
            for op in ops {
                self.then_ops.push(op.into());
            }
            Ok(self)
        }
    }

    fn els<I, P>(&mut self, ops: I) -> Result<&mut Self, Error>
    where
        I: IntoIterator<Item = P>,
        P: Into<RequestOp>,
    {
        self.seen_else = true;
        for op in ops {
            self.else_ops.push(op.into());
        }
        Ok(self)
    }

    async fn commit(&mut self) -> Result<Response<TxnResponse>, Error> {
        // Here you would typically send the transaction to the etcd server
        // and return the response. This is a placeholder implementation.
        let mut txn_request = TxnRequest::default();

        if !self.compares.is_empty() {
            txn_request.compare = self.compares.clone();
        }

        if !self.then_ops.is_empty() {
            txn_request.success = self.then_ops.clone();
        }

        if !self.else_ops.is_empty() {
            txn_request.failure = self.else_ops.clone();
        }

        // Send txn_request to etcd server and get response
        Ok(self.kv_client.txn(txn_request).await?)
    }
}
