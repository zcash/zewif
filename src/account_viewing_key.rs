use bc_envelope::prelude::*;

use crate::{
    sapling::SaplingExtendedFullViewingKey,
    sprout::SproutViewingKey,
    UnifiedFullViewingKey,
};

/// The viewing capability associated with an account.
///
/// This determines what the account can observe on-chain. Each variant
/// corresponds to a different era or style of Zcash key management.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AccountViewingKey {
    /// A Unified Full Viewing Key (canonical ZIP-316 encoding), which may
    /// contain Orchard, Sapling, and/or transparent components.
    Ufvk(UnifiedFullViewingKey),
    /// A standalone Sapling extended full viewing key (canonical encoding).
    SaplingExtFvk(SaplingExtendedFullViewingKey),
    /// A Sprout viewing key (64 bytes: `a_pk` + `sk_enc`). Sufficient to
    /// detect incoming Sprout notes.
    SproutViewingKey(SproutViewingKey),
    /// A set of transparent addresses with no unified key structure
    /// (legacy zcashd random-key wallet).
    TransparentAddressSet,
}

impl From<AccountViewingKey> for Envelope {
    fn from(value: AccountViewingKey) -> Self {
        let (variant, e) = match value {
            AccountViewingKey::Ufvk(ufvk) => {
                ("ufvk", Envelope::new(ufvk.encoding()))
            }
            AccountViewingKey::SaplingExtFvk(fvk) => {
                ("sapling_ext_fvk", Envelope::new(fvk))
            }
            AccountViewingKey::SproutViewingKey(key) => {
                ("sprout_viewing_key", Envelope::new(key.data().clone()))
            }
            AccountViewingKey::TransparentAddressSet => {
                ("transparent_address_set", Envelope::new("transparent_address_set"))
            }
        };
        e.add_type("AccountViewingKey")
            .add_assertion("variant", variant)
    }
}

impl TryFrom<Envelope> for AccountViewingKey {
    type Error = bc_envelope::Error;

    fn try_from(envelope: Envelope) -> bc_envelope::Result<Self> {
        envelope.check_type("AccountViewingKey")?;
        let variant: String = envelope.extract_object_for_predicate("variant")?;
        match variant.as_str() {
            "ufvk" => {
                let encoding: String = envelope.extract_subject()?;
                Ok(AccountViewingKey::Ufvk(UnifiedFullViewingKey::new(encoding)))
            }
            "sapling_ext_fvk" => {
                let fvk: SaplingExtendedFullViewingKey = envelope.extract_subject()?;
                Ok(AccountViewingKey::SaplingExtFvk(fvk))
            }
            "sprout_viewing_key" => {
                let data: crate::Data = envelope.extract_subject()?;
                Ok(AccountViewingKey::SproutViewingKey(SproutViewingKey::new(data)))
            }
            "transparent_address_set" => {
                Ok(AccountViewingKey::TransparentAddressSet)
            }
            other => Err(bc_envelope::Error::General(
                format!("unknown AccountViewingKey variant: {}", other),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{test_envelope_roundtrip, RandomInstance};

    use super::AccountViewingKey;

    impl RandomInstance for AccountViewingKey {
        fn random() -> Self {
            use rand::Rng;
            let mut rng = rand::rng();
            match rng.random_range(0..4u32) {
                0 => AccountViewingKey::Ufvk(
                    crate::UnifiedFullViewingKey::random(),
                ),
                1 => AccountViewingKey::SaplingExtFvk(
                    crate::sapling::SaplingExtendedFullViewingKey::random(),
                ),
                2 => AccountViewingKey::SproutViewingKey(
                    crate::sprout::SproutViewingKey::random(),
                ),
                _ => AccountViewingKey::TransparentAddressSet,
            }
        }
    }

    test_envelope_roundtrip!(AccountViewingKey);
}
