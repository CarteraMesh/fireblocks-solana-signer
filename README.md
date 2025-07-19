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
fireblocks-solana-signer = "1"
```

Or install via cargo:

```bash
cargo add fireblocks-solana-signer@1
```

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
| FIREBLOCKS_PUBKEY        | **optional** pubkey, or lookup based on `FIREBLOCKS_VAULT` |
| FIREBLOCKS_DEVNET        | set to any value if you are on devnet                 |
| FIREBLOCKS_VAULT         | your vault id                                         |
| FIREBLOCKS_POLL_TIMEOUT  | in seconds, total time to check status of transaction |
| FIREBLOCKS_POLL_INTERVAL | in seconds                                            |

## Configuration Files (Optional)

As an alternative to environment variables, you can use configuration files with the `config` feature. This provides a more structured approach to managing multiple Fireblocks environments and credentials.

### Enabling the Config Feature

Add the `config` feature to your `Cargo.toml`:

```toml
[dependencies]
fireblocks-solana-signer = { version = "1", features = ["config"] }
```

### Configuration File Setup

The config feature uses the [`fireblocks-config`](https://docs.rs/fireblocks-config/latest/fireblocks_config/) crate for configuration management. Configuration files are stored in the `~/.config/fireblocks/` directory using the `microxdg` crate.

**File Structure:**
- **Default configuration**: `~/.config/fireblocks/default.toml` (always loaded)
- **Profile configurations**: `~/.config/fireblocks/{profile}.toml` (override default settings)

**Example `~/.config/fireblocks/default.toml`:**
```toml
api_key = "your-sandbox-api-key-uuid"
secret_path = "/path/to/your/sandbox-private-key.pem"
url = "https://sandbox-api.fireblocks.io"
mainnet = false

[signer]
vault = "your-sandbox-vault-id"
poll_timeout = 30
poll_interval = 2
```

**Example `~/.config/fireblocks/production.toml`:**
```toml
api_key = "your-production-api-key"
secret_path = "/path/to/production-key.pem"
url = "https://api.fireblocks.io"
mainnet = true

[signer]
vault = "your-production-vault-id"
poll_timeout = 60
poll_interval = 3
```

### Using Configuration Files

```rust,no_run
use fireblocks_solana_signer::FireblocksSigner;

fn main() -> anyhow::Result<()> {
    // Use default configuration profile
    let signer = FireblocksSigner::try_from_config::<String>(
        &[],
        |tx_response| println!("Transaction status: {}", tx_response)
    )?;

    // Use specific configuration profiles
    let signer = FireblocksSigner::try_from_config(
        &["mainnet"],
        |tx_response| eprintln!("Mainnet TX: {}", tx_response)
    )?;

    // Use multiple profiles (later profiles override earlier ones)
    let signer = FireblocksSigner::try_from_config(
        &["default", "production"],
        |tx_response| println!("TX Update: {}", tx_response)
    )?;

    // Your transaction code here...
    Ok(())
}
```

**How Profile Loading Works:**
- Empty slice `&[]`: Loads only `~/.config/fireblocks/default.toml`
- Single profile `&["production"]`: Loads `default.toml` first, then `production.toml` overrides any matching settings
- Multiple profiles `&["staging", "production"]`: Loads `default.toml`, then `staging.toml`, then `production.toml` (each overriding previous values)

### Benefits of Configuration Files

- **Multiple Environments**: Easily switch between sandbox, testnet, and mainnet
- **Profile Management**: Organize different configurations by environment or use case
- **Version Control**: Configuration files can be committed (without secrets) for team sharing
- **Validation**: Built-in validation and error handling for configuration values
- **Flexibility**: Override specific settings per profile while inheriting defaults

### Configuration vs Environment Variables

| Method | Best For | Pros | Cons |
|--------|----------|------|------|
| Environment Variables | Simple setups, CI/CD | Easy to set, widely supported | Hard to manage multiple environments |
| Configuration Files | Complex setups, multiple environments | Organized, version-controllable, flexible | Requires additional feature, more setup |

For detailed configuration options and file locations, see the [`fireblocks-config` documentation](https://docs.rs/fireblocks-config/latest/fireblocks_config/).

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
