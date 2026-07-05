use crate::blob;

blob!(
    Anchor,
    32,
    r#"A root of either the Sapling or Orchard Zcash note commitment trees.

`Anchor` is a cryptographic commitment that represents the root of a note commitment
tree at a specific block height. It is used in shielded transaction validation to
prove that a note being spent was part of the blockchain state without revealing which
specific note is being spent.

# Zcash Concept Relation
In Zcash's shielded protocols (Sapling and Orchard), each spent note must reference
an anchor to create a zero-knowledge proof showing:

1. The note exists in the blockchain as of a particular state (the anchor)
2. The spender has the authority to spend this note
3. The note has not been previously spent

The anchors are critical for maintaining privacy while preventing double-spending.

# Data Preservation
`Anchor` preserves the exact 256-bit root hash values from wallet data, which are
needed when reconstructing or validating shielded transactions during wallet migration.

# Examples
```
# use zewif::Anchor;
#
// Create an anchor from a u256 value
let anchor = Anchor::new([0u8; 32]);
```"#
);
crate::blob_encoding!(Anchor, bytes);
impl Copy for Anchor {}

#[cfg(test)]
mod tests {
    use crate::test_cbor_roundtrip;

    use super::Anchor;

    test_cbor_roundtrip!(Anchor);
}
