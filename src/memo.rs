//! A memo associated with a Zcash shielded output.

use crate::data;

data!(Memo, "A memo associated with a Zcash shielded output.");

#[cfg(test)]
mod tests {
    use crate::test_cbor_roundtrip;

    use super::Memo;

    test_cbor_roundtrip!(Memo);
}
