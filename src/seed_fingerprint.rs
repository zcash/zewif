use crate::blob;

blob!(
    SeedFingerprint,
    32,
    "The fingerprint of an HD seed, as defined in ZIP 32"
);
crate::blob_encoding!(SeedFingerprint, bytes);
impl Copy for SeedFingerprint {}

#[cfg(test)]
mod tests {
    use crate::test_cbor_roundtrip;

    use super::SeedFingerprint;

    test_cbor_roundtrip!(SeedFingerprint);
}
