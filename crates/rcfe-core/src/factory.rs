use crate::{client::Client, error::Error, options::client::ClientOptions};
use tonic::async_trait;

/// A factory trait for creating clients with specified options.
/// This trait is asynchronous and can be implemented for different client types.
/// # Examples
/// ```rust
/// use rcfe_core::{
///     ClientFactory,
///     ClientOptions,
///     DefaultClientFactory,
///     Error
/// };
///
/// #[tokio::main]
/// async fn main() -> Result<(), Error> {
///     let factory = DefaultClientFactory;
///     let options = ClientOptions::builder()
///         .endpoints(vec!["http://localhost:2379"])
///         .build();
///
///     let client = factory.create(options).await?;
///     println!("Client created with options: {:?}", client.get_options());
///
///     Ok(())
/// }
/// ```
#[async_trait]
pub trait ClientFactory<T>
where
    T: Client,
{
    /// Asynchronously creates a client with the given options.
    /// # Arguments
    /// * `opts` - The client options to configure the client.
    /// # Returns
    /// * `Result<T, Error>` - The created client or an error if creation fails.
    /// # Examples
    /// ```rust
    /// use rcfe_core::{
    ///     ClientFactory,
    ///     ClientOptions,
    ///     DefaultClientFactory,
    ///     Error
    /// };
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     let factory = DefaultClientFactory;
    ///     let options = ClientOptions::builder()
    ///         .endpoints(vec!["http://localhost:2379"])
    ///         .build();
    ///
    ///     let client = factory.create(options).await?;
    ///     println!("Client created with options: {:?}", client.get_options());
    ///     Ok(())
    /// }
    /// ```
    /// # Errors
    /// Returns an `Error` if the client creation fails.
    async fn create(&self, opts: ClientOptions) -> Result<T, Error>;
}
