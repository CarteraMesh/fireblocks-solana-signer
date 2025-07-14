//! Extensions for Solana versioned transactions.
//!
//! This module provides additional functionality for working with Solana's
//! versioned transactions, particularly for partial signing scenarios and
//! address lookup table support. These extensions are especially useful when
//! working with multi-signature transactions or when you need to sign
//! transactions in multiple steps.
//!
//! The main trait [`VersionedTransactionExtension`] extends
//! [`VersionedTransaction`] with methods for creating unsigned transactions and
//! performing partial signing operations.
//!
//! Credit: Shameless borrowed from <https://github.com/ifiokjr/wasm_solana/blob/main/crates/wasm_client_solana/src/extensions.rs>

use {
    solana_hash::Hash,
    solana_instruction::Instruction,
    solana_message::{AddressLookupTableAccount, CompileError, VersionedMessage, v0},
    solana_pubkey::Pubkey,
    solana_signature::Signature,
    solana_signer::{SignerError, signers::Signers},
    solana_transaction::versioned::VersionedTransaction,
};

/// Extension trait for [`VersionedTransaction`] that adds support for partial
/// signing and address lookup table operations.
///
/// This trait provides methods to create unsigned versioned transactions and
/// sign them in multiple steps, which is particularly useful for
/// multi-signature scenarios or when working with hardware wallets and custody
/// solutions like Fireblocks.
pub trait VersionedTransactionExtension {
    /// Creates a new unsigned versioned transaction using the v0 message
    /// format.
    ///
    /// This method compiles instructions into a v0 message format that supports
    /// address lookup tables, allowing for more compact transactions when
    /// dealing with frequently used addresses.
    ///
    /// # Arguments
    ///
    /// * `payer` - The public key of the account that will pay for the
    ///   transaction
    /// * `instructions` - The instructions to include in the transaction
    /// * `address_lookup_tables` - Address lookup tables to use for address
    ///   compression
    /// * `recent_blockhash` - A recent blockhash for the transaction
    ///
    /// # Returns
    ///
    /// Returns a [`Result`] containing the unsigned [`VersionedTransaction`] on
    /// success, or a [`CompileError`] if the message compilation fails.
    ///
    /// # Errors
    ///
    /// This method can fail if:
    /// - The instructions cannot be compiled into a valid message
    /// - The address lookup tables are invalid
    /// - The payer account is invalid
    fn new_unsigned_v0(
        payer: &Pubkey,
        instructions: &[Instruction],
        address_lookup_tables: &[AddressLookupTableAccount],
        recent_blockhash: Hash,
    ) -> Result<VersionedTransaction, CompileError>;

    /// Creates a new unsigned versioned transaction from a
    /// [`VersionedMessage`].
    ///
    /// This method creates a transaction with default (empty) signatures that
    /// can be filled in later through signing operations.
    ///
    /// # Arguments
    ///
    /// * `message` - The versioned message to wrap in a transaction
    ///
    /// # Returns
    ///
    /// Returns a new [`VersionedTransaction`] with empty signatures.
    fn new_unsigned(message: VersionedMessage) -> VersionedTransaction;

    /// Attempts to sign the transaction with the provided signers.
    ///
    /// This method automatically determines the correct signature positions for
    /// each signer and updates the transaction's signatures accordingly. If
    /// a recent blockhash is provided and differs from the message's
    /// current blockhash, the message will be updated and all existing
    /// signatures cleared.
    ///
    /// # Arguments
    ///
    /// * `signers` - The signers to use for signing the transaction
    /// * `recent_blockhash` - Optional recent blockhash to update the message
    ///   with
    ///
    /// # Returns
    ///
    /// Returns a mutable reference to the transaction on success, or a
    /// [`SignerError`] if signing fails.
    ///
    /// # Errors
    ///
    /// This method can fail if:
    /// - The signers' public keys don't match required signers in the message
    /// - The signing operation fails
    /// - The message format is invalid
    fn try_sign<T: Signers + ?Sized>(
        &mut self,
        signers: &T,
        recent_blockhash: Option<Hash>,
    ) -> Result<&mut Self, SignerError>;

    /// Signs the transaction with a subset of required keys at specific
    /// positions.
    ///
    /// This method provides fine-grained control over signature placement,
    /// allowing you to specify exactly which signature positions should be
    /// filled by which signers. This is useful for complex multi-signature
    /// scenarios.
    ///
    /// # Arguments
    ///
    /// * `signers` - The signers to use for signing
    /// * `positions` - The signature positions to fill (indices into the
    ///   signatures array)
    /// * `recent_blockhash` - Optional recent blockhash to update the message
    ///   with
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success, or a [`SignerError`] if signing fails.
    ///
    /// # Errors
    ///
    /// This method can fail if:
    /// - The positions vector contains invalid indices
    /// - The signing operation fails
    /// - The number of signers doesn't match the number of positions
    ///
    /// # Safety
    ///
    /// This method does not verify that the signature positions are correct for
    /// the provided signers. It's the caller's responsibility to ensure the
    /// positions match the expected signers.
    fn try_sign_unchecked<T: Signers + ?Sized>(
        &mut self,
        signers: &T,
        positions: Vec<usize>,
        recent_blockhash: Option<Hash>,
    ) -> Result<(), SignerError>;

    /// Gets the signature positions for a set of public keys.
    ///
    /// This method determines where each public key should place its signature
    /// in the transaction's signature array based on the message's required
    /// signers.
    ///
    /// # Arguments
    ///
    /// * `pubkeys` - The public keys to find positions for
    ///
    /// # Returns
    ///
    /// Returns a vector where each element is either `Some(position)` if the
    /// corresponding public key is a required signer, or `None` if it's not.
    ///
    /// # Errors
    ///
    /// This method can fail if the message format is invalid or corrupted.
    fn get_signing_keypair_positions(
        &self,
        pubkeys: &[Pubkey],
    ) -> Result<Vec<Option<usize>>, SignerError>;
}

/// Implementation of [`VersionedTransactionExtension`] for
/// [`VersionedTransaction`].
///
/// This implementation provides all the extension methods for working with
/// versioned transactions, including creation of unsigned transactions and
/// partial signing support.
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

    fn new_unsigned(message: VersionedMessage) -> Self {
        let signatures =
            vec![Signature::default(); message.header().num_required_signatures as usize];

        Self {
            signatures,
            message,
        }
    }

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

            // Reset signatures if blockhash has changed
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
