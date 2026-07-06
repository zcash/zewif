use crate::text_key;

text_key!(
    TransparentSpendingKey,
    "A transparent (secp256k1) private key in its canonical WIF Base58Check
encoding (version byte 0x80 on mainnet, 0xEF on testnet; a trailing 0x01
before the checksum marks a compressed public key).",
    "L",
    redacted
);

#[cfg(test)]
mod tests {
    use super::TransparentSpendingKey;
    use crate::test_cbor_roundtrip;

    test_cbor_roundtrip!(TransparentSpendingKey);
}
