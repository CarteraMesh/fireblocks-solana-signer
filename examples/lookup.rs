mod common;

use {
    fireblocks_solana_signer::FireblocksSigner,
    solana_message::Message,
    solana_pubkey::{Pubkey, pubkey},
    solana_sdk::{
        account::from_account,
        clock::Clock,
        commitment_config::CommitmentConfig,
        instruction::Instruction,
        sysvar,
    },
    solana_signer::Signer,
    solana_transaction::Transaction,
};

const LOOKUP: Pubkey = pubkey!("24DJ3Um2ekF2isQVMZcNHusmzLMMUS1oHQXhpPkVX7WV");
const TO: Pubkey = pubkey!("E4SfgGV2v9GLYsEkCQhrrnFbBcYmAiUZZbJ7swKGzZHJ");

#[cfg(not(feature = "tokio"))]
#[allow(dead_code)]
pub fn lookup_table_instructions(
    rpc: &solana_rpc_client::rpc_client::RpcClient,
    payer: Pubkey,
) -> anyhow::Result<(Vec<Instruction>, Pubkey)> {
    let clock =
        rpc.get_account_with_commitment(&sysvar::clock::id(), CommitmentConfig::finalized())?;

    let clock = clock
        .value
        .ok_or_else(|| anyhow::format_err!("no clock for you"))?;
    let clock_account: Clock =
        from_account(&clock).ok_or(anyhow::format_err!("invalid clock account"))?;
    let (create, account) = solana_sdk::address_lookup_table::instruction::create_lookup_table(
        payer,
        payer,
        clock_account.slot,
    );
    let accounts = vec![
        spl_memo::id(),
        pubkey!("ComputeBudget111111111111111111111111111111"),
        pubkey!("Stake11111111111111111111111111111111111111"),
        pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
        pubkey!("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"), // token ext
    ];
    let extend = solana_sdk::address_lookup_table::instruction::extend_lookup_table(
        account,
        payer,
        Some(payer),
        accounts,
    );
    Ok((vec![create, extend], account))
}

#[cfg(feature = "tokio")]
#[allow(dead_code)]
pub async fn lookup_table_instructions(
    rpc: &solana_rpc_client::nonblocking::rpc_client::RpcClient,
    payer: Pubkey,
) -> anyhow::Result<(Vec<Instruction>, Pubkey)> {
    let clock = rpc
        .get_account_with_commitment(&sysvar::clock::id(), CommitmentConfig::finalized())
        .await?;

    let clock = clock
        .value
        .ok_or_else(|| anyhow::format_err!("no clock for you"))?;
    let clock_account: Clock =
        from_account(&clock).ok_or(anyhow::format_err!("invalid clock account"))?;
    let (create, account) = solana_sdk::address_lookup_table::instruction::create_lookup_table(
        payer,
        payer,
        clock_account.slot,
    );
    let accounts = vec![
        spl_memo::id(),
        pubkey!("ComputeBudget111111111111111111111111111111"),
        pubkey!("Stake11111111111111111111111111111111111111"),
        pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
        pubkey!("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"), // token ext
    ];
    let extend = solana_sdk::address_lookup_table::instruction::extend_lookup_table(
        account,
        payer,
        Some(payer),
        accounts,
    );
    Ok((vec![create, extend], account))
}

#[cfg(not(feature = "tokio"))]
#[allow(dead_code)]
fn create_lookup() -> anyhow::Result<()> {
    use solana_rpc_client::rpc_client::{RpcClient, SerializableTransaction};

    let signer: FireblocksSigner = FireblocksSigner::try_from_env(None)?;
    let rpc = RpcClient::new(
        std::env::var("RPC_URL")
            .ok()
            .unwrap_or("https://rpc.ankr.com/solana_devnet".to_string()),
    );
    let pk = signer.pubkey();
    println!("using pubkey {pk}");
    let hash = rpc.get_latest_blockhash()?;
    let (lookup_create, account) = lookup_table_instructions(&rpc, pk)?;
    println!("Creating lookup table {account}");
    let message = Message::new_with_blockhash(&lookup_create, Some(&pk), &hash);
    let mut tx = Transaction::new_unsigned(message);
    tx.try_sign(&[&signer], hash)?;
    let sig = tx.get_signature();
    println!("sig {sig}");
    Ok(())
}

#[cfg(feature = "tokio")]
#[allow(dead_code)]
async fn create_lookup() -> anyhow::Result<()> {
    use solana_rpc_client::{
        nonblocking::rpc_client::RpcClient,
        rpc_client::SerializableTransaction,
    };

    let signer: FireblocksSigner = FireblocksSigner::try_from_env(None).await?;
    let rpc = RpcClient::new(
        std::env::var("RPC_URL")
            .ok()
            .unwrap_or("https://rpc.ankr.com/solana_devnet".to_string()),
    );
    let pk = signer.pubkey();
    println!("using pubkey {pk}");
    let hash = rpc.get_latest_blockhash().await?;
    let (lookup_create, account) = lookup_table_instructions(&rpc, pk).await?;
    println!("Creating lookup table {account}");
    let message = Message::new_with_blockhash(&lookup_create, Some(&pk), &hash);
    let mut tx = Transaction::new_unsigned(message);
    tx.try_sign(&[&signer], hash)?;
    let sig = tx.get_signature();
    println!("sig {sig}");
    Ok(())
}

#[cfg(not(feature = "tokio"))]
#[allow(dead_code)]
fn append_lookup() -> anyhow::Result<()> {
    use solana_rpc_client::rpc_client::{RpcClient, SerializableTransaction};

    let signer: FireblocksSigner = FireblocksSigner::try_from_env(None)?;
    let rpc = RpcClient::new(
        std::env::var("RPC_URL")
            .ok()
            .unwrap_or("https://rpc.ankr.com/solana_devnet".to_string()),
    );
    let pk = signer.pubkey();
    println!("using pubkey {pk}");
    let hash = rpc.get_latest_blockhash()?;
    let extend = solana_sdk::address_lookup_table::instruction::extend_lookup_table(
        LOOKUP,
        pk,
        Some(pk),
        vec![TO],
    );
    println!("Extending lookup table {TO}");
    let message = Message::new_with_blockhash(&[extend], Some(&pk), &hash);
    let mut tx = Transaction::new_unsigned(message);
    tx.try_sign(&[&signer], hash)?;
    let sig = tx.get_signature();
    println!("sig {sig}");
    Ok(())
}

#[cfg(feature = "tokio")]
#[allow(dead_code)]
async fn append_lookup() -> anyhow::Result<()> {
    use solana_rpc_client::{
        nonblocking::rpc_client::RpcClient,
        rpc_client::SerializableTransaction,
    };

    let signer: FireblocksSigner = FireblocksSigner::try_from_env(None).await?;
    let rpc = RpcClient::new(
        std::env::var("RPC_URL")
            .ok()
            .unwrap_or("https://rpc.ankr.com/solana_devnet".to_string()),
    );
    let pk = signer.pubkey();
    println!("using pubkey {pk}");
    let hash = rpc.get_latest_blockhash().await?;
    let extend = solana_sdk::address_lookup_table::instruction::extend_lookup_table(
        LOOKUP,
        pk,
        Some(pk),
        vec![TO],
    );
    println!("Extending lookup table {TO}");
    let message = Message::new_with_blockhash(&[extend], Some(&pk), &hash);
    let mut tx = Transaction::new_unsigned(message);
    tx.try_sign(&[&signer], hash)?;
    let sig = tx.get_signature();
    println!("sig {sig}");
    Ok(())
}

#[cfg(not(feature = "tokio"))]
fn main() {
    common::setup();
    println!("extend or create lookup table");
    // create_lookup().unwrap();
    // append_lookup().unwrap();
}

#[cfg(feature = "tokio")]
#[tokio::main]
async fn main() {
    common::setup();
    println!("extend or create lookup table");
    // create_lookup().await.unwrap();
    // append_lookup().await.unwrap();
}
