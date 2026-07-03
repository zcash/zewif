use std::collections::BTreeMap;

use minicbor::{Decode, Encode};

use crate::Data;

/// Vendor-namespaced extension data attached to a ZeWIF record.
///
/// Entries are keyed first by a vendor identifier (reverse-DNS or another
/// collision-resistant convention is recommended) and then by a
/// vendor-chosen key. Each value byte string contains a single embedded
/// CBOR data item. Re-exporting software must preserve extension entries
/// it does not understand.
#[derive(Debug, Clone, PartialEq, Eq, Default, Encode, Decode)]
#[cbor(transparent)]
pub struct Extensions(#[n(0)] BTreeMap<String, BTreeMap<String, Data>>);

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
}
