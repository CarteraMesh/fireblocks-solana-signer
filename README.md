<div align="center">
  <h1><code>fireblocks-solana-signer</code></h1>
  <a href="https://docs.rs/fireblocks-solana-signer/">
    <img src="https://docs.rs/fireblocks-solana-signer/badge.svg" alt="docs" height="25">
  </a>
  <a href="https://github.com/CarteraMesh/fireblocks-solana-signer/actions">
    <img src="https://github.com/CarteraMesh/fireblocks-solana-signer/actions/workflows/test.yml/badge.svg" alt="build" height="25">
  </a>
  <a href="https://deps.rs/repo/github/CarteraMesh/fireblocks-solana-signer">
    <img src="https://deps.rs/repo/github/CarteraMesh/fireblocks-solana-signer/status.svg" alt="deps" height="25">
  </a>
  <a href="https://codecov.io/github/CarteraMesh/fireblocks-solana-signer" >
   <img src="https://codecov.io/github/CarteraMesh/fireblocks-solana-signer/graph/badge.svg?token=dILa1k9tlW" alt="codecov" height="25"/>
 </a>
  <a href="https://crates.io/crates/fireblocks-solana-signer">
    <img src="https://img.shields.io/crates/v/fireblocks-solana-signer.svg" height="25" alt="crate">
  </a>
</div>

# Overview

Implementation of a Solana [Signer](https://docs.rs/solana-signer/2.2.1/solana_signer/trait.Signer.html) using Fireblocks as backend signer

## Prerequisites

A fireblocks account with API key.
See developer [portal](https://developers.fireblocks.com/docs/introduction) and sign up for a [sandbox](https://developers.fireblocks.com/docs/sandbox-quickstart) account

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
fireblocks-solana-signer = "1.0.8"
```

Or install via cargo:

```bash
cargo add fireblocks-solana-signer@1.0.8
```

### Optional Features

The crate provides optional features that can be enabled in your `Cargo.toml`:

```toml
[dependencies]
fireblocks-solana-signer = { version = "1.0.4", features = ["tokio", "rustls-tls"] }
```

- **tokio**: Enables async support using tokio's `spawn_blocking` for non-blocking operations in async applications
- **rustls-tls**: Uses rustls instead of native-tls for TLS support

## ⚠️ IMPORTANT: Automatic Transaction Broadcasting

**This signer automatically broadcasts transactions to the Solana network.** When you call any signing method (like `try_sign`), Fireblocks will:

1. Sign the transaction with your private key
2. **Automatically broadcast the signed transaction to the network**
3. Return the signature to your application

This is a **purposeful security design decision** by Fireblocks to ensure transaction integrity. **You do not need to (and should not) broadcast the transaction yourself** after signing.

The transaction is already on-chain when the signing method returns successfully!

## TLDR


```rust,no_run
use {
    fireblocks_solana_signer::FireblocksSigner,
    solana_message::Message,
    solana_rpc_client::rpc_client::{RpcClient, SerializableTransaction},
    solana_sdk::instruction::Instruction,
    solana_transaction::Transaction,
};

fn memo(message: &str) -> Instruction {
    Instruction {
        program_id: spl_memo::id(),
        accounts: vec![],
        data: message.as_bytes().to_vec(),
    }
}

fn main() -> anyhow::Result<()> {
    let signer: FireblocksSigner = FireblocksSigner::try_from_env(None)?;
    let rpc = RpcClient::new(
        std::env::var("RPC_URL")
            .ok()
            .unwrap_or("https://rpc.ankr.com/solana_devnet".to_string()),
    );
    let hash = rpc.get_latest_blockhash()?;
    let message = Message::new(&[memo("fireblocks signer")], Some(&signer.pk));
    let mut tx = Transaction::new_unsigned(message);
    
    // ⚠️ This signs AND broadcasts the transaction automatically!
    tx.try_sign(&[&signer], hash)?;
    
    // ✅ Transaction is already on-chain, just get the signature
    println!("Transaction broadcasted with signature: {}", tx.get_signature());
    
    // ❌ DO NOT do this - transaction is already broadcasted!
    // rpc.send_transaction(&tx)?; // This will likely fail
    
    Ok(())
}
```

See [example](./examples/memo.rs) 

## Environment Variables

| Var                      | Example                                               |
|--------------------------|-------------------------------------------------------|
| FIREBLOCKS_SECRET        | RSA private key of your API user                      |
| FIREBLOCKS_API_KEY       | uuid of api user                                      |
| FIREBLOCKS_ENDPOINT      | https://sandbox-api.fireblocks.io                     |
| FIREBLOCKS_DEVNET        | set to any value if you are on devnet                 |
| FIREBLOCKS_VAULT         | your vault id                                         |
| FIREBLOCKS_POLL_TIMEOUT  | in seconds, total time to check status of transaction |
| FIREBLOCKS_POLL_INTERVAL | in seconds                                            |

## Development

### Prerequisites

- **Rust Nightly**: Required for code formatting with advanced features
  ```bash
  rustup install nightly
  ```

- **Environment Setup**: Create a `.env` file with your Fireblocks credentials
  ```bash
  cp env-sample .env
  # Edit .env with your actual Fireblocks API credentials
  ```

### Getting Started

1. **Clone the repository**
   ```bash
   git clone https://github.com/CarteraMesh/fireblocks-solana-signer.git
   cd fireblocks-solana-signer
   ```

2. **Set up environment**
   ```bash
   # Copy and configure environment variables
   cp env-sample .env
   
   # Install Rust nightly for formatting
   rustup install nightly
   ```

3. **Build and test**
   ```bash
   # Build the project
   cargo build
   
   # Run tests (requires valid Fireblocks credentials in .env)
   cargo test
   
   # Format code (requires nightly)
   cargo +nightly fmt --all
   ```

### Code Formatting

This project uses advanced Rust formatting features that require nightly:

```bash
# Format all code
cargo +nightly fmt --all

# Check formatting
cargo +nightly fmt --all -- --check
```

### Running Examples

```bash
# Make sure your .env file is configured first
cargo run --example memo
```
