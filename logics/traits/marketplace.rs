use crate::impls::marketplace::types::{
    MarketplaceError,
};
use ink_env::Hash;

use openbrush::{
    contracts::psp34::Id,
    traits::{
        AccountId,
        Balance,
        String,
        Timestamp,
    },
};

#[openbrush::trait_definition]
pub trait NFTMarketplace {
    #[ink(message)]
    fn create_collection(&mut self, name: String, symbol: String, collection_hash: String, royalty: u16 ) -> Result<AccountId, MarketplaceError>;

    #[ink(message)]
    fn add_collection(&mut self, address: AccountId, name: String, symbol: String, collection_hash: String, royalty: u16 ) -> Result<(), MarketplaceError>;

    #[ink(message)]
    fn set_contract_hash(&mut self,contract_hash: Hash) -> Result<(), MarketplaceError>;

    #[ink(message)]
    fn get_contract_hash(&self) -> Hash;

    #[ink(message)]
    fn create_market_item(&mut self,address: AccountId, token_id: Id)  -> Result<(), MarketplaceError>;

    #[ink(message)]
    fn create_direct_sale(&mut self,address: AccountId, token_id: Id, price: Balance) -> Result<(), MarketplaceError>;

    #[ink(message)]
    fn create_auction(&mut self,address: AccountId, token_id: Id, price: Balance, min_bid: Balance, duration: Timestamp) -> Result<(), MarketplaceError>;

    #[ink(message,payable)]
    fn close_direct_sale(&mut self,address: AccountId, token_id: Id) -> Result<(), MarketplaceError>;

    #[ink(message)]
    fn withdraw_auction(&mut self,address: AccountId, token_id: Id) -> Result<(), MarketplaceError>;

    #[ink(message,payable)]
    fn make_bid(&mut self,address: AccountId, token_id: Id) -> Result<(), MarketplaceError>;

    #[ink(message)]
    fn settle_auction(&mut self,address: AccountId, token_id: Id) -> Result<(), MarketplaceError>;

    #[ink(message)]
    fn get_fee_recipient(&self) -> AccountId;

    #[ink(message)]
    fn set_marketplace_fee(&mut self, fee: u16) -> Result<(), MarketplaceError>;

    #[ink(message)]
    fn get_marketplace_fee(&self) -> u16;

}
