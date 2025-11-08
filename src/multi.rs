use {
    crate::{DynSigner, FireblocksSigner, VersionedTransactionExtension},
    solana_hash::Hash,
    solana_signer::{Signer, SignerError},
    solana_transaction::{Transaction, versioned::VersionedTransaction},
    tracing::info,
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
        all_signers: &[&DynSigner],
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
        all_signers: &[&DynSigner],
        hash: Option<Hash>,
    ) -> Result<(), SignerError>;
}

/// Macro to implement default MultiSigner behavior for standard signer types.
///
/// This macro generates implementations for signers that don't need special
/// multi-sig handling (i.e., they just sign their own part of the transaction).
macro_rules! impl_default_multi_signer {
    ($type:ty) => {
        impl MultiSigner for $type {
            fn try_sign_multi_legacy(
                &self,
                tx: &mut Transaction,
                _all_signers: &[&DynSigner],
                hash: Hash,
            ) -> Result<(), SignerError> {
                tx.try_partial_sign(&[self], hash)
            }

            fn try_sign_multi_versioned(
                &self,
                tx: &mut VersionedTransaction,
                _all_signers: &[&DynSigner],
                hash: Option<Hash>,
            ) -> Result<(), SignerError> {
                tx.try_sign(&[self], hash)?;
                Ok(())
            }
        }
    };
}

// Implement default multi-sig behavior for standard Solana signer types
impl_default_multi_signer!(solana_keypair::Keypair);
impl_default_multi_signer!(solana_presigner::Presigner);
impl_default_multi_signer!(solana_remote_wallet::remote_keypair::RemoteKeypair);
impl_default_multi_signer!(solana_signer::null_signer::NullSigner);

impl MultiSigner for FireblocksSigner {
    fn try_sign_multi_legacy(
        &self,
        tx: &mut Transaction,
        all_signers: &[&DynSigner],
        hash: Hash,
    ) -> Result<(), SignerError> {
        info!(
            "multi signing: {} other signer(s) plus FireblocksSigner",
            all_signers.len() - 1
        );
        // Sign with all other signers first
        for signer in all_signers {
            if signer.pubkey() != self.pubkey() {
                signer.try_sign_multi_legacy(tx, &[], hash)?;
            }
        }

        let vtx: VersionedTransaction = tx.clone().into();
        let sig = self
            .sign_versioned_transaction(&vtx)
            .map_err(|e| SignerError::Custom(e.to_string()))?;

        // Find position and insert Fireblocks signature
        let positions = tx.get_signing_keypair_positions(&[self.pubkey()])?;
        match positions.first() {
            Some(Some(pos)) => {
                tracing::debug!("using slot {} for fireblocks sig {sig}", *pos);
                tx.signatures[*pos] = sig;
            }
            Some(None) => {
                return Err(SignerError::Custom(
                    "Fireblocks pubkey not found in transaction's required signers".to_string(),
                ));
            }
            None => {
                return Err(SignerError::Custom(
                    "Failed to get signing positions from transaction".to_string(),
                ));
            }
        }
        Ok(())
    }

    fn try_sign_multi_versioned(
        &self,
        tx: &mut VersionedTransaction,
        all_signers: &[&DynSigner],
        hash: Option<Hash>,
    ) -> Result<(), SignerError> {
        // Sign with all other signers first
        for signer in all_signers {
            if signer.pubkey() != self.pubkey() {
                signer.try_sign_multi_versioned(tx, &[], hash)?;
            }
        }

        // Sign with Fireblocks using the partially-signed transaction
        let sig = self
            .sign_versioned_transaction(tx)
            .map_err(|e| SignerError::Custom(e.to_string()))?;

        // Find position and insert signature
        let positions = tx.get_signing_keypair_positions(&[self.pubkey()])?;
        match positions.first() {
            Some(Some(pos)) => {
                tx.signatures[*pos] = sig;
            }
            Some(None) => {
                return Err(SignerError::Custom(
                    "Fireblocks pubkey not found in transaction's required signers".to_string(),
                ));
            }
            None => {
                return Err(SignerError::Custom(
                    "Failed to get signing positions from transaction".to_string(),
                ));
            }
        }
        Ok(())
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
