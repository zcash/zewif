/// Generates a test that round-trips 20 random instances of a type through
/// its CBOR encoding, asserting value equality and byte-exact re-encoding.
#[macro_export]
macro_rules! test_cbor_roundtrip {
    // Only type parameter - test function named `test_cbor`
    ($type:ty) => {
        #[test]
        fn test_cbor() {
            $crate::test_cbor_roundtrip::<$type>(20);
        }
    };

    // Type and custom test function name
    ($type:ty, $name:ident) => {
        #[test]
        fn $name() {
            $crate::test_cbor_roundtrip::<$type>(20);
        }
    };
}
