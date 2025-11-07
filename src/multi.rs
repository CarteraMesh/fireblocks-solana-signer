use {
    crate::{FireblocksSigner, VersionedTransactionExtension},
    solana_hash::Hash,
    solana_signer::{Signer, SignerError},
    solana_transaction::{Transaction, versioned::VersionedTransaction},
};

/// Trait for multi-signature signing that works with Fireblocks.
///
/// This trait extends the standard `Signer` trait to support multi-signature
/// scenarios where Fireblocks needs access to partially-signed transactions.
pub trait MultiSigner: Signer {
    /// Sign a legacy transaction in a multi-sig context.
    ///
    /// # Arguments
    ///
    /// * `tx` - The transaction to sign
    /// * `all_signers` - All signers participating in this multi-sig (including
    ///   self)
    /// * `hash` - Recent blockhash for the transaction
    fn try_sign_multi_legacy(
        &self,
        tx: &mut Transaction,
        all_signers: &[&dyn MultiSigner],
        hash: Hash,
    ) -> Result<(), SignerError>;

    /// Sign a versioned transaction in a multi-sig context.
    ///
    /// # Arguments
    ///
    /// * `tx` - The versioned transaction to sign
    /// * `all_signers` - All signers participating in this multi-sig (including
    ///   self)
    /// * `hash` - Optional recent blockhash (None if already set in message)
    fn try_sign_multi_versioned(
        &self,
        tx: &mut VersionedTransaction,
        all_signers: &[&dyn MultiSigner],
        hash: Option<Hash>,
    ) -> Result<(), SignerError>;
}

impl MultiSigner for solana_keypair::Keypair {
    fn try_sign_multi_legacy(
        &self,
        tx: &mut Transaction,
        _all_signers: &[&dyn MultiSigner],
        hash: Hash,
    ) -> Result<(), SignerError> {
        tx.try_partial_sign(&[self], hash)
    }

    fn try_sign_multi_versioned(
        &self,
        _tx: &mut VersionedTransaction,
        _all_signers: &[&dyn MultiSigner],
        _hash: Option<Hash>,
    ) -> Result<(), SignerError> {
        todo!("Implement versioned transaction multi-sig for Keypair")
    }
}

impl MultiSigner for FireblocksSigner {
    fn try_sign_multi_legacy(
        &self,
        tx: &mut Transaction,
        all_signers: &[&dyn MultiSigner],
        hash: Hash,
    ) -> Result<(), SignerError> {
        // Sign with all other signers first
        for signer in all_signers {
            if signer.pubkey() != self.pubkey() {
                signer.try_sign_multi_legacy(tx, &[], hash)?;
            }
        }

        // Convert to VersionedTransaction for Fireblocks
        let vtx: VersionedTransaction = tx.clone().into();

        // Sign with Fireblocks using the partially-signed transaction
        let sig = self
            .sign_versioned_transaction(&vtx)
            .map_err(|e| SignerError::Custom(e.to_string()))?;

        // Find position and insert signature
        let positions = vtx.get_signing_keypair_positions(&[self.pubkey()])?;
        if let Some(Some(pos)) = positions.first() {
            tx.signatures[*pos] = sig;
        } else {
            return Err(SignerError::KeypairPubkeyMismatch);
        }

        Ok(())
    }

    fn try_sign_multi_versioned(
        &self,
        _tx: &mut VersionedTransaction,
        _all_signers: &[&dyn MultiSigner],
        _hash: Option<Hash>,
    ) -> Result<(), SignerError> {
        todo!("Implement versioned transaction multi-sig for FireblocksSigner")
    }
}

impl MultiSigner for solana_presigner::Presigner {
    fn try_sign_multi_legacy(
        &self,
        tx: &mut Transaction,
        _all_signers: &[&dyn MultiSigner],
        hash: Hash,
    ) -> Result<(), SignerError> {
        tx.try_partial_sign(&[self], hash)
    }

    fn try_sign_multi_versioned(
        &self,
        _tx: &mut VersionedTransaction,
        _all_signers: &[&dyn MultiSigner],
        _hash: Option<Hash>,
    ) -> Result<(), SignerError> {
        todo!("Implement versioned transaction multi-sig for FireblocksSigner")
    }
}

impl MultiSigner for solana_remote_wallet::remote_keypair::RemoteKeypair {
    fn try_sign_multi_legacy(
        &self,
        tx: &mut Transaction,
        _all_signers: &[&dyn MultiSigner],
        hash: Hash,
    ) -> Result<(), SignerError> {
        tx.try_partial_sign(&[self], hash)
    }

    fn try_sign_multi_versioned(
        &self,
        _tx: &mut VersionedTransaction,
        _all_signers: &[&dyn MultiSigner],
        _hash: Option<Hash>,
    ) -> Result<(), SignerError> {
        todo!("Implement versioned transaction multi-sig for FireblocksSigner")
    }
}

impl MultiSigner for solana_signer::null_signer::NullSigner {
    fn try_sign_multi_legacy(
        &self,
        tx: &mut Transaction,
        _all_signers: &[&dyn MultiSigner],
        hash: Hash,
    ) -> Result<(), SignerError> {
        tx.try_partial_sign(&[self], hash)
    }

    fn try_sign_multi_versioned(
        &self,
        _tx: &mut VersionedTransaction,
        _all_signers: &[&dyn MultiSigner],
        _hash: Option<Hash>,
    ) -> Result<(), SignerError> {
        todo!("Implement versioned transaction multi-sig for FireblocksSigner")
    }
}

impl PartialEq for dyn MultiSigner {
    fn eq(&self, other: &dyn MultiSigner) -> bool {
        self.pubkey() == other.pubkey()
    }
}

impl Eq for dyn MultiSigner {}

impl std::fmt::Debug for dyn MultiSigner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MultiSigner({})", self.pubkey())
    }
}
