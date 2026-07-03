use crate::error::{Error, Result};
use std::{
    fmt,
    io::{self, Read, Write},
};

/// A 32-byte transaction identifier, displayed in reverse byte order by convention.
#[derive(Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct TxId([u8; 32]);

impl fmt::Debug for TxId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TxId({})", self)
    }
}

impl fmt::Display for TxId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // The (byte-flipped) hex string is more useful than the raw bytes, because we can
        // look that up in RPC methods and block explorers.
        let mut data = self.0;
        data.reverse();
        f.write_str(&hex::encode(data))
    }
}

impl AsRef<[u8; 32]> for TxId {
    fn as_ref(&self) -> &[u8; 32] {
        &self.0
    }
}

impl From<TxId> for [u8; 32] {
    fn from(value: TxId) -> Self {
        value.0
    }
}

impl TxId {
    /// Creates a new `TxId` from a 32-byte array.
    ///
    /// This is the primary constructor for `TxId` when you have the raw transaction
    /// hash available.
    ///
    /// # Examples
    /// ```
    /// # use zewif::TxId;
    /// // Usually this would be a real transaction hash
    /// let bytes = [0u8; 32];
    /// let txid = TxId::from_bytes(bytes);
    /// ```
    pub fn from_bytes(bytes: [u8; 32]) -> Self {
        TxId(bytes)
    }

    /// Parses a `TxId` from a canonically-encoded (byte-reversed) hexadecimal string.
    ///
    /// # Examples
    /// ```
    /// # use zewif::TxId;
    ///
    /// let hex = "0000000000000000000000000000000000000000000000000000000000000001";
    /// let blob = TxId::from_hex(hex).unwrap();
    /// let mut expected = [0u8; 32];
    /// expected[0] = 1;
    /// assert_eq!(blob.as_ref(), &expected);
    /// ```
    pub fn from_hex(hex: &str) -> Result<Self> {
        let mut data = hex::decode(hex)?;
        data.reverse();

        Ok(Self(<[u8; 32]>::try_from(&data[..]).map_err(|_| {
            Error::HexLengthMismatch {
                expected: 32,
                actual: data.len(),
            }
        })?))
    }

    /// Reads a `TxId` from any source implementing the `Read` trait.
    ///
    /// This method is useful when reading transaction IDs directly from files
    /// or other byte streams.
    ///
    /// # Errors
    /// Returns an IO error if reading fails or if there aren't enough bytes available.
    ///
    /// # Examples
    /// ```no_run
    /// # use std::io::Cursor;
    /// # use zewif::TxId;
    /// #
    /// # fn example() -> std::io::Result<()> {
    /// // Create a cursor with 32 bytes
    /// let data = vec![0u8; 32];
    /// let mut cursor = Cursor::new(data);
    ///
    /// // Read a TxId from the cursor
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
    /// This method is useful when serializing transaction IDs to files or
    /// other byte streams.
    ///
    /// # Errors
    /// Returns an IO error if writing fails.
    ///
    /// # Examples
    /// ```no_run
    /// # use std::io::Cursor;
    /// # use zewif::TxId;
    /// #
    /// # fn example() -> std::io::Result<()> {
    /// let txid = TxId::from_bytes([0u8; 32]);
    /// let mut buffer = Vec::new();
    ///
    /// // Write the TxId to the buffer
    /// txid.write(&mut buffer)?;
    ///
    /// // The buffer now contains the 32-byte transaction ID
    /// assert_eq!(buffer.len(), 32);
    /// # Ok(())
    /// # }
    /// ```
    pub fn write<W: Write>(&self, mut writer: W) -> io::Result<()> {
        writer.write_all(&self.0)?;
        Ok(())
    }
}

impl<C> minicbor::Encode<C> for TxId {
    fn encode<W: minicbor::encode::Write>(
        &self,
        e: &mut minicbor::Encoder<W>,
        _ctx: &mut C,
    ) -> std::result::Result<(), minicbor::encode::Error<W::Error>> {
        e.bytes(&self.0)?;
        Ok(())
    }
}

impl<'b, C> minicbor::Decode<'b, C> for TxId {
    fn decode(
        d: &mut minicbor::Decoder<'b>,
        _ctx: &mut C,
    ) -> std::result::Result<Self, minicbor::decode::Error> {
        let bytes = d.bytes()?;
        let hash = <[u8; 32]>::try_from(bytes).map_err(|_| {
            minicbor::decode::Error::message("expected a byte string of length 32 for TxId")
        })?;
        Ok(TxId::from_bytes(hash))
    }
}

#[cfg(test)]
mod tests {
    use crate::test_cbor_roundtrip;

    use super::TxId;

    impl crate::RandomInstance for TxId {
        fn random() -> Self {
            Self(rand::random())
        }
    }

    test_cbor_roundtrip!(TxId);
}
