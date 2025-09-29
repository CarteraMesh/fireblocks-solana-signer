mod common;
use {
    fireblocks_solana_signer::FireblocksSigner,
    solana_message::Message,
    solana_native_token::sol_str_to_lamports,
    solana_signer::Signer,
    solana_stake_interface::{
        self,
        instruction::{self as stake_instruction},
        state::Authorized,
    },
    solana_transaction::Transaction,
};

fn main() -> anyhow::Result<()> {
    use solana_rpc_client::rpc_client::RpcClient;

    common::setup();
    let stake_signer = FireblocksSigner::new();
    let stake_account = stake_signer.pubkey();
    let mut signer: FireblocksSigner = FireblocksSigner::try_from_env(None)?;
    signer.additional_signers(vec![Box::new(stake_signer)]);
    let rpc = RpcClient::new(
        std::env::var("RPC_URL")
            .ok()
            .unwrap_or("https://rpc.ankr.com/solana_devnet".to_string()),
    );
    let authorized = Authorized::auto(&signer.pubkey());
    let inxs = stake_instruction::create_account_checked(
        &signer.pubkey(),
        &stake_account,
        &authorized,
        sol_str_to_lamports("2.0").ok_or(anyhow::format_err!("oh no"))?,
    );
    let block = rpc.get_latest_blockhash()?;
    let msg = Message::new_with_blockhash(&inxs, Some(&signer.pk), &block);
    let mut tx = Transaction::new_unsigned(msg);
    tx.try_sign(&[&signer], block)?;
    for s in &tx.signatures {
        eprintln!("{s}");
    }
    let sig = rpc.send_and_confirm_transaction(&tx)?;
    println!("{sig}");
    Ok(())
}
