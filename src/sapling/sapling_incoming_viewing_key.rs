use std::fmt::Display;

use crate::blob;

blob!(
    SaplingIncomingViewingKey,
    32,
    "A 32-byte Sapling Incoming Viewing Key, enabling detection of incoming notes."
);

impl Copy for SaplingIncomingViewingKey {}

impl Display for SaplingIncomingViewingKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_hex())
    }
}

#[cfg(test)]
mod tests {
    use crate::test_cbor_roundtrip;

    use super::SaplingIncomingViewingKey;

    test_cbor_roundtrip!(SaplingIncomingViewingKey);
}
