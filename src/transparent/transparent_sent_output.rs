use bc_envelope::prelude::*;

use crate::{Amount, Indexed};

/// Sender-side metadata for a transparent output not recoverable from the chain
/// without full transaction data.
///
/// Preserves the recipient address and value so the sending wallet can
/// reconstruct payment history.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransparentSentOutput {
    index: usize,

    /// Index of the output within the transaction's transparent output
    /// (vout) list (maps to zcash_client_sqlite sent_notes.output_index).
    output_index: u32,

    /// The recipient transparent address string.
    recipient_address: String,

    value: Amount,
}

impl Indexed for TransparentSentOutput {
    fn index(&self) -> usize {
        self.index
    }

    fn set_index(&mut self, index: usize) {
        self.index = index;
    }
}

impl TransparentSentOutput {
    pub fn from_parts(
        index: usize,
        output_index: u32,
        recipient_address: String,
        value: Amount,
    ) -> Self {
        Self {
            index,
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

impl From<TransparentSentOutput> for Envelope {
    fn from(value: TransparentSentOutput) -> Self {
        Envelope::new(value.index)
            .add_type("TransparentSentOutput")
            .add_assertion("output_index", value.output_index)
            .add_assertion("recipient_address", value.recipient_address)
            .add_assertion("value", value.value)
    }
}

impl TryFrom<Envelope> for TransparentSentOutput {
    type Error = bc_envelope::Error;

    fn try_from(envelope: Envelope) -> bc_envelope::Result<Self> {
        envelope.check_type("TransparentSentOutput")?;
        let index = envelope.extract_subject()?;
        let output_index = envelope.extract_object_for_predicate("output_index")?;
        let recipient_address = envelope.extract_object_for_predicate("recipient_address")?;
        let value = envelope.extract_object_for_predicate("value")?;

        Ok(TransparentSentOutput {
            index,
            output_index,
            recipient_address,
            value,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{Amount, test_envelope_roundtrip};

    use super::TransparentSentOutput;

    impl crate::RandomInstance for TransparentSentOutput {
        fn random() -> Self {
            Self {
                index: 0,
                output_index: u32::random() % 100,
                recipient_address: String::random(),
                value: Amount::random(),
            }
        }
    }

    test_envelope_roundtrip!(TransparentSentOutput);
}
