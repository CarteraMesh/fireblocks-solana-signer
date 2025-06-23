use {
    crate::{Error, Result},
    solana_account_decoder::parse_address_lookup_table::{
        LookupTableAccountType,
        parse_address_lookup_table,
    },
    solana_message::AddressLookupTableAccount,
    solana_pubkey::Pubkey,
    solana_rpc_client::rpc_client::RpcClient,
    std::str::FromStr,
};

fn get_address_lookup_table(rpc: &RpcClient, pubkey: &Pubkey) -> Result<LookupTableAccountType> {
    let account = rpc
        .get_account(pubkey)
        .map_err(|e| Error::SolanaRpcErrpr(format!("{e}")))?;
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
