use fireblocks_solana_signer::FireblocksSigner;
mod common;
use {
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
