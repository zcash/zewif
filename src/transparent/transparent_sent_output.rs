use minicbor::{Decode, Encode};

use crate::Amount;

/// Sender-side metadata for a transparent output not recoverable from the chain
/// without full transaction data.
///
/// Preserves the recipient address and value so the sending wallet can
/// reconstruct payment history.
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
#[cbor(map)]
pub struct TransparentSentOutput {
    /// Index of the output within the transaction's transparent output
    /// (vout) list (maps to zcash_client_sqlite sent_notes.output_index).
    #[n(0)]
    output_index: u32,

    /// The recipient transparent address string.
    #[n(1)]
    recipient_address: String,

    #[n(2)]
    value: Amount,
}

impl TransparentSentOutput {
    pub fn from_parts(output_index: u32, recipient_address: String, value: Amount) -> Self {
        Self {
            output_index,
            recipient_address,
            value,
        }
    }

    pub fn output_index(&self) -> u32 {
        self.output_index
    }

    pub fn set_output_index(&mut self, output_index: u32) {
        self.output_index = output_index;
    }

    pub fn recipient_address(&self) -> &str {
        &self.recipient_address
    }

    pub fn set_recipient_address(&mut self, recipient_address: String) {
        self.recipient_address = recipient_address;
    }

    pub fn value(&self) -> Amount {
        self.value
    }

    pub fn set_value(&mut self, value: Amount) {
        self.value = value;
    }
}

#[cfg(test)]
mod tests {
    use crate::{Amount, test_cbor_roundtrip};

    use super::TransparentSentOutput;

    impl crate::RandomInstance for TransparentSentOutput {
        fn random() -> Self {
            Self {
                output_index: u32::random() % 100,
                recipient_address: String::random(),
                value: Amount::random(),
            }
        }
    }

    test_cbor_roundtrip!(TransparentSentOutput);
}
