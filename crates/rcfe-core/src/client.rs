use crate::{
    kv::KVClient,
    options::client::ClientOptions,
};

/// Client trait defining the interface for a client.
/// Implementors must provide methods to retrieve client options and a key-value client.
///
/// # Examples
/// ```rust
/// use rcfe_core::Client;
/// use rcfe_core::options::client::ClientOptions;
/// use rcfe_core::kv::KVClient;
///
/// struct MyClient;
///
/// impl Client for MyClient {
///     fn get_options(&self) -> &ClientOptions {
///         unimplemented!()
///     }
///     fn get_kv_client(&self) -> impl KVClient {
///         unimplemented!()
///     }
/// }
/// let my_client = MyClient;
/// let options = my_client.get_options();
/// let kv_client = my_client.get_kv_client();
/// ```
pub trait Client {

    /// Get a reference to the client options.
    /// # Returns
    /// A reference to the `ClientOptions`.
    /// # Examples
    /// ```rust
    /// use rcfe_core::Client;
    /// use rcfe_core::options::client::ClientOptions;
    ///
    /// struct MyClient;
    ///
    /// impl Client for MyClient {
    ///     fn get_options(&self) -> &ClientOptions {
    ///         unimplemented!()
    ///     }
    ///     fn get_kv_client(&self) -> impl KVClient {
    ///         unimplemented!()
    ///     }
    /// }
    /// let my_client = MyClient;
    /// let options = my_client.get_options();
    /// ```
    fn get_options(&self) -> &ClientOptions;

    /// Get the key-value client.
    /// # Returns
    /// An implementation of the `KVClient` trait.
    /// # Examples
    /// ```rust
    /// use rcfe_core::Client;
    /// use rcfe_core::options::client::ClientOptions;
    /// use rcfe_core::kv::KVClient;
    ///
    /// struct MyClient;
    ///
    /// impl Client for MyClient {
    ///     fn get_options(&self) -> &ClientOptions {
    ///         unimplemented!()
    ///     }
    ///     fn get_kv_client(&self) -> impl KVClient {
    ///         unimplemented!()
    ///     }
    /// }
    /// let my_client = MyClient;
    /// let kv_client = my_client.get_kv_client();
    /// ```
    fn get_kv_client(&self) -> impl KVClient;
}