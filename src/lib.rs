//! # Zcash Wallet Interchange Format (ZeWIF)
//!
//! A standard data format for migrating wallet data between Zcash wallet
//! implementations. Supports transparent, Sprout, Sapling, and Orchard protocols.
//!
//! ## Type Hierarchy
//!
//! - [`Zewif`]: Root container (wallets + global transaction history)
//!   - [`ZewifWallet`]: Wallet (network, seed material, accounts)
//!     - [`Account`]: Viewing key, addresses, transaction references
//!       - [`Address`]: Protocol-specific address ([`ProtocolAddress`])
//!   - [`Transaction`]: Transaction metadata and optional raw/compact data

// Macros
mod blob_macro;
mod data_macro;
mod envelope_macros;
mod mod_use_macro;
mod string_macro;
mod test_roundtrip_macros;

// Test utilities
#[cfg(any(test, feature = "test-dependencies"))]
mod_use!(test_utils);

// Modules requiring qualified paths
pub mod orchard;
pub mod sapling;
pub mod sprout;
pub mod transparent;

// Modules that can use unqualified paths
mod_use!(account);
mod_use!(account_viewing_key);
mod_use!(address);
mod_use!(amount);
mod_use!(anchor);
mod_use!(bip_39_mnemonic);
mod_use!(blob);
mod_use!(block_hash);
mod_use!(block_height);
mod_use!(data);
mod_use!(error);
mod_use!(derivation_info);
mod_use!(incremental_witness);
mod_use!(indexed);
mod_use!(key_scope);
mod_use!(key_source);
mod_use!(memo);
mod_use!(mnemonic_language);
mod_use!(network);
mod_use!(non_hardened_child_index);
mod_use!(protocol_address);
mod_use!(received_output);
mod_use!(scan_range);
mod_use!(sent_output);
mod_use!(script);
mod_use!(legacy_seed);
mod_use!(seed_material);
mod_use!(seed_fingerprint);
mod_use!(string_utils);
mod_use!(transaction);
mod_use!(transaction_data);
mod_use!(tx_block_position);
mod_use!(txid);
mod_use!(unified_address);
mod_use!(unified_full_viewing_key);
mod_use!(zewif_envelope);
mod_use!(zewif_impl);
mod_use!(zewif_wallet);

use std::fmt::{self, Debug, Display, Formatter};

pub use blob::Blob;

#[doc(hidden)]
pub struct NoQuotesDebugOption<'a, T>(pub &'a Option<T>);

impl<T: Display> Debug for NoQuotesDebugOption<'_, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.0 {
            Some(val) => write!(f, "Some({})", val),
            None => write!(f, "None"),
        }
    }
}
