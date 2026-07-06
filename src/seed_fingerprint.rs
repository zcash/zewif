use crate::text_key;

text_key!(
    SeedFingerprint,
    "The fingerprint of an HD seed in its canonical string encoding: Bech32m
with the Human-Readable Part \"zip32seedfp\" over the 32 fingerprint bytes,
as defined in ZIP 32. Seed fingerprints are not network-bound, so a single
Human-Readable Part serves all networks.",
    "zip32seedfp1"
);

#[cfg(test)]
mod tests {
    use crate::test_cbor_roundtrip;

    use super::SeedFingerprint;

    test_cbor_roundtrip!(SeedFingerprint);
}
