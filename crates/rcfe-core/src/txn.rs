use crate::{
    error::Error,
    etcdserverpb::TxnResponse,
    options::txn::{compare::Compare, op::RequestOp},
};
use tonic::Response;

/// Trait representing a transaction in the key-value store.
/// # Examples
/// ```rust
/// use rcfe_core::txn::Txn;
/// use rcfe_core::options::txn::compare::Compare;
/// use rcfe_core::options::txn::op::RequestOp;
/// use rcfe_core::error::Error;
/// use tonic::Response;
/// use rcfe_core::etcdserverpb::TxnResponse;
///
/// async fn example<T: Txn>(txn: &mut T, compares: Vec<Compare>, then_ops: Vec<RequestOp>, otherwise_ops: Vec<RequestOp>) -> Result<Response<TxnResponse>, Error> {
///     txn.when(compares)?
///        .then(ops)?
///        .otherwise(otherwise_ops)?
///        .commit()
///        .await?
/// }
#[tonic::async_trait]
pub trait Txn {
    /// Adds comparison conditions to the transaction.
    /// # Arguments
    /// * `compares` - An iterable collection of comparison conditions.
    /// # Returns
    /// * `Result<&mut Self, error::Error>` - A mutable reference to the transaction or an error.
    /// # Examples
    /// ```rust
    /// use rcfe_core::txn::Txn;
    /// use rcfe_core::options::txn::compare::Compare;
    /// use rcfe_core::error::Error;
    /// fn example<T: Txn>(txn: &mut T, compares: Vec<Compare>) -> Result<&mut T, Error> {
    ///     txn.when(compares)
    /// }
    /// ```
    fn when<I, P>(&mut self, compares: I) -> Result<&mut Self, Error>
    where
        I: IntoIterator<Item = P>,
        P: Into<Compare>;

    /// Adds operations to be executed if the comparison conditions are met.
    /// # Arguments
    /// * `ops` - An iterable collection of operations.
    /// # Returns
    /// * `Result<&mut Self, error::Error>` - A mutable reference to the transaction or an error.
    /// # Examples
    /// ```rust
    /// use rcfe_core::txn::Txn;
    /// use rcfe_core::options::txn::op::RequestOp;
    /// use rcfe_core::error::Error;
    ///
    /// fn example<T: Txn>(txn: &mut T, ops: Vec<RequestOp>) -> Result<&mut T, Error> {
    ///     txn.then(ops)
    /// }
    /// ```
    fn then<I, P>(&mut self, ops: I) -> Result<&mut Self, Error>
    where
        I: IntoIterator<Item = P>,
        P: Into<RequestOp>;

    /// Adds operations to be executed if the comparison conditions are not met.
    /// # Arguments
    /// * `ops` - An iterable collection of operations.
    /// # Returns
    /// * `Result<&mut Self, error::Error>` - A mutable reference to the transaction or an error.
    /// # Examples
    /// ```rust
    /// use rcfe_core::txn::Txn;
    /// use rcfe_core::options::txn::op::RequestOp;
    /// use rcfe_core::error::Error;
    ///
    /// fn example<T: Txn>(txn: &mut T, ops: Vec<RequestOp>) -> Result<&mut T, Error> {
    ///     txn.otherwise(ops)
    /// }
    /// ```
    fn otherwise<I, P>(&mut self, ops: I) -> Result<&mut Self, Error>
    where
        I: IntoIterator<Item = P>,
        P: Into<RequestOp>;

    /// Commits the transaction and executes the operations.
    /// # Returns
    /// * `Result<Response<TxnResponse>, error::Error>` - The response containing the transaction result or an error.
    /// # Examples
    /// ```rust
    /// use rcfe_core::txn::Txn;
    /// use rcfe_core::error::Error;
    /// use tonic::Response;
    /// use rcfe_core::etcdserverpb::TxnResponse;
    ///
    /// async fn example<T: Txn>(txn: &mut T) -> Result<Response<TxnResponse>, Error> {
    ///     txn.commit().await
    /// }
    /// ```
    async fn commit(&mut self) -> Result<Response<TxnResponse>, Error>;
}
