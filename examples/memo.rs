use fireblocks_solana_signer::FireblocksSigner;
mod common;
use {
    solana_message::Message,
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

#[cfg(not(feature = "tokio"))]
fn main() -> anyhow::Result<()> {
    use solana_rpc_client::rpc_client::{RpcClient, SerializableTransaction};

    common::setup();
    let signer: FireblocksSigner = FireblocksSigner::try_from_env(None)?;
    let rpc = RpcClient::new(
        std::env::var("RPC_URL")
            .ok()
            .unwrap_or("https://rpc.ankr.com/solana_devnet".to_string()),
    );
    let hash = rpc.get_latest_blockhash()?;
    let message = Message::new(&[memo("fireblocks signer")], Some(&signer.pk));
    let mut tx = Transaction::new_unsigned(message);
    tx.try_sign(&[&signer], hash)?;
    println!("sig {}", tx.get_signature());
    Ok(())
}

#[cfg(feature = "tokio")]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use solana_rpc_client::{
        nonblocking::rpc_client::RpcClient,
        rpc_client::SerializableTransaction,
    };

    common::setup();
    let signer: FireblocksSigner = FireblocksSigner::try_from_env(None).await?;
    let rpc = RpcClient::new(
        std::env::var("RPC_URL")
            .ok()
            .unwrap_or("https://rpc.ankr.com/solana_devnet".to_string()),
    );
    let hash = rpc.get_latest_blockhash().await?;
    let message = Message::new(&[memo("fireblocks signer")], Some(&signer.pk));
    let mut tx = Transaction::new_unsigned(message);
    tx.try_sign(&[&signer], hash)?;
    println!("sig {}", tx.get_signature());
    Ok(())
}
