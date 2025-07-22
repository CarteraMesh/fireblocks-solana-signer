#![cfg(not(feature = "agave"))]

mod utils;
use {
    fireblocks_solana_signer::*,
    solana_message::Message,
    solana_pubkey::{Pubkey, pubkey},
    solana_rpc_client::rpc_client::SerializableTransaction,
    solana_signer::Signer,
    solana_transaction::{Transaction, versioned::VersionedTransaction},
    utils::*,
};
pub const LOOKUP: Pubkey = pubkey!("24DJ3Um2ekF2isQVMZcNHusmzLMMUS1oHQXhpPkVX7WV");
#[allow(dead_code)]
pub const USDC: Pubkey = pubkey!("4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU");
pub const TO: Pubkey = pubkey!("E4SfgGV2v9GLYsEkCQhrrnFbBcYmAiUZZbJ7swKGzZHJ");

#[test]
fn test_signer_legacy() -> anyhow::Result<()> {
    setup();
    let (signer, rpc) = signer()?;
    let hash = rpc.get_latest_blockhash()?;
    let message = Message::new(&[memo("fireblocks signer")], Some(&signer.pk));
    let mut tx = Transaction::new_unsigned(message);
    assert!(signer.is_interactive());
    tx.try_sign(&[&signer], hash)?;
    tracing::info!("sig {}", tx.get_signature());
    Ok(())
}

#[test]
fn test_signer_versioned() -> anyhow::Result<()> {
    setup();
    let (signer, rpc) = signer()?;
    let instructions = vec![
        memo("fireblocks signer versioned"),
        memo("lookup this"),
        solana_sdk::system_instruction::transfer(&signer.pk, &TO, 1),
    ];
    let hash = rpc.get_latest_blockhash()?;
    let alt = utils::lookup(&rpc, &[LOOKUP])?;
    let mut tx = VersionedTransaction::new_unsigned_v0(&signer.pk, &instructions, &alt, hash)?;
    tx.try_sign(&[&signer], None)?;
    tracing::info!("sig {}", tx.get_signature());
    Ok(())
}

#[test]
fn test_env() -> anyhow::Result<()> {
    setup();
    let _ = FireblocksSigner::try_from_env(None)?;
    let _ = FireblocksSigner::try_from_env(Some(|t| println!("{t}")))?;
    Ok(())
}

#[test]
fn test_keypair() -> anyhow::Result<()> {
    setup();
    let (_, rpc) = signer()?;
    let signer = FireblocksSigner::new();
    let hash = rpc.get_latest_blockhash()?;
    let message = Message::new(&[memo("fireblocks signer")], Some(&signer.pk));
    let mut tx = Transaction::new_unsigned(message);
    tx.try_sign(&[&signer], hash)?;
    tracing::info!("sig {}", tx.get_signature());

    let base64 = signer.to_base58_string();
    let from_base64 = FireblocksSigner::from_base58_string(&base64);
    assert_eq!(signer.pk, from_base64.pk);
    let b = signer.to_bytes();
    let from_b = FireblocksSigner::from_bytes(&b)?;
    assert_eq!(signer.pk, from_b.pk);

    Ok(())
}
