use crate::blob;

blob!(
    Nullifier,
    32,
    "A shielded note nullifier, revealed when the note is spent."
);
crate::blob_encoding!(Nullifier, bytes);
impl Copy for Nullifier {}
