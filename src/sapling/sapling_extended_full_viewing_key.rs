use crate::blob;

// A hierarchical deterministic (HD) Sapling spending key with derivation information.
//
// `SaplingExtendedFullViewingKey` extends the core spending key functionality by adding the
// necessary components for hierarchical deterministic (HD) key derivation according to [ZIP 32].
// This enables the creation of structured wallet hierarchies with parent-child key relationships.
//
// This key is encoded as defined in https://zips.z.cash/zip-0032#sapling-extended-full-viewing-keys
//
// [ZIP 32]: https://zips.z.cash/zip-0032
// The ZIP 32 Sapling Extended Full Viewing Key encoding is 169 bytes:
//   depth(1) + parent_fvk_tag(4) + child_index(4) + chain_code(32)
//     + fvk(ak || nk || ovk = 96) + dk(32) = 169
blob!(
    SaplingExtendedFullViewingKey,
    169,
    "A Sapling Extended Full Viewing Key, encoded as specified in ZIP 32"
);
crate::blob_hex!(SaplingExtendedFullViewingKey, forward);

#[cfg(test)]
mod tests {
    use crate::test_cbor_roundtrip;

    use super::SaplingExtendedFullViewingKey;

    test_cbor_roundtrip!(SaplingExtendedFullViewingKey);
}
