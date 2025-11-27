use crate::{Compare, Error, GrpcKVClient, RequestOp, Txn, TxnRequest, TxnResponse};
use tonic::{Response, async_trait, transport::Channel};

pub struct DefaultTxn {
    /// The list of comparisons to evaluate. like Txn.is
    when_compares: Vec<Compare>,

    /// The list of operations to execute if all comparisons succeed. like Txn.then
    then_ops: Vec<RequestOp>,

    /// The list of operations to execute if any comparison fails. like Txn.els
    otherwise_ops: Vec<RequestOp>,

    seen_then: bool,
    seen_otherwise: bool,
    kv_client: GrpcKVClient<Channel>,
}

impl DefaultTxn {
    pub fn new(kv_client: GrpcKVClient<Channel>) -> Self {
        DefaultTxn {
            kv_client,
            when_compares: Vec::new(),
            then_ops: Vec::new(),
            otherwise_ops: Vec::new(),
            seen_then: false,
            seen_otherwise: false,
        }
    }
}

#[async_trait]
impl Txn for DefaultTxn {
    fn when<I, P>(&mut self, compares: I) -> Result<&mut Self, Error>
    where
        I: IntoIterator<Item = P>,
        P: Into<Compare>,
    {
        if self.seen_then {
            return Err(Error::InvalidTxnSequence(
                "Cannot add comparisons after 'then' clause!".into(),
            ));
        }

        if self.seen_otherwise {
            return Err(Error::InvalidTxnSequence(
                "Cannot add comparisons after 'otherwise' clause!".into(),
            ));
        }
        for compare in compares {
            self.when_compares.push(compare.into());
        }
        Ok(self)
    }

    fn then<I, P>(&mut self, ops: I) -> Result<&mut Self, Error>
    where
        I: IntoIterator<Item = P>,
        P: Into<RequestOp>,
    {
        if self.seen_otherwise {
            Err(Error::InvalidTxnSequence(
                "Cannot add 'then' clause after 'otherwise' clause!".into(),
            ))
        } else {
            self.seen_then = true;
            for op in ops {
                self.then_ops.push(op.into());
            }
            Ok(self)
        }
    }

    fn otherwise<I, P>(&mut self, ops: I) -> Result<&mut Self, Error>
    where
        I: IntoIterator<Item = P>,
        P: Into<RequestOp>,
    {
        self.seen_otherwise = true;
        for op in ops {
            self.otherwise_ops.push(op.into());
        }
        Ok(self)
    }

    async fn commit(&mut self) -> Result<Response<TxnResponse>, Error> {
        // Here you would typically send the transaction to the etcd server
        // and return the response. This is a placeholder implementation.
        let mut txn_request = TxnRequest::default();

        if !self.when_compares.is_empty() {
            txn_request.compare = self
                .when_compares
                .iter()
                .cloned()
                .map(|c| c.into())
                .collect();
        }

        if !self.then_ops.is_empty() {
            txn_request.success = self.then_ops.iter().cloned().map(|c| c.into()).collect();
        }

        if !self.otherwise_ops.is_empty() {
            txn_request.failure = self
                .otherwise_ops
                .iter()
                .cloned()
                .map(|c| c.into())
                .collect();
        }

        // Send txn_request to etcd server and get response
        Ok(self.kv_client.txn(txn_request).await?)
    }
}
