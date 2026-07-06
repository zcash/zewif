/// Creates a new type wrapping the canonical text encoding of a key.
///
/// Shareable Zcash keys have protocol-defined string encodings (Bech32,
/// Bech32m, or Base58Check) that carry a checksum and embed type and
/// network discrimination; ZeWIF stores those strings verbatim. The
/// generated type is a transparent newtype over `String` on the wire.
///
/// The third argument is the prefix used to synthesize plausible values in
/// randomized tests. A fourth `redacted` argument marks secret material:
/// no `Display` is generated and `Debug` output elides the content.
#[macro_export]
macro_rules! text_key {
    ($name:ident, $doc:expr, $test_prefix:literal) => {
        $crate::text_key!(@common $name, $doc, $test_prefix);

        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}({})", stringify!($name), self.0)
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.write_str(&self.0)
            }
        }
    };

    ($name:ident, $doc:expr, $test_prefix:literal, redacted) => {
        $crate::text_key!(@common $name, $doc, $test_prefix);

        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}(<elided>)", stringify!($name))
            }
        }
    };

    (@common $name:ident, $doc:expr, $test_prefix:literal) => {
        #[doc = $doc]
        #[derive(Clone, PartialEq, Eq, Hash, minicbor::Encode, minicbor::Decode)]
        #[cbor(transparent)]
        pub struct $name(#[n(0)] String);

        impl $name {
            pub fn new(encoding: impl Into<String>) -> Self {
                Self(encoding.into())
            }

            /// The canonical string encoding of this key.
            pub fn encoding(&self) -> &str {
                &self.0
            }
        }

        impl From<String> for $name {
            fn from(encoding: String) -> Self {
                Self(encoding)
            }
        }

        #[cfg(test)]
        impl $crate::RandomInstance for $name {
            fn random() -> Self {
                Self(format!("{}{}", $test_prefix, String::random()))
            }
        }
    };
}
