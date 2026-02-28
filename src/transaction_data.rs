use bc_envelope::prelude::*;

use crate::Data;

/// The transaction data available for a given transaction.
///
/// A wallet may have the full raw transaction (from a full node or from
/// `getrawtransaction`), or only the compact representation returned by a
/// light wallet server, or neither (only the txid is known).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransactionData {
    /// Full serialized transaction in the canonical Zcash encoding.
    Raw(Data),
    /// Compact transaction from a light wallet server.
    Compact {
        /// Protobuf-encoded CompactTx bytes.
        data: Data,
        /// Semver release version of the lightwallet protocol that
        /// produced this encoding (e.g. "1.0.0"). Needed because the
        /// protobuf schema is not self-describing.
        protocol_version: String,
    },
}

impl From<TransactionData> for Envelope {
    fn from(value: TransactionData) -> Self {
        match value {
            TransactionData::Raw(data) => {
                Envelope::new(data)
                    .add_type("TransactionData")
                    .add_assertion("format", "raw")
            }
            TransactionData::Compact { data, protocol_version } => {
                Envelope::new(data)
                    .add_type("TransactionData")
                    .add_assertion("format", "compact")
                    .add_assertion("protocol_version", protocol_version)
            }
        }
    }
}

impl TryFrom<Envelope> for TransactionData {
    type Error = bc_envelope::Error;

    fn try_from(envelope: Envelope) -> bc_envelope::Result<Self> {
        envelope.check_type("TransactionData")?;
        let data: Data = envelope.extract_subject()?;
        let format: String = envelope.extract_object_for_predicate("format")?;
        match format.as_str() {
            "raw" => Ok(TransactionData::Raw(data)),
            "compact" => {
                let protocol_version =
                    envelope.extract_object_for_predicate("protocol_version")?;
                Ok(TransactionData::Compact { data, protocol_version })
            }
            other => Err(bc_envelope::Error::General(
                format!("unknown transaction data format: {}", other),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{test_envelope_roundtrip, Data, RandomInstance};

    use super::TransactionData;

    impl RandomInstance for TransactionData {
        fn random() -> Self {
            use rand::Rng;
            let mut rng = rand::rng();
            if rng.random_bool(0.5) {
                TransactionData::Raw(Data::random())
            } else {
                TransactionData::Compact {
                    data: Data::random(),
                    protocol_version: "1.0.0".to_string(),
                }
            }
        }
    }

    test_envelope_roundtrip!(TransactionData);
}
