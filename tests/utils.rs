use {
    fireblocks_solana_signer::*,
    solana_account_decoder::parse_address_lookup_table::{
        LookupTableAccountType,
        parse_address_lookup_table,
    },
    solana_hash::Hash,
    solana_instruction::Instruction,
    solana_message::AddressLookupTableAccount,
    solana_rpc_client::rpc_client::RpcClient,
    std::{
        env,
        str::FromStr,
        sync::{Arc, Once},
    },
    tracing_subscriber::{EnvFilter, fmt::format::FmtSpan},
};
pub static INIT: Once = Once::new();
pub fn memo(msg: &str, signers: &Pubkey) -> Instruction {
    spl_memo_interface::instruction::build_memo(&spl_memo_interface::v3::ID, msg.as_bytes(), &[
        signers,
    ])
}

#[allow(clippy::unwrap_used, clippy::missing_panics_doc)]
pub fn setup() {
    INIT.call_once(|| {
        if env::var("CI").is_err() {
            // only load .env if not in CI
            if dotenvy::dotenv_override().is_err() {
                eprintln!("no .env file");
            }
        }
        tracing_subscriber::fmt()
            .with_target(false)
            .with_level(true)
            .with_span_events(FmtSpan::CLOSE)
            .with_env_filter(EnvFilter::from_default_env())
            .init();
    });
}

pub fn signer() -> anyhow::Result<(FireblocksSigner, Arc<RpcClient>)> {
    let signer = FireblocksSigner::try_from_env(None)?;
    let rpc = Arc::new(RpcClient::new(
        std::env::var("RPC_URL")
            .ok()
            .unwrap_or("https://rpc.ankr.com/solana_devnet".to_string()),
    ));
    tracing::info!("using pubkey {}", signer.pk);
    Ok((signer, rpc))
}

fn get_address_lookup_table(rpc: &RpcClient, pubkey: &Pubkey) -> Result<LookupTableAccountType> {
    let account = rpc
        .get_account(pubkey)
        .map_err(|e| Error::SolanaRpcError(format!("{e}")))?;
    // AddressLookupTableAccount::deserialize(&account.data)
    let table_type = parse_address_lookup_table(&account.data)
        .map_err(|error| crate::Error::ParseAddressTableError(error.to_string()))?;

    Ok(table_type)
}

#[allow(dead_code)]
pub(crate) fn lookup(
    rpc: &RpcClient,
    addresses: &[Pubkey],
) -> Result<Vec<AddressLookupTableAccount>> {
    let mut lookups: Vec<AddressLookupTableAccount> = Vec::with_capacity(3);
    for a in addresses {
        let addr_table = get_address_lookup_table(rpc, a)?;
        match addr_table {
            LookupTableAccountType::Uninitialized => tracing::debug!("no lookups for {a}"),
            LookupTableAccountType::LookupTable(t) => {
                let mut pk_address = Vec::with_capacity(t.addresses.len());
                for s in &t.addresses {
                    pk_address.push(Pubkey::from_str(s).map_err(|_| Error::InvalidPubkey)?);
                }
                lookups.push(AddressLookupTableAccount {
                    addresses: pk_address,
                    key: *a,
                });
            }
        }
    }
    Ok(lookups)
}
