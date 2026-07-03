use minicbor::{Decode, Encode};

use crate::{Amount, Memo};

/// Sender-side metadata for a Sapling output not recoverable from the chain.
///
/// Preserves the recipient address, value, and memo so the sending wallet
/// can reconstruct payment history and generate proofs of payment.
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
#[cbor(map)]
pub struct SaplingSentOutput {
    /// Index of the output within the transaction's Sapling output list
    /// (maps to zcash_client_sqlite sent_notes.output_index).
    #[n(0)]
    output_index: u32,

    /// The recipient address string (a Sapling address or a unified address
    /// with a Sapling receiver). Preserved verbatim because unified address
    /// encoding includes receivers that can't be reconstructed from the
    /// Sapling component alone.
    #[n(1)]
    recipient_address: String,

    #[n(2)]
    value: Amount,

    #[n(3)]
    memo: Option<Memo>,
}

impl SaplingSentOutput {
    pub fn new() -> Self {
        Self {
            output_index: 0,
            recipient_address: "".to_string(),
            value: Amount::zero(),
            memo: None,
        }
    }

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

impl Default for SaplingSentOutput {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::SaplingSentOutput;
    use crate::{Amount, Memo, test_cbor_roundtrip};

    impl crate::RandomInstance for SaplingSentOutput {
        fn random() -> Self {
            Self {
                output_index: u32::random() % 100,
                recipient_address: String::random(),
                value: Amount::random(),
                memo: Some(Memo::random()),
            }
        }
    }

    test_cbor_roundtrip!(SaplingSentOutput);
}
