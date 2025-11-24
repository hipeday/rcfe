# RCFE - Rust Client for ETCD V3.

[![Crates.io](https://img.shields.io/crates/v/rcfe.svg)](https://crates.io/crates/rcfe)
[![Docs.rs](https://docs.rs/rcfe/badge.svg)](https://docs.rs/rcfe)
[![License](https://img.shields.io/crates/l/rcfe.svg)](https://crates.io/crates/rcfe)

RCFE is a Rust client library for interacting with ETCD V3, a distributed key-value store that provides a reliable way to store data across a cluster of machines. This library aims to provide a simple and efficient interface for Rust developers to work with ETCD.

The name "RCFE" stands for "Rust Client for ETCD".

> [!WARNING]
> This project is still in its early stages of development. APIs may change without notice. Use at your own risk.

## Features

- [X] Async/Await support using Tokio
- [ ] Basic KV operations (get, put, delete, transactions)
   - [X] Range Get
   - [X] Put
   - [X] Delete
   - [X] Transactions
   - [ ] Compact
- [ ] Lease management
  - [ ] Grant lease
  - [ ] Revoke lease
  - [ ] Keep-alive lease
- [ ] Watch functionality
- [ ] Authentication support
- [ ] TLS support
- [ ] Comprehensive error handling
- [ ] Documentation and examples
## Usage

Add the following to your `Cargo.toml`:

```toml
[dependencies]
rcfe = "<version>"
```

Replace `<version>` with the latest version of RCFE.

Or use the following command:

```bash
cargo add rcfe
```

Here is a simple example of how to use RCFE to connect to an ETCD server and perform a basic key-value operation:

```rust
use rcfe::{ClientFactory, DefaultClient, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
  let client_options = rcfe::ClientOptions::builder()
          .endpoints(vec!["http://localhost:2379"])
          .build();

  let client = rcfe::DefaultClientFactory::new().create(client_options).await?;
  
  // Get the KV client
  let mut kv_client = client.get_kv_client();
  
  // Get a value by key
  let response = kv_client.get(ByteSequence::from("greeting")).await?;
  
  println!("Received response: {:?}", response);
  
  Ok(())
}
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.