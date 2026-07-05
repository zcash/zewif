use std::fmt::Display;

use crate::{blob, blob_envelope};

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

blob_envelope!(SaplingIncomingViewingKey);
