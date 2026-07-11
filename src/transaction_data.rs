use minicbor::{Decode, Encode};

use crate::Data;

/// The transaction data available for a given transaction.
///
/// A wallet may have the full raw transaction (from a full node or from
/// `getrawtransaction`), or only the compact representation returned by a
/// light wallet server, or neither (only the txid is known).
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
#[cbor(flat)]
pub enum TransactionData {
    /// Full serialized transaction in the canonical Zcash encoding.
    #[n(0)]
    Raw(#[n(0)] RawTxData),
    /// Compact transaction from a light wallet server.
    #[n(1)]
    Compact(#[n(0)] CompactTxData),
}

/// A full serialized transaction in the canonical Zcash encoding.
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
#[cbor(map)]
pub struct RawTxData {
    #[n(0)]
    data: Data,
}

impl RawTxData {
    pub fn new(data: Data) -> Self {
        Self { data }
    }

    pub fn data(&self) -> &Data {
        &self.data
    }
}

/// A compact transaction from a light wallet server, together with the
/// protocol version that produced it.
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
#[cbor(map)]
pub struct CompactTxData {
    /// Protobuf-encoded CompactTx bytes.
    #[n(0)]
    data: Data,
    /// Semver release version of the lightwallet protocol that
    /// produced this encoding (e.g. "1.0.0"). Needed because the
    /// protobuf schema is not self-describing.
    #[n(1)]
    protocol_version: String,
}

impl CompactTxData {
    pub fn new(data: Data, protocol_version: impl Into<String>) -> Self {
        Self {
            data,
            protocol_version: protocol_version.into(),
        }
    }

    pub fn data(&self) -> &Data {
        &self.data
    }

    pub fn protocol_version(&self) -> &str {
        &self.protocol_version
    }
}

#[cfg(test)]
mod tests {
    use crate::{Data, RandomInstance, test_cbor_roundtrip};

    use super::{CompactTxData, RawTxData, TransactionData};

    impl RandomInstance for TransactionData {
        fn random() -> Self {
            use rand::Rng;
            let mut rng = rand::rng();
            if rng.random_bool(0.5) {
                TransactionData::Raw(RawTxData::new(Data::random()))
            } else {
                TransactionData::Compact(CompactTxData::new(Data::random(), "1.0.0"))
            }
        }
    }

    test_cbor_roundtrip!(TransactionData);
}
