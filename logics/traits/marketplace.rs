use crate::impls::marketplace::types::{
    MarketplaceError,
};

use openbrush::{
    contracts::psp34::Id,
    traits::{
        AccountId,
        Balance,
        String,
        Timestamp,
    },
};

#[openbrush::wrapper]
pub type MarketRef = dyn NFTMarketplace;

#[openbrush::trait_definition]
pub trait NFTMarketplace {

    #[ink(message)]
    fn create_market_item(&mut self,address: AccountId, token_id: Id)  -> Result<(), MarketplaceError>;

    #[ink(message)]
    fn create_direct_sale(&mut self,address: AccountId, token_id: Id, price: Balance) -> Result<(), MarketplaceError>;

    #[ink(message)]
    fn get_fee_recipient(&self) -> AccountId;

    #[ink(message)]
    fn get_marketplace_fee(&self) -> u16;

}
