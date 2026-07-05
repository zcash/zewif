use super::TransparentSpendingKey;
use crate::DerivationInfo;
use bc_envelope::prelude::*;

/// How a transparent address's key was obtained.
///
/// For HD-derived addresses, the derivation info is sufficient to recover
/// the spending key from the seed. For independently-generated keys (e.g.
/// legacy zcashd random-key addresses), the private key must be stored
/// directly.
#[derive(Debug, Clone, PartialEq)]
pub enum TransparentSpendAuthority {
    /// Key derived from an HD seed; derivation info is sufficient.
    Derived(DerivationInfo),
    /// Independently generated key; must store the private key.
    Imported(TransparentSpendingKey),
}

impl From<TransparentSpendAuthority> for Envelope {
    fn from(value: TransparentSpendAuthority) -> Self {
        match value {
            TransparentSpendAuthority::Derived(info) => {
                Envelope::new("derived")
                    .add_type("TransparentSpendAuthority")
                    .add_assertion("derivation_info", info)
            }
            TransparentSpendAuthority::Imported(key) => {
                Envelope::new(key)
                    .add_type("TransparentSpendAuthority")
                    .add_assertion("variant", "imported")
            }
        }
    }
}

impl TryFrom<Envelope> for TransparentSpendAuthority {
    type Error = bc_envelope::Error;

    fn try_from(envelope: Envelope) -> bc_envelope::Result<Self> {
        envelope.check_type("TransparentSpendAuthority")?;
        if let Ok(info) = envelope.try_object_for_predicate::<DerivationInfo>("derivation_info") {
            Ok(TransparentSpendAuthority::Derived(info))
        } else {
            let key: TransparentSpendingKey = envelope.extract_subject()?;
            Ok(TransparentSpendAuthority::Imported(key))
        }
    }
}

#[cfg(test)]
impl crate::RandomInstance for TransparentSpendAuthority {
    fn random() -> Self {
        use rand::Rng;
        let mut rng = rand::rng();
        if rng.random_bool(0.5) {
            TransparentSpendAuthority::Derived(DerivationInfo::random())
        } else {
            TransparentSpendAuthority::Imported(TransparentSpendingKey::random())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::test_envelope_roundtrip;

    use super::TransparentSpendAuthority;

    test_envelope_roundtrip!(TransparentSpendAuthority);
}
