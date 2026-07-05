use std::io::{self, Read, Write};

use crate::blob;

blob!(
    BlockHash,
    32,
    "A 32-byte block hash, displayed in reverse byte order by convention."
);
crate::blob_hex!(BlockHash, reversed);
impl Copy for BlockHash {}

impl BlockHash {
    /// Creates a new `BlockHash` from a 32-byte array.
    ///
    /// This is the primary constructor for `BlockHash` when you have the raw
    /// block hash available.
    ///
    /// # Examples
    /// ```
    /// # use zewif::BlockHash;
    /// // Usually this would be a real block hash
    /// let bytes = [0u8; 32];
    /// let block_hash = BlockHash::from_bytes(bytes);
    /// ```
    pub fn from_bytes(bytes: [u8; 32]) -> Self {
        Self::new(bytes)
    }

    /// Reads a `BlockHash` from any source implementing the `Read` trait.
    ///
    /// # Examples
    /// ```
    /// # use zewif::BlockHash;
    /// # use std::io::Cursor;
    /// # fn example() -> std::io::Result<()> {
    /// let bytes = [0u8; 32];
    /// let mut cursor = Cursor::new(bytes);
    /// let block_hash = BlockHash::read(&mut cursor)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn read<R: Read>(mut reader: R) -> io::Result<Self> {
        let mut hash = [0u8; 32];
        reader.read_exact(&mut hash)?;
        Ok(BlockHash::from_bytes(hash))
    }

    /// Writes a `BlockHash` to any destination implementing the `Write` trait.
    ///
    /// # Examples
    /// ```
    /// # use zewif::BlockHash;
    /// # fn example() -> std::io::Result<()> {
    /// let block_hash = BlockHash::from_bytes([0u8; 32]);
    /// let mut buffer = Vec::new();
    /// block_hash.write(&mut buffer)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn write<W: Write>(&self, mut writer: W) -> io::Result<()> {
        writer.write_all(self.as_slice())
    }
}

#[cfg(test)]
mod tests {
    use super::BlockHash;
    use crate::test_cbor_roundtrip;

    test_cbor_roundtrip!(BlockHash);

    #[test]
    fn hex_round_trips_in_display_order() {
        let hex = "000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f";
        let block_hash = BlockHash::from_hex(hex).unwrap();
        assert_eq!(block_hash.as_slice()[0], 0x6f);
        assert_eq!(block_hash.to_string(), hex);
    }
}
