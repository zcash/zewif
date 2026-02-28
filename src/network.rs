use crate::error::Error;
use bc_envelope::prelude::*;

/// The Zcash network a wallet belongs to: mainnet, testnet, or regtest.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Network {
    Main,
    Test,
    Regtest,
}

impl From<Network> for String {
    fn from(value: Network) -> String {
        match value {
            Network::Main => "main".to_string(),
            Network::Test => "test".to_string(),
            Network::Regtest => "regtest".to_string(),
        }
    }
}

impl TryFrom<String> for Network {
    type Error = Error;

    fn try_from(value: String) -> crate::error::Result<Self> {
        if value == "main" {
            Ok(Network::Main)
        } else if value == "test" {
            Ok(Network::Test)
        } else if value == "regtest" {
            Ok(Network::Regtest)
        } else {
            Err(Error::InvalidNetwork(value))
        }
    }
}

impl From<Network> for CBOR {
    fn from(value: Network) -> Self {
        String::from(value).into()
    }
}

impl TryFrom<CBOR> for Network {
    type Error = dcbor::Error;

    fn try_from(cbor: CBOR) -> dcbor::Result<Self> {
        Ok(cbor.try_into_text()?.try_into()?)
    }
}

impl From<Network> for Envelope {
    fn from(value: Network) -> Self {
        Envelope::new(String::from(value))
    }
}

impl TryFrom<Envelope> for Network {
    type Error = bc_envelope::Error;

    fn try_from(envelope: Envelope) -> bc_envelope::Result<Self> {
        let network_str: String = envelope.extract_subject()?;
        Network::try_from(network_str).map_err(|e| e.into())
    }
}

#[cfg(test)]
mod tests {
    use crate::{test_cbor_roundtrip, test_envelope_roundtrip};

    use super::Network;

    impl crate::RandomInstance for Network {
        fn random() -> Self {
            match rand::random::<u8>() % 3 {
                0 => Network::Main,
                1 => Network::Test,
                _ => Network::Regtest,
            }
        }
    }

    test_cbor_roundtrip!(Network);
    test_envelope_roundtrip!(Network);
}
