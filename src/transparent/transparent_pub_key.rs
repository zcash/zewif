use minicbor::{Decode, Encode};

/// A secp256k1 public key in its serialized form: 33 bytes (compressed) or
/// 65 bytes (uncompressed).
///
/// Uncompressed keys occur in watch-only imports from legacy zcashd
/// wallets; some importers accept only compressed keys, but the data is
/// carried in either form so that it survives interchange.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TransparentPubKey(Vec<u8>);

impl TransparentPubKey {
    /// Creates a public key from its serialized bytes, which must be 33
    /// (compressed) or 65 (uncompressed) bytes long.
    pub fn from_bytes(data: impl Into<Vec<u8>>) -> crate::Result<Self> {
        let data = data.into();
        if data.len() == 33 || data.len() == 65 {
            Ok(Self(data))
        } else {
            Err(crate::Error::InvalidTransparentPubKeyLength(data.len()))
        }
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.0
    }

    /// Whether this is the 33-byte compressed form.
    pub fn is_compressed(&self) -> bool {
        self.0.len() == 33
    }
}

impl AsRef<[u8]> for TransparentPubKey {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl<C> Encode<C> for TransparentPubKey {
    fn encode<W: minicbor::encode::Write>(
        &self,
        e: &mut minicbor::Encoder<W>,
        _ctx: &mut C,
    ) -> Result<(), minicbor::encode::Error<W::Error>> {
        e.bytes(&self.0)?;
        Ok(())
    }
}

impl<'b, C> Decode<'b, C> for TransparentPubKey {
    fn decode(
        d: &mut minicbor::Decoder<'b>,
        _ctx: &mut C,
    ) -> Result<Self, minicbor::decode::Error> {
        let bytes = d.bytes()?;
        Self::from_bytes(bytes.to_vec()).map_err(|_| {
            minicbor::decode::Error::message(
                "expected a byte string of length 33 or 65 for TransparentPubKey",
            )
        })
    }
}

#[cfg(test)]
mod tests {
    use super::TransparentPubKey;
    use crate::test_cbor_roundtrip;

    impl crate::RandomInstance for TransparentPubKey {
        fn random() -> Self {
            use rand::Rng;
            let mut rng = rand::rng();
            let len = if rng.random_bool(0.8) { 33 } else { 65 };
            let mut data = vec![0u8; len];
            rng.fill(&mut data[..]);
            TransparentPubKey::from_bytes(data).expect("valid length")
        }
    }

    test_cbor_roundtrip!(TransparentPubKey);

    #[test]
    fn rejects_invalid_lengths() {
        assert!(TransparentPubKey::from_bytes(vec![0u8; 32]).is_err());
        assert!(TransparentPubKey::from_bytes(vec![0u8; 64]).is_err());
    }
}
