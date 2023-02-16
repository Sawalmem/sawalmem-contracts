use crate::impls::marketplace::types::{
    MarketplaceError,
    Collection,
    AuctionItem,
};
use ink::primitives::Hash;
use ink::prelude::vec::Vec;
use openbrush::{
    contracts::psp34::Id,
    traits::{
        AccountId,
        Balance,
        String,
        Timestamp,
        BlockNumber,
    },
};

#[openbrush::trait_definition]
pub trait NFTMarketplace {
    //#[ink(message)]
    //fn create_collection(&mut self, name: String, symbol: String, collection_hash: String, royalty: u16 ) -> Result<AccountId, MarketplaceError>;

    #[ink(message)]
    fn add_collection(&mut self, address: AccountId, name: String, symbol: String, collection_hash: String, royalty: u16 ) -> Result<(), MarketplaceError>;

    #[ink(message)]
    fn get_collection(&self, address: AccountId) -> Option<Collection>;

    #[ink(message)]
    fn get_item(&self, address: AccountId,token_id: Id) -> Option<AuctionItem>;

    #[ink(message)]
    fn get_collection_count(&self) -> u64;

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

    #[ink(message)]
    fn get_item_count(&self) -> u64;

    #[ink(message)]
    fn get_all_market_items(&self) -> Vec<(AccountId,Id)>;

    #[ink(message)]
    fn get_timestamp(&self) ->Timestamp;

    #[ink(message)]
    fn get_blocknumber(&self) -> BlockNumber;

}
