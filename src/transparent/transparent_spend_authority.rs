use crate::DerivationInfo;
use minicbor::{Decode, Encode};

/// How a transparent address's key was obtained.
///
/// For HD-derived addresses, the derivation info is sufficient to recover
/// the spending key from the seed. For independently-generated keys (e.g.
/// legacy zcashd random-key addresses), the private key, if exported, is
/// stored in the secret store under the address's public key.
#[derive(Debug, Clone, PartialEq, Encode, Decode)]
#[cbor(flat)]
pub enum TransparentSpendAuthority {
    /// Key derived from an HD seed; derivation info is sufficient.
    #[n(0)]
    Derived(#[n(0)] DerivationInfo),
    /// Independently generated key; the private key, if exported, lives in
    /// the secret store under this address's public key.
    #[n(1)]
    Imported,
}

#[cfg(test)]
impl crate::RandomInstance for TransparentSpendAuthority {
    fn random() -> Self {
        use rand::Rng;
        let mut rng = rand::rng();
        if rng.random_bool(0.5) {
            TransparentSpendAuthority::Derived(DerivationInfo::random())
        } else {
            TransparentSpendAuthority::Imported
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::test_cbor_roundtrip;

    use super::TransparentSpendAuthority;

    test_cbor_roundtrip!(TransparentSpendAuthority);
}
