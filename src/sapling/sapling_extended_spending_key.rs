use crate::blob;

// A hierarchical deterministic (HD) Sapling spending key with derivation information.
//
// `SaplingExtendedSpendingKey` extends the core spending key functionality by adding the necessary
// components for hierarchical deterministic (HD) key derivation according to [ZIP 32]. This
// enables the creation of structured wallet hierarchies with parent-child key relationships.
//
// This key is encoded as defined in https://zips.z.cash/zip-0032#sapling-extended-spending-keys
//
// [ZIP 32]: https://zips.z.cash/zip-0032
blob!(
    SaplingExtendedSpendingKey,
    169,
    "A Sapling Extended Spending Key, encoded as specified in ZIP 32"
);
crate::blob_hex!(SaplingExtendedSpendingKey, forward);

#[cfg(test)]
mod tests {
    use crate::test_cbor_roundtrip;

    use super::SaplingExtendedSpendingKey;

    test_cbor_roundtrip!(SaplingExtendedSpendingKey);
}
