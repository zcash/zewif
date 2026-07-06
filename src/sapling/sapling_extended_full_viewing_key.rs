use crate::text_key;

text_key!(
    SaplingExtendedFullViewingKey,
    "A Sapling extended full viewing key in its canonical Bech32 encoding
per ZIP 32 (Human-Readable Part \"zxviews\" on mainnet,
\"zxviewtestsapling\" on testnet).",
    "zxviews1"
);

#[cfg(test)]
mod tests {
    use super::SaplingExtendedFullViewingKey;
    use crate::test_cbor_roundtrip;

    test_cbor_roundtrip!(SaplingExtendedFullViewingKey);
}
