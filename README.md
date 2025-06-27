<div align="center">
  <h1><code>fireblocks-solana-signer</code></h1>
  <a href="https://docs.rs/fireblocks-solana-signer/">
    <img src="https://docs.rs/fireblocks-solana-signer/badge.svg" alt="docs" height="25">
  </a>
  <a href="https://github.com/CarteraMesh/fireblocks-solana-signer/actions">
    <img src="https://github.com/CarteraMesh/fireblocks-solana-signer/workflows/Continuous%20integration/badge.svg" alt="build" height="25">
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

## ⚠️ IMPORTANT: Automatic Transaction Broadcasting

**This signer automatically broadcasts transactions to the Solana network.** When you call any signing method (like `try_sign`), Fireblocks will:

1. Sign the transaction with your private key
2. **Automatically broadcast the signed transaction to the network**
3. Return the signature to your application

This is a **purposeful security design decision** by Fireblocks to ensure transaction integrity. **You do not need to (and should not) broadcast the transaction yourself** after signing.

The transaction is already on-chain when the signing method returns successfully!

## TLDR


```rust
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
    let signer: FireblocksSigner = FireblocksSigner::from_env(None)?;
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
