/// Produces randomized instances of a type for round-trip serialization
/// testing.
pub trait RandomInstance {
    fn random() -> Self;

    fn opt_random() -> Option<Self>
    where
        Self: Sized,
    {
        rand::random::<bool>().then(Self::random)
    }

    fn random_with_size(_size: usize) -> Self
    where
        Self: Sized,
    {
        panic!("RandomInstance::random_with_size is not implemented for this type");
    }

    fn opt_random_with_size(size: usize) -> Option<Self>
    where
        Self: Sized,
    {
        rand::random::<bool>().then(|| Self::random_with_size(size))
    }
}

impl RandomInstance for u8 {
    fn random() -> Self {
        rand::random()
    }
}

impl RandomInstance for u32 {
    fn random() -> Self {
        rand::random()
    }
}

impl RandomInstance for usize {
    fn random() -> Self {
        rand::random::<u64>() as usize
    }
}

impl RandomInstance for String {
    fn random() -> Self {
        use rand::Rng;
        let mut rng = rand::rng();
        let len = rng.random_range(10..=100);
        let alphabet = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
        (0..len)
            .map(|_| {
                alphabet
                    .chars()
                    .nth(rng.random_range(0..alphabet.len()))
                    .unwrap()
            })
            .collect()
    }
}

impl<T> RandomInstance for Vec<T>
where
    T: RandomInstance,
{
    fn random() -> Self {
        use rand::Rng;
        let mut rng = rand::rng();
        let len = rng.random_range(1..=5);
        (0..len).map(|_| T::random()).collect()
    }
}

/// Asserts that random instances of `T` survive a CBOR round trip, and that
/// re-encoding the decoded value reproduces the original bytes exactly
/// (round-trip determinism).
pub fn test_cbor_roundtrip<T>(iterations: usize)
where
    T: RandomInstance
        + minicbor::Encode<()>
        + for<'b> minicbor::Decode<'b, ()>
        + std::fmt::Debug
        + PartialEq,
{
    for _ in 0..iterations {
        let original = T::random();
        let encoded = minicbor::to_vec(&original).unwrap();
        let decoded: T = minicbor::decode(&encoded).unwrap();
        assert_eq!(original, decoded);
        let reencoded = minicbor::to_vec(&decoded).unwrap();
        assert_eq!(
            encoded, reencoded,
            "re-encoding must reproduce identical bytes"
        );
    }
}
