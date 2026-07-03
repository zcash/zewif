use minicbor::{Decode, Encode};

use super::{Account, Network};
use crate::{AddressBookEntry, Extensions};

/// A Zcash wallet: network context, accounts, and address book.
///
/// Seed material is not stored here: it lives in the document's secret
/// store, referenced by ZIP 32 seed fingerprint from each account's key
/// source.
#[derive(Debug, Clone, PartialEq, Encode, Decode)]
#[cbor(map)]
pub struct ZewifWallet {
    #[n(0)]
    network: Network,
    #[n(1)]
    accounts: Vec<Account>,
    /// User-facing metadata about addresses of interest to this wallet.
    #[n(2)]
    address_book: Vec<AddressBookEntry>,
    #[cbor(n(3), with = "crate::extensions_field", has_nil)]
    extensions: Extensions,
}

impl ZewifWallet {
    pub fn new(network: Network) -> Self {
        Self {
            network,
            accounts: Vec::new(),
            address_book: Vec::new(),
            extensions: Extensions::new(),
        }
    }

    pub fn network(&self) -> &Network {
        &self.network
    }

    pub fn accounts(&self) -> &Vec<Account> {
        &self.accounts
    }

    pub fn add_account(&mut self, account: Account) {
        self.accounts.push(account);
    }

    pub fn address_book(&self) -> &[AddressBookEntry] {
        &self.address_book
    }

    pub fn add_address_book_entry(&mut self, entry: AddressBookEntry) {
        self.address_book.push(entry);
    }

    pub fn extensions(&self) -> &Extensions {
        &self.extensions
    }

    pub fn extensions_mut(&mut self) -> &mut Extensions {
        &mut self.extensions
    }
}

#[cfg(test)]
mod tests {
    use crate::{Extensions, Network, test_cbor_roundtrip};

    use super::ZewifWallet;

    impl crate::RandomInstance for ZewifWallet {
        fn random() -> Self {
            Self {
                network: Network::random(),
                accounts: Vec::random(),
                address_book: Vec::random(),
                extensions: Extensions::random(),
            }
        }
    }

    test_cbor_roundtrip!(ZewifWallet);
}
