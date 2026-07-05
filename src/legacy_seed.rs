use crate::blob;

blob!(
    LegacySeed,
    32,
    "A raw 32-byte pre-mnemonic HD seed.

The seed's ZIP 32 fingerprint is not stored here: the seed entry in the
secret store carries it, and it is derivable from the seed bytes."
);
crate::blob_encoding!(LegacySeed, redacted);

#[cfg(test)]
mod tests {
    use super::LegacySeed;
    use crate::test_cbor_roundtrip;

    test_cbor_roundtrip!(LegacySeed);
}
