use minicbor::{Decode, Encode};

/// The role of an address within its account (maps to zcash_client_sqlite
/// addresses.key_scope encodings 0/1/2/-1).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Encode, Decode)]
#[cbor(index_only)]
#[non_exhaustive]
pub enum KeyScope {
    /// User-facing receiving addresses.
    #[n(0)]
    External,
    /// Wallet-internal change/shielding addresses; never exposed to users.
    #[n(1)]
    Internal,
    /// ZIP 320 single-use ephemeral transparent addresses.
    #[n(2)]
    Ephemeral,
    /// Imported standalone keys/scripts not derived from account key material.
    #[n(3)]
    Foreign,
}

#[cfg(test)]
mod tests {
    use crate::test_cbor_roundtrip;

    use super::KeyScope;

    impl crate::RandomInstance for KeyScope {
        fn random() -> Self {
            match rand::random::<u8>() % 4 {
                0 => KeyScope::External,
                1 => KeyScope::Internal,
                2 => KeyScope::Ephemeral,
                _ => KeyScope::Foreign,
            }
        }
    }

    test_cbor_roundtrip!(KeyScope);
}
