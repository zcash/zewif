use minicbor::{Decode, Encode};

/// A Unified Full Viewing Key in its canonical ZIP-316 encoding.
///
/// UFVKs bundle viewing keys for multiple Zcash protocols (transparent,
/// Sapling, Orchard) into a single key. The encoded string is the
/// canonical representation and can be parsed by any UFVK-aware wallet.
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
#[cbor(map)]
pub struct UnifiedFullViewingKey {
    #[n(0)]
    encoding: String,
}

impl UnifiedFullViewingKey {
    pub fn new(encoding: impl Into<String>) -> Self {
        Self {
            encoding: encoding.into(),
        }
    }

    pub fn encoding(&self) -> &str {
        &self.encoding
    }
}

#[cfg(test)]
mod tests {
    use crate::{RandomInstance, test_cbor_roundtrip};

    use super::UnifiedFullViewingKey;

    impl RandomInstance for UnifiedFullViewingKey {
        fn random() -> Self {
            // Use a placeholder UFVK-like string for testing
            Self {
                encoding: format!("uview1{}", String::random()),
            }
        }
    }

    test_cbor_roundtrip!(UnifiedFullViewingKey);
}
