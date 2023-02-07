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

#[openbrush::trait_definition]
pub trait NFTMarketplace {

    #[ink(message)]
    fn get_fee_recipient(&self) -> AccountId;

    #[ink(message)]
    fn get_marketplace_fee(&self) -> u16;

}
