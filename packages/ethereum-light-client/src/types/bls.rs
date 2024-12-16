//! BLS12-381 signature types for Ethereum beacon chain.

use alloy_primitives::{FixedBytes, B256};

/// The Domain Separation Tag for `hash_to_point` in Ethereum beacon chain BLS12-381 signatures.
///
/// This is also the name of the ciphersuite that defines beacon chain BLS signatures.
///
/// See:
/// <https://github.com/ethereum/consensus-specs/blob/ffa95b7b72149960c5aded5c95fb40d64bcab199/specs/phase0/beacon-chain.md#bls-signatures>
/// <https://datatracker.ietf.org/doc/html/draft-irtf-cfrg-bls-signature-04>
pub const BLS_DST_SIG: &[u8] = b"BLS_SIG_BLS12381G2_XMD:SHA-256_SSWU_RO_POP_";

/// The number of bytes in a BLS12-381 public key.
pub const BLS_PUBLIC_KEY_BYTES_LEN: usize = 48;

/// The number of bytes in a BLS12-381 secret key.
pub const BLS_SECRET_KEY_BYTES_LEN: usize = 32;

/// The number of bytes in a BLS12-381 signature.
pub const BLS_SIGNATURE_BYTES_LEN: usize = 96;

/// The bytes representing a BLS12-381 public key.
#[allow(clippy::module_name_repetitions)]
pub type BlsPublicKey = FixedBytes<BLS_PUBLIC_KEY_BYTES_LEN>;
/// The bytes representing a BLS12-381 signature.
#[allow(clippy::module_name_repetitions)]
pub type BlsSignature = FixedBytes<BLS_SIGNATURE_BYTES_LEN>;

/// The BLS verifier trait.
#[allow(clippy::module_name_repetitions)]
pub trait BlsVerify {
    /// The error type for the BLS verifier.
    type Error: std::fmt::Display;

    /// Verify a BLS signature.
    /// # Errors
    /// Returns an error if the signature cannot be verified.
    fn fast_aggregate_verify(
        &self,
        public_keys: Vec<&BlsPublicKey>,
        msg: B256,
        signature: BlsSignature,
    ) -> Result<(), Self::Error>;
}