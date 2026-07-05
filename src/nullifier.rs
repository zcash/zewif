use crate::blob;

blob!(
    Nullifier,
    32,
    "A shielded note nullifier, revealed when the note is spent."
);
impl Copy for Nullifier {}
