# RCFE - Rust Client for ETCD V3.

RCFE is a Rust client library for interacting with ETCD V3, a distributed key-value store that provides a reliable way to store data across a cluster of machines. This library aims to provide a simple and efficient interface for Rust developers to work with ETCD.

The name "RCFE" stands for "Rust Client for ETCD".

> [!WARNING]
> This project is still in its early stages of development. APIs may change without notice. Use at your own risk.

## Features

- [ ] Basic KV operations (get, put, delete, transactions)
   - [X] Range Get
   - [X] Put
   - [X] Delete
   - [ ] Transactions

## Usage

Add the following to your `Cargo.toml`:

```toml
[dependencies]
rcfe = "<version>"
```

Replace `<version>` with the latest version of RCFE.

```rust
use rcfe::{ClientFactory, DefaultClient, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
  let client_options = rcfe::ClientOptions::builder()
          .endpoints(vec!["http://localhost:2379"])
          .build();

  let client = rcfe::DefaultClientFactory::new().create(client_options).await?;
  
  // Get a value by key
  let response = kv_client.range(ByteSequence::from("greeting")).await?;
  
  println!("Received response: {:?}", response);
  
  Ok(())
}
```