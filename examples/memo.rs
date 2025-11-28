use fireblocks_solana_signer::FireblocksSigner;
mod common;
use solana_sdk::transaction::Transaction;

fn main() -> anyhow::Result<()> {
    use solana_client::rpc_client::{RpcClient, SerializableTransaction};

    common::setup();
    let signer: FireblocksSigner = FireblocksSigner::try_from_env(None)?;
    let rpc = RpcClient::new(
        std::env::var("RPC_URL")
            .ok()
            .unwrap_or("https://rpc.ankr.com/solana_devnet".to_string()),
    );
    let hash = rpc.get_latest_blockhash()?;
    let message = common::memo(&hash, &signer, "fireblocks signer");
    let mut tx = Transaction::new_unsigned(message);
    tx.try_sign(&[&signer], hash)?;
    println!("sig {}", tx.get_signature());
    Ok(())
}
