use std::io::{self, Read, Write};

use crate::blob;

blob!(
    TxId,
    32,
    "A 32-byte transaction identifier, displayed in reverse byte order by convention."
);
crate::blob_encoding!(TxId, reversed_hex);
impl Copy for TxId {}

impl TxId {
    /// Creates a new `TxId` from a 32-byte array.
    ///
    /// This is the primary constructor for `TxId` when you have the raw
    /// transaction hash available.
    ///
    /// # Examples
    /// ```
    /// # use zewif::TxId;
    /// // Usually this would be a real transaction hash
    /// let bytes = [0u8; 32];
    /// let txid = TxId::from_bytes(bytes);
    /// ```
    pub fn from_bytes(bytes: [u8; 32]) -> Self {
        Self::new(bytes)
    }

    /// Reads a `TxId` from any source implementing the `Read` trait.
    ///
    /// # Examples
    /// ```
    /// # use zewif::TxId;
    /// # use std::io::Cursor;
    /// # fn example() -> std::io::Result<()> {
    /// let bytes = [0u8; 32];
    /// let mut cursor = Cursor::new(bytes);
    /// let txid = TxId::read(&mut cursor)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn read<R: Read>(mut reader: R) -> io::Result<Self> {
        let mut hash = [0u8; 32];
        reader.read_exact(&mut hash)?;
        Ok(TxId::from_bytes(hash))
    }

    /// Writes a `TxId` to any destination implementing the `Write` trait.
    ///
    /// # Examples
    /// ```
    /// # use zewif::TxId;
    /// # fn example() -> std::io::Result<()> {
    /// let txid = TxId::from_bytes([0u8; 32]);
    /// let mut buffer = Vec::new();
    /// txid.write(&mut buffer)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn write<W: Write>(&self, mut writer: W) -> io::Result<()> {
        writer.write_all(self.as_slice())
    }
}

#[cfg(test)]
mod tests {
    use super::TxId;
    use crate::test_cbor_roundtrip;

    test_cbor_roundtrip!(TxId);

    #[test]
    fn display_is_byte_reversed() {
        let mut bytes = [0u8; 32];
        bytes[0] = 1;
        let txid = TxId::from_bytes(bytes);
        assert_eq!(
            txid.to_string(),
            "0000000000000000000000000000000000000000000000000000000000000001"
        );
        assert_eq!(TxId::from_hex(&txid.to_string()).unwrap(), txid);
    }
}
