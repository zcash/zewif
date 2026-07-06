use minicbor::{Decode, Encode};

use crate::NonHardenedChildIndex;

/// BIP-44/ZIP-32 derivation path components (change flag + address index).
///
/// Captures the last two non-hardened path components:
/// `m / purpose' / coin_type' / account' / change / address_index`
///
/// # Examples
/// ```
/// # use zewif::{DerivationInfo, NonHardenedChildIndex};
/// let change = NonHardenedChildIndex::from(0u32); // external
/// let address_index = NonHardenedChildIndex::from(5u32);
///
/// let derivation_info = DerivationInfo::new(change, address_index);
///
/// // The values can be retrieved for further derivation or reference
/// assert_eq!(u32::from(derivation_info.change()), 0);
/// assert_eq!(u32::from(derivation_info.address_index()), 5);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Encode, Decode)]
#[cbor(map)]
pub struct DerivationInfo {
    /// The change level (0 = external addresses, 1 = internal/change
    /// addresses, 2 = ephemeral addresses)
    #[n(0)]
    change: NonHardenedChildIndex,

    /// The address index at the specified change level
    #[n(1)]
    address_index: NonHardenedChildIndex,
}

impl DerivationInfo {
    /// Creates a new `DerivationInfo` with the specified change and address
    /// index components.
    pub fn new(change: NonHardenedChildIndex, address_index: NonHardenedChildIndex) -> Self {
        Self {
            change,
            address_index,
        }
    }

    /// Returns the change component of the derivation path
    /// (0 for external, 1 for internal/change, 2 for ephemeral addresses).
    pub fn change(&self) -> NonHardenedChildIndex {
        self.change
    }

    /// Returns the address index component of the derivation path: the
    /// sequential index of an address within its chain.
    pub fn address_index(&self) -> NonHardenedChildIndex {
        self.address_index
    }
}

#[cfg(test)]
mod tests {
    use crate::{NonHardenedChildIndex, test_cbor_roundtrip};

    use super::DerivationInfo;

    impl crate::RandomInstance for DerivationInfo {
        fn random() -> Self {
            Self {
                change: NonHardenedChildIndex::random(),
                address_index: NonHardenedChildIndex::random(),
            }
        }
    }

    test_cbor_roundtrip!(DerivationInfo);
}
