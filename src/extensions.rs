use std::collections::BTreeMap;

use minicbor::Decode;
use minicbor::encode::{Error as EncodeError, Write};
use minicbor::{Encode, Encoder};

use crate::Data;

/// Vendor-namespaced extension data attached to a ZeWIF record.
///
/// Entries are keyed first by a vendor identifier (reverse-DNS or another
/// collision-resistant convention is recommended) and then by a
/// vendor-chosen key. Each value byte string contains a single embedded
/// CBOR data item. Re-exporting software must preserve extension entries
/// it does not understand.
///
/// The [`Encode`] implementation is hand-written to satisfy RFC 8949 §4.2:
/// `BTreeMap`'s iteration order is the content-lexicographic `String` `Ord`,
/// but §4.2 requires map keys to be sorted by the bytewise order of their
/// *encodings*. For a definite-length text string that order is length-first
/// (the initial byte carries the length), so `"z"` sorts before `"ab"`.
/// [`Decode`] is order-agnostic, so the derived transparent decoder is kept.
#[derive(Debug, Clone, PartialEq, Eq, Default, Decode)]
#[cbor(transparent)]
pub struct Extensions(#[n(0)] BTreeMap<String, BTreeMap<String, Data>>);

/// Emits a string-keyed map as a definite-length CBOR map whose keys are in
/// RFC 8949 §4.2 bytewise order: for definite-length text strings that is
/// length-first, then bytewise over the UTF-8 bytes.
fn encode_map_deterministic<C, W, V, F>(
    map: &BTreeMap<String, V>,
    e: &mut Encoder<W>,
    ctx: &mut C,
    mut encode_value: F,
) -> Result<(), EncodeError<W::Error>>
where
    W: Write,
    F: FnMut(&V, &mut Encoder<W>, &mut C) -> Result<(), EncodeError<W::Error>>,
{
    let mut entries: Vec<(&String, &V)> = map.iter().collect();
    entries.sort_by(|(a, _), (b, _)| {
        a.len()
            .cmp(&b.len())
            .then_with(|| a.as_bytes().cmp(b.as_bytes()))
    });
    e.map(entries.len() as u64)?;
    for (key, value) in entries {
        e.str(key)?;
        encode_value(value, e, ctx)?;
    }
    Ok(())
}

impl<C> Encode<C> for Extensions {
    fn encode<W: Write>(
        &self,
        e: &mut Encoder<W>,
        ctx: &mut C,
    ) -> Result<(), EncodeError<W::Error>> {
        encode_map_deterministic(&self.0, e, ctx, |inner, e, ctx| {
            encode_map_deterministic(inner, e, ctx, |data, e, ctx| Encode::encode(data, e, ctx))
        })
    }
}

impl Extensions {
    /// Creates an empty extension set.
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    /// Returns `true` if no extension data is present.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Adds an extension value under the given vendor identifier and key.
    pub fn add(&mut self, vendor: impl Into<String>, key: impl Into<String>, value: Data) {
        self.0
            .entry(vendor.into())
            .or_default()
            .insert(key.into(), value);
    }

    /// Returns the extension value stored under the given vendor identifier
    /// and key, if present.
    pub fn get(&self, vendor: &str, key: &str) -> Option<&Data> {
        self.0.get(vendor).and_then(|entries| entries.get(key))
    }

    /// Iterates over all extension entries as `(vendor, key, value)` triples.
    pub fn iter(&self) -> impl Iterator<Item = (&str, &str, &Data)> {
        self.0.iter().flat_map(|(vendor, entries)| {
            entries
                .iter()
                .map(move |(key, value)| (vendor.as_str(), key.as_str(), value))
        })
    }
}

/// Field codec for `extensions` record fields: the map entry is omitted
/// when the extension set is empty, and an absent (or null) entry decodes
/// as an empty set.
///
/// Use as `#[cbor(n(IDX), with = "crate::extensions_field", has_nil)]`.
#[doc(hidden)]
pub mod extensions_field {
    use minicbor::decode::Error as DecodeError;
    use minicbor::encode::{Error as EncodeError, Write};
    use minicbor::{Decoder, Encoder};

    use super::Extensions;

    pub fn encode<Ctx, W: Write>(
        v: &Extensions,
        e: &mut Encoder<W>,
        ctx: &mut Ctx,
    ) -> Result<(), EncodeError<W::Error>> {
        minicbor::Encode::encode(v, e, ctx)
    }

    pub fn decode<'b, Ctx>(d: &mut Decoder<'b>, ctx: &mut Ctx) -> Result<Extensions, DecodeError> {
        if d.datatype()? == minicbor::data::Type::Null {
            d.skip()?;
            return Ok(Extensions::new());
        }
        minicbor::Decode::decode(d, ctx)
    }

    pub fn nil() -> Option<Extensions> {
        Some(Extensions::new())
    }

    pub fn is_nil(v: &Extensions) -> bool {
        v.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use crate::{Data, RandomInstance, test_cbor_roundtrip};

    use super::Extensions;

    impl RandomInstance for Extensions {
        fn random() -> Self {
            use rand::Rng;
            let mut rng = rand::rng();
            let mut extensions = Extensions::new();
            for _ in 0..rng.random_range(0..=3) {
                extensions.add(String::random(), String::random(), Data::random());
            }
            extensions
        }
    }

    test_cbor_roundtrip!(Extensions);

    /// The encoded wire bytes place shorter keys before longer ones at both
    /// nesting levels, per RFC 8949 §4.2 (bytewise order of the encoded
    /// definite-length text keys). Content-lexicographic `String` `Ord` —
    /// what `BTreeMap` iteration and the pre-fix derived encoder produced —
    /// would instead place "alpha" before "zeta" and "ab" before "z".
    #[test]
    fn keys_are_encoded_in_length_first_order() {
        let mut ext = Extensions::new();
        // Outer keys "zeta" (4 bytes) and "alpha" (5 bytes): length-first
        // places "zeta" first even though 'a' < 'z'.
        // Inner keys "z" (1 byte) and "ab" (2 bytes): length-first places
        // "z" first even though 'a' < 'z'.
        ext.add("zeta", "z", Data::from_hex("01").unwrap());
        ext.add("zeta", "ab", Data::from_hex("02").unwrap());
        ext.add("alpha", "z", Data::from_hex("03").unwrap());

        #[rustfmt::skip]
        let expected: Vec<u8> = vec![
            0xa2,                               // outer map, 2 entries
                0x64, 0x7a, 0x65, 0x74, 0x61,   // "zeta"
                0xa2,                           //   inner map, 2 entries
                    0x61, 0x7a,                 //   "z"
                    0x41, 0x01,                 //   h'01'
                    0x62, 0x61, 0x62,           //   "ab"
                    0x41, 0x02,                 //   h'02'
                0x65, 0x61, 0x6c, 0x70, 0x68, 0x61, // "alpha"
                0xa1,                           //   inner map, 1 entry
                    0x61, 0x7a,                 //   "z"
                    0x41, 0x03,                 //   h'03'
        ];
        assert_eq!(minicbor::to_vec(&ext).unwrap(), expected);
    }
}
