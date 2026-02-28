use super::Data;
use bc_envelope::prelude::*;
use std::ops::{
    Index, IndexMut, Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive,
};

/// A serialized Bitcoin-style script (scriptPubKey or scriptSig).
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Script(Data);

impl Script {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Debug formatting that includes script length and hex representation
impl std::fmt::Debug for Script {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Script<{}>({})", self.0.len(), hex::encode(self))
    }
}

/// Allows treating a Script as a byte slice
impl AsRef<[u8]> for Script {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

/// Converts a Script to a Data value, allowing manipulation as variable-length bytes
impl From<Script> for Data {
    fn from(script: Script) -> Self {
        script.0
    }
}

/// Creates a Script from Data, allowing conversion from variable-length bytes
impl From<Data> for Script {
    fn from(data: Data) -> Self {
        Script(data)
    }
}

/// Allows accessing individual bytes in the script by index
impl Index<usize> for Script {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

/// Allows modifying individual bytes in the script by index
impl IndexMut<usize> for Script {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl Index<Range<usize>> for Script {
    type Output = [u8];

    fn index(&self, index: Range<usize>) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<Range<usize>> for Script {
    fn index_mut(&mut self, index: Range<usize>) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl Index<RangeTo<usize>> for Script {
    type Output = [u8];

    fn index(&self, index: RangeTo<usize>) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<RangeTo<usize>> for Script {
    fn index_mut(&mut self, index: RangeTo<usize>) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl Index<RangeFrom<usize>> for Script {
    type Output = [u8];

    fn index(&self, index: RangeFrom<usize>) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<RangeFrom<usize>> for Script {
    fn index_mut(&mut self, index: RangeFrom<usize>) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl Index<RangeFull> for Script {
    type Output = [u8];

    fn index(&self, index: RangeFull) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<RangeFull> for Script {
    fn index_mut(&mut self, index: RangeFull) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl Index<RangeInclusive<usize>> for Script {
    type Output = [u8];

    fn index(&self, index: RangeInclusive<usize>) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<RangeInclusive<usize>> for Script {
    fn index_mut(&mut self, index: RangeInclusive<usize>) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl Index<RangeToInclusive<usize>> for Script {
    type Output = [u8];

    fn index(&self, index: RangeToInclusive<usize>) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<RangeToInclusive<usize>> for Script {
    fn index_mut(&mut self, index: RangeToInclusive<usize>) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl From<Script> for CBOR {
    fn from(value: Script) -> Self {
        CBOR::to_byte_string(value.0)
    }
}

impl From<&Script> for CBOR {
    fn from(value: &Script) -> Self {
        CBOR::to_byte_string(value.0.clone())
    }
}

impl TryFrom<CBOR> for Script {
    type Error = dcbor::Error;

    fn try_from(cbor: CBOR) -> dcbor::Result<Self> {
        let bytes = cbor.try_into_byte_string()?;
        if bytes.len() > 0xffff {
            return Err("Script length exceeds maximum size of 65535 bytes".into());
        }
        Ok(Script(Data::from_vec(bytes)))
    }
}

impl From<Script> for Envelope {
    fn from(value: Script) -> Self {
        Envelope::new(CBOR::from(value))
    }
}

impl TryFrom<Envelope> for Script {
    type Error = bc_envelope::Error;

    fn try_from(envelope: Envelope) -> bc_envelope::Result<Self> {
        envelope.extract_subject()
    }
}

#[cfg(test)]
mod tests {
    use crate::{Data, test_cbor_roundtrip, test_envelope_roundtrip};

    use super::Script;

    impl crate::RandomInstance for Script {
        fn random_with_size(size: usize) -> Self {
            Self(Data::random_with_size(size))
        }

        fn random() -> Self {
            Self(Data::random())
        }
    }

    test_cbor_roundtrip!(Script);
    test_envelope_roundtrip!(Script);
}
