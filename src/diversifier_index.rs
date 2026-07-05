use crate::blob;

blob!(
    DiversifierIndex,
    11,
    "An 11-byte ZIP 32 diversifier index identifying which diversified
address of a viewing key an address corresponds to."
);
crate::blob_hex!(DiversifierIndex, forward);
impl Copy for DiversifierIndex {}
