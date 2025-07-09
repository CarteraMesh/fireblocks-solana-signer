//! Keypair compatibility layer for FireblocksSigner
//!
//! This module provides a compatibility layer that allows `FireblocksSigner` to
//! be used as a drop-in replacement for Solana's `Keypair` struct. This enables
//! seamless switching between local signing and Fireblocks-based signing using
//! feature flags.
//!
//! # Feature Flag Usage
//!
//! The intended usage pattern is to conditionally import either the Fireblocks
//! signer or the standard Solana keypair based on feature flags:
//!
//! ```rust,ignore
//! #[cfg(feature = "fireblocks")]
//! use fireblocks_solana_signer::FireblocksSigner as Keypair;
//! #[cfg(not(feature = "fireblocks"))]
//! use solana_sdk::signature::Keypair;
//! ```
//!
//! This allows the same codebase to work with both signing methods without
//! modification.
//!
//! # Compatibility
//!
//! The methods in this module mirror the API of
//! `solana_sdk::signature::Keypair`, providing the same function signatures and
//! behavior. This ensures that existing code using Solana's Keypair can work
//! unchanged when switched to FireblocksSigner.
//!
//! # Important Note
//!
//! When using FireblocksSigner with Fireblocks backend (not just the keypair
//! compatibility layer), remember that **transactions are automatically
//! broadcasted** to the Solana network upon signing. See the main crate
//! documentation for more details on this behavior.

use {
    super::FireblocksSigner,
    solana_keypair::Keypair,
    solana_signature::error::Error as SignatureError,
    solana_signer::Signer,
};

/// The length of a keypair in bytes (32 bytes secret key + 32 bytes public key)
const KEYPAIR_LENGTH: usize = 64;

/// Constructs a `FireblocksSigner` from caller-provided seed entropy.
///
/// This function provides compatibility with Solana's `keypair_from_seed`
/// function, allowing for drop-in replacement when using feature flags.
///
/// # Arguments
///
/// * `seed` - A byte slice containing the seed entropy for keypair generation
///
/// # Returns
///
/// A `Result` containing either a `FireblocksSigner` or an error if keypair
/// generation fails.
///
/// # Example
///
/// ```rust,no_run
/// use fireblocks_solana_signer::keypair_from_seed;
///
/// let seed = b"my_seed_bytes_here_exactly_32_bytes!";
/// let signer = keypair_from_seed(seed).expect("Failed to create signer from seed");
/// ```
pub fn keypair_from_seed(seed: &[u8]) -> Result<FireblocksSigner, Box<dyn std::error::Error>> {
    let kp = solana_keypair::keypair_from_seed(seed)?;
    Ok(FireblocksSigner::new_with_keypair(kp))
}

impl FireblocksSigner {
    /// Constructs a new `FireblocksSigner` with a randomly generated keypair.
    ///
    /// This method provides compatibility with Solana's `Keypair::new()`
    /// function, enabling seamless replacement when using feature flags for
    /// Fireblocks integration.
    ///
    /// # Returns
    ///
    /// A new `FireblocksSigner` instance with a randomly generated keypair.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use {fireblocks_solana_signer::FireblocksSigner, solana_signer::Signer};
    ///
    /// let signer = FireblocksSigner::new();
    /// println!("Public key: {}", signer.pubkey());
    /// ```
    pub fn new() -> Self {
        Self::new_with_keypair(Keypair::new())
    }

    /// Constructs a new `FireblocksSigner` using an existing Solana `Keypair`.
    ///
    /// This method allows wrapping an existing Solana keypair within the
    /// FireblocksSigner structure, maintaining compatibility while adding
    /// Fireblocks functionality.
    ///
    /// # Arguments
    ///
    /// * `keypair` - A Solana `Keypair` to wrap within the FireblocksSigner
    ///
    /// # Returns
    ///
    /// A new `FireblocksSigner` instance wrapping the provided keypair.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use {fireblocks_solana_signer::FireblocksSigner, solana_sdk::signature::Keypair};
    ///
    /// let keypair = Keypair::new();
    /// let signer = FireblocksSigner::new_with_keypair(keypair);
    /// ```
    pub fn new_with_keypair(keypair: Keypair) -> Self {
        Self {
            pk: keypair.pubkey(),
            keypair: Some(std::sync::Arc::new(keypair)),
            ..Default::default()
        }
    }

    /// Constructs a `FireblocksSigner` from a byte array.
    ///
    /// This method provides compatibility with Solana's `Keypair::from_bytes()`
    /// function, allowing for drop-in replacement when using feature flags.
    ///
    /// # Arguments
    ///
    /// * `bytes` - A byte slice containing the keypair data (64 bytes: 32
    ///   secret + 32 public)
    ///
    /// # Returns
    ///
    /// A `Result` containing either a `FireblocksSigner` or a signature error
    /// if the bytes are invalid.
    ///
    /// # Errors
    ///
    /// Returns `ed25519_dalek::SignatureError` if the provided bytes cannot be
    /// parsed as a valid keypair.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use fireblocks_solana_signer::FireblocksSigner;
    ///
    /// let bytes = [0u8; 64]; // Example bytes (not a valid keypair)
    /// match FireblocksSigner::from_bytes(&bytes) {
    ///     Ok(signer) => println!("Signer created successfully"),
    ///     Err(e) => println!("Failed to create signer: {}", e),
    /// }
    /// ```
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, ed25519_dalek::SignatureError> {
        #[allow(deprecated)]
        let kp = Keypair::from_bytes(bytes)?;
        Ok(Self::new_with_keypair(kp))
    }

    // /// Constructs a `FireblocksSigner` from a 32-byte secret key array.
    // ///
    // /// This method provides compatibility with Solana's
    // `Keypair::from_secret_key_bytes()` function. /// Currently commented out
    // but can be enabled if needed for full Keypair compatibility. ///
    // /// # Arguments
    // ///
    // /// * `secret_key` - A 32-byte array containing the secret key
    // ///
    // /// # Returns
    // ///
    // /// A new `FireblocksSigner` instance created from the secret key.
    // pub fn new_from_array(secret_key: [u8; 32]) -> Self {
    //     Self::new_with_keypair(Keypair::new_from_array(secret_key))
    // }

    /// Returns the keypair as a byte array.
    ///
    /// This method provides compatibility with Solana's `Keypair::to_bytes()`
    /// function, enabling seamless replacement when using feature flags.
    ///
    /// # Returns
    ///
    /// A 64-byte array containing the keypair data (32 bytes secret key + 32
    /// bytes public key). If no keypair is available, returns bytes from a
    /// newly generated keypair.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use fireblocks_solana_signer::FireblocksSigner;
    ///
    /// let signer = FireblocksSigner::new();
    /// let bytes = signer.to_bytes();
    /// assert_eq!(bytes.len(), 64);
    /// ```
    pub fn to_bytes(&self) -> [u8; KEYPAIR_LENGTH] {
        match &self.keypair {
            Some(kp) => kp.to_bytes(),
            None => Keypair::new().to_bytes(),
        }
    }

    /// Constructs a `FireblocksSigner` from a base58-encoded string.
    ///
    /// This method provides compatibility with Solana's
    /// `Keypair::from_base58_string()` function, allowing for drop-in
    /// replacement when using feature flags.
    ///
    /// # Arguments
    ///
    /// * `s` - A base58-encoded string representing the keypair
    ///
    /// # Returns
    ///
    /// A new `FireblocksSigner` instance created from the base58 string.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use fireblocks_solana_signer::FireblocksSigner;
    ///
    /// let base58_string = "your_base58_encoded_keypair_here";
    /// let signer = FireblocksSigner::from_base58_string(base58_string);
    /// ```
    pub fn from_base58_string(s: &str) -> Self {
        let kp = Keypair::from_base58_string(s);
        Self::new_with_keypair(kp)
    }

    /// Returns the keypair as a base58-encoded string.
    ///
    /// This method provides compatibility with Solana's
    /// `Keypair::to_base58_string()` function, enabling seamless
    /// replacement when using feature flags.
    ///
    /// # Returns
    ///
    /// A base58-encoded string representation of the keypair.
    /// If no keypair is available, returns a string from a newly generated
    /// keypair.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use fireblocks_solana_signer::FireblocksSigner;
    ///
    /// let signer = FireblocksSigner::new();
    /// let base58_string = signer.to_base58_string();
    /// println!("Keypair as base58: {}", base58_string);
    /// ```
    pub fn to_base58_string(&self) -> String {
        match &self.keypair {
            Some(kp) => kp.to_base58_string(),
            None => Keypair::new().to_base58_string(),
        }
    }
}

impl TryFrom<&[u8]> for FireblocksSigner {
    type Error = SignatureError;

    fn try_from(bytes: &[u8]) -> std::result::Result<Self, Self::Error> {
        let kp = Keypair::try_from(bytes)?;
        Ok(FireblocksSigner::new_with_keypair(kp))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keypair_from_seed_deterministic() {
        // Test that the same seed produces the same keypair
        let seed = b"test_seed_exactly_32_bytes_long!!";

        let signer1 = keypair_from_seed(seed).expect("Failed to create signer from seed");
        let signer2 = keypair_from_seed(seed).expect("Failed to create signer from seed");

        // Both signers should have the same public key
        assert_eq!(signer1.pubkey(), signer2.pubkey());
    }
}
