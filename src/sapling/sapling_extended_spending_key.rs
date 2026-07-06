use crate::text_key;

text_key!(
    SaplingExtendedSpendingKey,
    "A Sapling extended spending key in its canonical Bech32 encoding per
ZIP 32 (Human-Readable Part \"secret-extended-key-main\" on mainnet,
\"secret-extended-key-test\" on testnet).",
    "secret-extended-key-main1",
    redacted
);

#[cfg(test)]
mod tests {
    use super::SaplingExtendedSpendingKey;
    use crate::test_cbor_roundtrip;

    test_cbor_roundtrip!(SaplingExtendedSpendingKey);
}
