mod utils;
use {
    base64::prelude::*,
    fireblocks_solana_signer::*,
    solana_message::{Message, VersionedMessage},
    solana_pubkey::{Pubkey, pubkey},
    solana_rpc_client::rpc_client::{RpcClient, SerializableTransaction},
    solana_signer::Signer,
    solana_transaction::{Transaction, versioned::VersionedTransaction},
    spl_memo_interface::{instruction::build_memo, v3::ID as MEMO_ID},
    test_macros::instrumented_test,
    utils::*,
};
pub const LOOKUP: Pubkey = pubkey!("24DJ3Um2ekF2isQVMZcNHusmzLMMUS1oHQXhpPkVX7WV");
#[allow(dead_code)]
pub const USDC: Pubkey = pubkey!("4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU");
pub const TO: Pubkey = pubkey!("E4SfgGV2v9GLYsEkCQhrrnFbBcYmAiUZZbJ7swKGzZHJ");

fn broadcast(rpc: &RpcClient, tx: &impl SerializableTransaction) -> anyhow::Result<Signature> {
    tracing::info!("broadcasting transaction");
    let sig = rpc.send_and_confirm_transaction(tx)?;
    tracing::info!("sig {sig}");
    Ok(sig)
}

#[test]
#[instrumented_test]
fn test_multi_sig_legacy() -> anyhow::Result<()> {
    let (fireblocks_signer, rpc) = signer()?;
    let kp_secret = std::env::var("TEST_PRIVATE_KEY")?;
    let kp = solana_keypair::Keypair::from_base58_string(&kp_secret);
    let hash = rpc.get_latest_blockhash()?;
    let ins = build_memo(&MEMO_ID, "multi".as_bytes(), &[
        &fireblocks_signer.pubkey(),
        &kp.pubkey(),
    ]);
    let message = Message::new_with_blockhash(&[ins], Some(&fireblocks_signer.pubkey()), &hash);
    let mut tx: Transaction = Transaction::new_unsigned(message);
    kp.try_sign_multi_legacy(&mut tx, &[], hash)?; // don't need to really do this, i'm just testing if Keypair can see this function.
    fireblocks_signer.try_sign_multi_legacy(&mut tx, &[&fireblocks_signer, &kp], hash)?;
    assert!(tx.is_signed());
    broadcast(&rpc, &tx)?;
    Ok(())
}

#[test]
#[instrumented_test]
fn test_multi_sig_not_fee_payer() -> anyhow::Result<()> {
    let (fireblocks_signer, rpc) = signer()?;
    let kp_secret = std::env::var("TEST_PRIVATE_KEY")?;
    let kp = solana_keypair::Keypair::from_base58_string(&kp_secret);
    let hash = rpc.get_latest_blockhash()?;
    let ins = build_memo(&MEMO_ID, "multi order".as_bytes(), &[
        &kp.pubkey(),
        &fireblocks_signer.pubkey(),
    ]);
    let message = Message::new_with_blockhash(&[ins], Some(&kp.pubkey()), &hash);
    let mut tx: Transaction = Transaction::new_unsigned(message);
    kp.try_sign_multi_legacy(&mut tx, &[], hash)?;
    assert!(!tx.is_signed());
    fireblocks_signer.try_sign_multi_legacy(&mut tx, &[&kp, &fireblocks_signer], hash)?;
    assert!(tx.is_signed());
    let tx: VersionedTransaction = tx.into();
    tracing::info!("slot[0]={}", tx.signatures[0]);
    tracing::info!("slot[1]={}", tx.signatures[1]);
    let transaction_base64 = BASE64_STANDARD.encode(bincode::serialize(&tx)?);
    tracing::info!("broadcasting transaction non fireblocks fee payer {transaction_base64}");
    // bug on fireblocks remote signer
    // let sig = rpc.send_and_confirm_transaction(&tx)?;
    // tracing::info!("sig {sig}");
    Ok(())
}

#[test]
#[instrumented_test]
fn test_multi_sig_versioned() -> anyhow::Result<()> {
    let (fireblocks_signer, rpc) = signer()?;
    let kp_secret = std::env::var("TEST_PRIVATE_KEY")?;
    let kp = solana_keypair::Keypair::from_base58_string(&kp_secret);
    let hash = rpc.get_latest_blockhash()?;
    let ins = build_memo(&MEMO_ID, "multi".as_bytes(), &[
        &fireblocks_signer.pubkey(),
        &kp.pubkey(),
    ]);
    let mut tx =
        VersionedTransaction::new_unsigned_v0(&fireblocks_signer.pubkey(), &[ins], &[], hash)?;
    kp.try_sign_multi_versioned(&mut tx, &[&fireblocks_signer], Some(hash))?;
    assert_ne!(tx.signatures[1], Signature::default());
    assert_eq!(tx.signatures[0], Signature::default());
    fireblocks_signer.try_sign_multi_versioned(&mut tx, &[&kp], Some(hash))?;
    broadcast(&rpc, &tx)?;
    Ok(())
}

#[test]
#[instrumented_test]
fn test_signer_legacy() -> anyhow::Result<()> {
    let (signer, rpc) = signer()?;
    let hash = rpc.get_latest_blockhash()?;
    let message = Message::new(&[memo("fireblocks signer", &signer.pk)], Some(&signer.pk));
    let mut tx = Transaction::new_unsigned(message);
    assert!(signer.is_interactive());
    tx.try_sign(&[&signer], hash)?;
    broadcast(&rpc, &tx)?;
    Ok(())
}

#[test]
#[instrumented_test]
fn test_signer_versioned() -> anyhow::Result<()> {
    let (signer, rpc) = signer()?;
    let instructions = vec![
        memo("fireblocks signer versioned", &signer.pubkey()),
        memo("lookup this", &signer.pubkey()),
        solana_system_interface::instruction::transfer(&signer.pubkey(), &TO, 1),
    ];
    let hash = rpc.get_latest_blockhash()?;
    let alt = utils::lookup(&rpc, &[LOOKUP])?;
    let mut tx = VersionedTransaction::new_unsigned_v0(&signer.pk, &instructions, &alt, hash)?;
    tx.try_sign(&[&signer], None)?;
    broadcast(&rpc, &tx)?;
    Ok(())
}

#[test]
#[instrumented_test]
fn test_env() -> anyhow::Result<()> {
    let _ = FireblocksSigner::try_from_env(None)?;
    let _ = FireblocksSigner::try_from_env(Some(|t| println!("{t}")))?;
    Ok(())
}

#[test]
#[instrumented_test]
fn test_keypair() -> anyhow::Result<()> {
    let (_, rpc) = signer()?;
    let signer = FireblocksSigner::new();
    let hash = rpc.get_latest_blockhash()?;
    let message = Message::new(&[memo("fireblocks signer", &signer.pk)], Some(&signer.pk));
    let mut tx = Transaction::new_unsigned(message);
    tx.try_sign(&[&signer], hash)?;
    tracing::info!("sig {}", tx.get_signature());

    let base64 = signer.to_base58_string();
    let from_base64 = FireblocksSigner::from_base58_string(&base64);
    assert_eq!(signer.pk, from_base64.pk);
    let b = signer.to_bytes();
    let from_b = FireblocksSigner::new_from_array(b[..32].try_into().unwrap());
    assert_eq!(signer.pk, from_b.pk);

    Ok(())
}
