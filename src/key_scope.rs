use bc_envelope::prelude::*;

/// The role of an address within its account (maps to zcash_client_sqlite
/// addresses.key_scope encodings 0/1/2/-1).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum KeyScope {
    /// User-facing receiving addresses.
    External,
    /// Wallet-internal change/shielding addresses; never exposed to users.
    Internal,
    /// ZIP 320 single-use ephemeral transparent addresses.
    Ephemeral,
    /// Imported standalone keys/scripts not derived from account key material.
    Foreign,
}

impl From<KeyScope> for Envelope {
    fn from(value: KeyScope) -> Self {
        let scope = match value {
            KeyScope::External => "external",
            KeyScope::Internal => "internal",
            KeyScope::Ephemeral => "ephemeral",
            KeyScope::Foreign => "foreign",
        };
        Envelope::new(scope).add_type("KeyScope")
    }
}

impl TryFrom<Envelope> for KeyScope {
    type Error = bc_envelope::Error;

    fn try_from(envelope: Envelope) -> bc_envelope::Result<Self> {
        envelope.check_type("KeyScope")?;
        let scope: String = envelope.extract_subject()?;
        match scope.as_str() {
            "external" => Ok(KeyScope::External),
            "internal" => Ok(KeyScope::Internal),
            "ephemeral" => Ok(KeyScope::Ephemeral),
            "foreign" => Ok(KeyScope::Foreign),
            other => Err(bc_envelope::Error::General(format!(
                "unknown KeyScope: {}",
                other
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::test_envelope_roundtrip;

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

    test_envelope_roundtrip!(KeyScope);
}
