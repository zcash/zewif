/// Creates a new type wrapping a fixed-size byte array with common methods and
/// trait implementations.
///
/// The `blob!` macro generates a new type that wraps a fixed-size byte array
/// of the specified size, automatically implementing common methods and traits.
/// This provides a convenient way to create domain-specific types for
/// fixed-size binary data with minimal boilerplate.
///
/// # Usage
///
/// ```
/// # use zewif::blob;
/// #
/// blob!(ExampleHash, 32, "An example 32-byte hash type");
/// ```
///
/// Hex parsing and display are provided separately by the [`blob_hex!`]
/// macro, which every `blob!` type pairs with to declare its canonical hex
/// convention (`forward` byte order, or `reversed` for the hash-display
/// convention used by transaction identifiers and block hashes).
///
/// # Generated Functionality
///
/// The generated type includes methods for creation, conversion, and
/// inspection, hex parsing and formatting, ordering/equality/hash traits,
/// byte-collection conversions, and the ZeWIF CBOR codec (a byte string of
/// exactly the declared length).
#[macro_export]
macro_rules! blob {
    ($name:ident, $size:expr, $doc:expr) => {
        #[doc = $doc]
        #[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name([u8; $size]);

        impl $name {
            /// The length of this type's byte content.
            pub const SIZE: usize = $size;

            /// Creates a new instance from a fixed-size byte array.
            ///
            /// This is the primary constructor when you have an exact-sized
            /// array available.
            pub fn new(data: [u8; $size]) -> Self {
                Self(data)
            }

            /// Returns the length of this blob in bytes.
            ///
            /// This will always return `$size` for this type.
            pub fn len(&self) -> usize {
                $size
            }

            /// Returns `true` if this blob contains no bytes.
            ///
            /// This will always return `false` for this type (unless `$size` is
            /// 0).
            pub fn is_empty(&self) -> bool {
                $size != 0
            }

            /// Converts this blob to a `Vec<u8>`, creating a copy of the data.
            pub fn to_vec(&self) -> Vec<u8> {
                self.0.to_vec()
            }

            /// Exposes the underlying byte array as a slice.
            pub fn as_slice(&self) -> &[u8] {
                &self.0
            }

            /// Exposes the underlying byte array.
            pub fn as_bytes(&self) -> &[u8; $size] {
                &self.0
            }

            /// Creates an instance from a slice of bytes.
            ///
            /// # Errors
            /// Returns an error if the slice's length doesn't match the
            /// expected size ($size).
            pub fn from_slice(data: &[u8]) -> Result<Self, std::array::TryFromSliceError> {
                Ok(Self(<[u8; $size]>::try_from(data)?))
            }

            /// Creates an instance from a `Vec<u8>`.
            ///
            /// # Errors
            /// Returns an error if the vector's length doesn't match the
            /// expected size ($size).
            pub fn from_vec(data: Vec<u8>) -> Result<Self, std::array::TryFromSliceError> {
                Ok(Self(<[u8; $size]>::try_from(&data[..])?))
            }
        }

        impl AsRef<[u8]> for $name {
            fn as_ref(&self) -> &[u8] {
                &self.0[..]
            }
        }

        impl AsRef<[u8; $size]> for $name {
            fn as_ref(&self) -> &[u8; $size] {
                &self.0
            }
        }

        impl From<$name> for [u8; $size] {
            fn from(blob: $name) -> [u8; $size] {
                blob.0
            }
        }

        impl From<$name> for Vec<u8> {
            fn from(blob: $name) -> Vec<u8> {
                blob.to_vec()
            }
        }

        impl From<&$name> for Vec<u8> {
            fn from(blob: &$name) -> Vec<u8> {
                blob.to_vec()
            }
        }

        impl From<Vec<u8>> for $name {
            fn from(data: Vec<u8>) -> Self {
                Self::from_vec(data).unwrap()
            }
        }

        impl From<&[u8]> for $name {
            fn from(data: &[u8]) -> Self {
                Self::from_slice(data).unwrap()
            }
        }

        impl<C> minicbor::Encode<C> for $name {
            fn encode<W: minicbor::encode::Write>(
                &self,
                e: &mut minicbor::Encoder<W>,
                _ctx: &mut C,
            ) -> Result<(), minicbor::encode::Error<W::Error>> {
                e.bytes(self.as_slice())?;
                Ok(())
            }
        }

        impl<'b, C> minicbor::Decode<'b, C> for $name {
            fn decode(
                d: &mut minicbor::Decoder<'b>,
                _ctx: &mut C,
            ) -> Result<Self, minicbor::decode::Error> {
                let bytes = d.bytes()?;
                Self::from_slice(bytes).map_err(|_| {
                    minicbor::decode::Error::message(concat!(
                        "expected a byte string of length ",
                        stringify!($size),
                        " for ",
                        stringify!($name)
                    ))
                })
            }
        }

        #[cfg(test)]
        impl $crate::RandomInstance for $name {
            fn random() -> Self {
                Self(rand::random())
            }
        }
    };
}

/// Declares the canonical hexadecimal convention for a [`blob!`] type,
/// generating its `Debug` implementation and hex parsing/formatting.
///
/// Every `blob!` type pairs with exactly one `blob_hex!` invocation:
///
/// - `forward`: `Debug`, `from_hex`, and `to_hex` use the bytes in their
///   stored order. This is the convention for nullifiers, Merkle hashes,
///   key material, and fingerprints.
/// - `reversed`: `Display`, `Debug`, `from_hex`, and `to_hex` use the
///   byte-reversed (big-endian) form that RPC methods and block explorers
///   display. Only transaction identifiers and block hashes are canonically
///   encoded in this fashion.
#[macro_export]
macro_rules! blob_hex {
    ($name:ident, forward) => {
        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}({})", stringify!($name), self.to_hex())
            }
        }

        impl $name {
            /// Parses an instance from a hex string.
            pub fn from_hex(hex: &str) -> $crate::Result<Self> {
                let data = hex::decode(hex)?;
                let data_len = data.len();
                Self::from_vec(data).map_err(|_| $crate::Error::HexLengthMismatch {
                    expected: Self::SIZE,
                    actual: data_len,
                })
            }

            /// Formats the bytes of this object as a hex string.
            pub fn to_hex(&self) -> String {
                hex::encode(self.as_slice())
            }
        }
    };

    ($name:ident, reversed) => {
        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}({})", stringify!($name), self)
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                // The byte-reversed hex string is what RPC methods and block
                // explorers display.
                let mut data = *self.as_bytes();
                data.reverse();
                f.write_str(&hex::encode(data))
            }
        }

        impl $name {
            /// Parses an instance from its canonically-displayed
            /// (byte-reversed) hexadecimal form.
            pub fn from_hex(hex: &str) -> $crate::Result<Self> {
                let mut data = hex::decode(hex)?;
                let data_len = data.len();
                data.reverse();
                Self::from_vec(data).map_err(|_| $crate::Error::HexLengthMismatch {
                    expected: Self::SIZE,
                    actual: data_len,
                })
            }

            /// Formats this value in its canonically-displayed
            /// (byte-reversed) hexadecimal form.
            pub fn to_hex(&self) -> String {
                self.to_string()
            }
        }
    };
}
