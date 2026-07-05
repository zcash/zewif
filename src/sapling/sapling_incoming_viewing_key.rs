use crate::blob;

blob!(
    SaplingIncomingViewingKey,
    32,
    "A 32-byte Sapling Incoming Viewing Key, enabling detection of incoming notes."
);
crate::blob_encoding!(SaplingIncomingViewingKey, bytes);

impl Copy for SaplingIncomingViewingKey {}

#[cfg(test)]
mod tests {
    use crate::test_cbor_roundtrip;

    use super::SaplingIncomingViewingKey;

    test_cbor_roundtrip!(SaplingIncomingViewingKey);
}
