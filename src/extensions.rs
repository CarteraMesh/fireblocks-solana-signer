use {
    solana_message::{AddressLookupTableAccount, CompileError, VersionedMessage, v0},
    solana_pubkey::Pubkey,
    solana_sdk::{hash::Hash, instruction::Instruction},
    solana_signature::Signature,
    solana_signer::{SignerError, signers::Signers},
    solana_transaction::versioned::VersionedTransaction,
};

/// Add extensions which make it possible to partially sign a versioned
/// transaction.
/// Shameless borrowed from  https://github.com/ifiokjr/wasm_solana/blob/main/crates/wasm_client_solana/src/extensions.rs
pub trait VersionedTransactionExtension {
    /// Create a new unsigned transaction from the payer and instructions with a
    /// recent blockhash. Under the hood this creates the message which needs to
    /// be signed.
    fn new_unsigned_v0(
        payer: &Pubkey,
        instructions: &[Instruction],
        address_lookup_tables: &[AddressLookupTableAccount],
        recent_blockhash: Hash,
    ) -> Result<VersionedTransaction, CompileError>;

    fn new_unsigned(message: VersionedMessage) -> VersionedTransaction;

    /// Attempt to sign this transaction with provided signers.
    fn try_sign<T: Signers + ?Sized>(
        &mut self,
        signers: &T,
        recent_blockhash: Option<Hash>,
    ) -> Result<&mut Self, SignerError>;

    /// Sign the transaction with a subset of required keys, returning any
    /// errors.
    ///
    /// This places each of the signatures created from `keypairs` in the
    /// corresponding position, as specified in the `positions` vector, in the
    /// transactions [`signatures`] field. It does not verify that the signature
    /// positions are correct.
    ///
    /// [`signatures`]: VersionedTransaction::signatures
    ///
    /// # Errors
    ///
    /// Returns an error if signing fails.
    fn try_sign_unchecked<T: Signers + ?Sized>(
        &mut self,
        signers: &T,
        positions: Vec<usize>,
        recent_blockhash: Option<Hash>,
    ) -> Result<(), SignerError>;

    fn get_signing_keypair_positions(
        &self,
        pubkeys: &[Pubkey],
    ) -> Result<Vec<Option<usize>>, SignerError>;
}

impl VersionedTransactionExtension for VersionedTransaction {
    fn new_unsigned_v0(
        payer: &Pubkey,
        instructions: &[Instruction],
        address_lookup_tables: &[AddressLookupTableAccount],
        recent_blockhash: Hash,
    ) -> Result<Self, CompileError> {
        let message =
            v0::Message::try_compile(payer, instructions, address_lookup_tables, recent_blockhash)?;
        let versioned_message = VersionedMessage::V0(message);

        Ok(Self::new_unsigned(versioned_message))
    }

    /// Create an unsigned transaction from a [`VersionedMessage`].
    fn new_unsigned(message: VersionedMessage) -> Self {
        let signatures =
            vec![Signature::default(); message.header().num_required_signatures as usize];

        Self {
            signatures,
            message,
        }
    }

    /// Sign the transaction with a subset of required keys, returning any
    /// errors.
    ///
    /// Unlike [`VersionedTransaction::try_new`], this method does not require
    /// all keypairs to be provided, allowing a transaction to be signed in
    /// multiple steps.
    ///
    /// It is permitted to sign a transaction with the same keypair multiple
    /// times.
    ///
    /// If `recent_blockhash` is different than recorded in the transaction
    /// message's [`VersionedMessage::recent_blockhash()`] method, then the
    /// message's `recent_blockhash` will be updated to the provided
    /// `recent_blockhash`, and any prior signatures will be cleared.
    fn try_sign<T: Signers + ?Sized>(
        &mut self,
        keypairs: &T,
        recent_blockhash: Option<Hash>,
    ) -> Result<&mut Self, SignerError> {
        let positions = self
            .get_signing_keypair_positions(&keypairs.pubkeys())?
            .iter()
            .map(|pos| pos.ok_or(SignerError::KeypairPubkeyMismatch))
            .collect::<Result<Vec<_>, _>>()?;
        self.try_sign_unchecked(keypairs, positions, recent_blockhash)?;

        Ok(self)
    }

    /// Get the positions of the pubkeys in
    /// [`VersionedMessage::static_account_keys`] associated with
    /// signing keypairs.
    fn get_signing_keypair_positions(
        &self,
        pubkeys: &[Pubkey],
    ) -> Result<Vec<Option<usize>>, SignerError> {
        let static_account_keys = self.message.static_account_keys();

        if static_account_keys.len() < self.message.header().num_required_signatures as usize {
            return Err(SignerError::InvalidInput("invalid message".to_string()));
        }

        let signed_keys =
            &static_account_keys[0..self.message.header().num_required_signatures as usize];

        Ok(pubkeys
            .iter()
            .map(|pubkey| signed_keys.iter().position(|x| x == pubkey))
            .collect())
    }

    fn try_sign_unchecked<T: Signers + ?Sized>(
        &mut self,
        keypairs: &T,
        positions: Vec<usize>,
        recent_blockhash: Option<Hash>,
    ) -> Result<(), SignerError> {
        let message_blockhash = *self.message.recent_blockhash();
        let recent_blockhash = recent_blockhash.unwrap_or(message_blockhash);

        if recent_blockhash != message_blockhash {
            self.message.set_recent_blockhash(recent_blockhash);

            // reset signatures if blockhash has changed
            self.signatures
                .iter_mut()
                .for_each(|signature| *signature = Signature::default());
        }

        let signatures = keypairs.try_sign_message(&self.message.serialize())?;

        for ii in 0..positions.len() {
            self.signatures[positions[ii]] = signatures[ii];
        }

        Ok(())
    }
}
