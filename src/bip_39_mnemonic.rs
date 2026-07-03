use minicbor::{Decode, Encode};

use crate::{MnemonicLanguage, NoQuotesDebugOption};

/// A BIP-39 mnemonic phrase and, optionally, the language of its wordlist.
///
/// The seed's ZIP 32 fingerprint is not stored here: the seed entry in the
/// secret store carries it, and it is derivable from the mnemonic.
#[derive(Clone, PartialEq, Encode, Decode)]
#[cbor(map)]
pub struct Bip39Mnemonic {
    #[n(0)]
    mnemonic: String,
    #[n(1)]
    language: Option<MnemonicLanguage>,
}

impl std::fmt::Debug for Bip39Mnemonic {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Bip39Mnemonic")
            .field("language", &NoQuotesDebugOption(&self.language))
            .field("mnemonic", &"<elided>".to_string())
            .finish()
    }
}

impl Bip39Mnemonic {
    pub fn new(mnemonic: impl AsRef<str>, language: Option<MnemonicLanguage>) -> Self {
        Self {
            mnemonic: mnemonic.as_ref().to_string(),
            language,
        }
    }

    pub fn mnemonic(&self) -> &String {
        &self.mnemonic
    }

    pub fn set_mnemonic(&mut self, mnemonic: String) {
        self.mnemonic = mnemonic;
    }

    pub fn language(&self) -> Option<&MnemonicLanguage> {
        self.language.as_ref()
    }

    pub fn set_language(&mut self, language: MnemonicLanguage) {
        self.language = Some(language);
    }
}

#[cfg(test)]
mod tests {
    use crate::{MnemonicLanguage, test_cbor_roundtrip};

    use super::Bip39Mnemonic;

    impl crate::RandomInstance for Bip39Mnemonic {
        fn random() -> Self {
            Self {
                mnemonic: String::random(),
                language: MnemonicLanguage::opt_random(),
            }
        }
    }

    test_cbor_roundtrip!(Bip39Mnemonic);
}
