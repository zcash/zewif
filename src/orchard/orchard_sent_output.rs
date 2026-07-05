use bc_envelope::prelude::*;

use crate::{Amount, Indexed, Memo};

/// Sender-side metadata for an Orchard output not recoverable from the chain.
///
/// Preserves the recipient address, value, and memo so the sending wallet
/// can reconstruct payment history and generate proofs of payment.
#[derive(Debug, Clone, PartialEq)]
pub struct OrchardSentOutput {
    index: usize,

    /// Index of the action within the transaction's Orchard action list
    /// (maps to zcash_client_sqlite sent_notes.output_index).
    output_index: u32,

    /// The recipient address string (typically a unified address with an
    /// Orchard receiver). Preserved verbatim because unified address
    /// encoding includes receivers that can't be reconstructed from the
    /// Orchard component alone.
    recipient_address: String,

    value: Amount,

    memo: Option<Memo>,
}

impl Indexed for OrchardSentOutput {
    fn index(&self) -> usize {
        self.index
    }

    fn set_index(&mut self, index: usize) {
        self.index = index;
    }
}

impl OrchardSentOutput {
    pub fn from_parts(
        index: usize,
        output_index: u32,
        recipient_address: String,
        value: Amount,
        memo: Option<Memo>,
    ) -> Self {
        Self {
            index,
            output_index,
            recipient_address,
            value,
            memo,
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

    pub fn memo(&self) -> Option<&Memo> {
        self.memo.as_ref()
    }

    pub fn set_memo(&mut self, memo: Option<Memo>) {
        self.memo = memo;
    }
}

impl From<OrchardSentOutput> for Envelope {
    fn from(value: OrchardSentOutput) -> Self {
        Envelope::new(value.index)
            .add_type("OrchardSentOutput")
            .add_assertion("output_index", value.output_index)
            .add_assertion("recipient_address", value.recipient_address)
            .add_assertion("value", value.value)
            .add_optional_assertion("memo", value.memo)
    }
}

impl TryFrom<Envelope> for OrchardSentOutput {
    type Error = bc_envelope::Error;

    fn try_from(envelope: Envelope) -> bc_envelope::Result<Self> {
        envelope.check_type("OrchardSentOutput")?;
        let index = envelope.extract_subject()?;
        let output_index = envelope.extract_object_for_predicate("output_index")?;
        let recipient_address = envelope.extract_object_for_predicate("recipient_address")?;
        let value = envelope.extract_object_for_predicate("value")?;
        let memo = envelope.extract_optional_object_for_predicate("memo")?;

        Ok(OrchardSentOutput {
            index,
            output_index,
            recipient_address,
            value,
            memo,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{Amount, Memo, UnifiedAddress, test_envelope_roundtrip};

    use super::OrchardSentOutput;

    impl crate::RandomInstance for OrchardSentOutput {
        fn random() -> Self {
            Self {
                index: 0,
                output_index: u32::random() % 100,
                recipient_address: UnifiedAddress::random().address().to_string(),
                value: Amount::random(),
                memo: Some(Memo::random()),
            }
        }
    }

    test_envelope_roundtrip!(OrchardSentOutput);
}
