use crate::{
    ByteSequence,
    options::Namespaceable
};
use crate::options::NamespaceBuilder;

/// Client options for configuring the RCFE client.
/// # Fields
/// * `endpoints` - A vector of endpoint strings for connecting to the RCFE server.
#[derive(Debug, Clone)]
pub struct ClientOptions {
    endpoints: Vec<String>,
    namespace: Option<ByteSequence>,
}

/// Builder for ClientOptions.
#[derive(Default, Debug)]
pub struct ClientOptionsBuilder {
    endpoints: Vec<String>,
    namespace: Option<ByteSequence>,
}

impl Namespaceable for ClientOptions {
    fn namespace(&self) -> Option<ByteSequence> {
        self.namespace.clone()
    }
}

impl ClientOptions {
    /// Returns the list of endpoints.
    /// # Returns
    /// * `&Vec<String>` - A reference to the vector of endpoint strings.
    /// # Example
    /// ```rust
    /// let options = ClientOptions::builder()
    ///     .endpoints(vec!["http://localhost:2379", "http://localhost:2380"])
    ///     .build();
    /// let endpoints = options.endpoints();
    /// ```
    pub fn endpoints(&self) -> &Vec<String> {
        &self.endpoints
    }

    /// Creates a new ClientOptionsBuilder.
    /// # Returns
    /// * `ClientOptionsBuilder` - A new instance of ClientOptionsBuilder.
    /// # Example
    /// ```rust
    /// let builder = ClientOptions::builder();
    /// ```
    pub fn builder() -> ClientOptionsBuilder {
        ClientOptionsBuilder::default()
    }
}

impl NamespaceBuilder for ClientOptionsBuilder {
    fn namespace<N>(mut self, namespace: Option<N>) -> Self
    where
        N: Into<ByteSequence>
    {
        if let Some(namespace) = namespace {
            self.namespace = Some(namespace.into());
        }
        self
    }
}

impl ClientOptionsBuilder {
    /// Sets the endpoints for the client.
    /// # Arguments
    /// * `endpoints` - An iterable collection of endpoint strings.
    /// # Returns
    /// * `Self` - The updated ClientOptionsBuilder.
    /// # Example
    /// ```rust
    /// let builder = ClientOptions::builder()
    ///     .endpoints(vec!["http://localhost:2379", "http://localhost:2380"]);
    /// ```
    pub fn endpoints<I, S>(mut self, endpoints: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let endpoints: Vec<String> = endpoints.into_iter().map(|s| s.into()).collect();
        {
            self.endpoints = endpoints;
            self
        }
    }

    /// Builds the ClientOptions.
    /// # Returns
    /// * `ClientOptions` - The constructed ClientOptions instance.
    /// # Example
    /// ```rust
    /// let options = ClientOptions::builder()
    ///     .endpoints(vec!["http://localhost:2379", "http://localhost:2380"])
    ///     .build();
    /// ```
    pub fn build(self) -> ClientOptions {
        ClientOptions {
            endpoints: self.endpoints,
            namespace: self.namespace,
        }
    }
}
