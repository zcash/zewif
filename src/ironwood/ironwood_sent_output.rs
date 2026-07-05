use minicbor::{Decode, Encode};

use crate::{Amount, Memo};

/// Sender-side metadata for an Ironwood output not recoverable from the chain.
///
/// Preserves the recipient address, value, and memo so the sending wallet
/// can reconstruct payment history and generate proofs of payment.
#[derive(Debug, Clone, PartialEq, Encode, Decode)]
#[cbor(map)]
pub struct IronwoodSentOutput {
    /// Index of the action within the transaction's Ironwood action list
    /// (maps to zcash_client_sqlite sent_notes.output_index).
    #[n(0)]
    output_index: u32,

    /// The recipient address string (typically a unified address with an
    /// Ironwood receiver). Preserved verbatim because unified address
    /// encoding includes receivers that can't be reconstructed from the
    /// Ironwood component alone.
    #[n(1)]
    recipient_address: String,

    #[n(2)]
    value: Amount,

    #[n(3)]
    memo: Option<Memo>,
}

impl IronwoodSentOutput {
    pub fn from_parts(
        output_index: u32,
        recipient_address: String,
        value: Amount,
        memo: Option<Memo>,
    ) -> Self {
        Self {
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

#[cfg(test)]
mod tests {
    use crate::{Amount, Memo, UnifiedAddress, test_cbor_roundtrip};

    use super::IronwoodSentOutput;

    impl crate::RandomInstance for IronwoodSentOutput {
        fn random() -> Self {
            Self {
                output_index: u32::random() % 100,
                recipient_address: UnifiedAddress::random().address().to_string(),
                value: Amount::random(),
                memo: Some(Memo::random()),
            }
        }
    }

    test_cbor_roundtrip!(IronwoodSentOutput);
}
