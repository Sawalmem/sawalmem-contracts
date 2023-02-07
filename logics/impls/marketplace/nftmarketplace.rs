use crate::{
    impls::marketplace::types::{
        Data,
        AuctionItem,
        MarketplaceError,
    },
    traits::marketplace::NFTMarketplace,
    traits::custom_mint::TokenRef,
};
use openbrush::{
    contracts::{
        ownable::*,
        psp34::*,
        reentrancy_guard::*,
    },
    modifiers,
    traits::{
        AccountId,
        Balance,
        Storage,
        String,
        Timestamp,
    },
};

pub trait Internal {
    fn calculate_next_minimum_bid(&self, address: AccountId, token_id: Id);

    fn get_next_minimum_bid(&self, address: AccountId, token_id: Id) -> Balance;

    fn update_highest_bid(&self, address: AccountId, token_id: Id, new_bidder: AccountId, new_bid_amount: Balance) -> Result<(),MarketplaceError>;
}

impl<T> NFTMarketplace for T
where
    T: Storage<Data> + Storage<ownable::Data> + Storage<reentrancy_guard::Data>,
{

    default fn get_fee_recipient(&self) -> AccountId {
        self.data::<Data>().market_fee_recipient
    }

    default fn get_marketplace_fee(&self) -> u16 {
        self.data::<Data>().fee
    }
}

impl<T> Internal for T
where
    T: Storage<Data>,
{
    default fn calculate_next_minimum_bid(&self, address: AccountId, token_id: Id) {
        let mut item = self.data::<Data>().items.get(&(address, token_id)).unwrap();
        let mut next_min_bid = item.highest_bid;
        next_min_bid += self.data::<Data>().bid_inc_percent * next_min_bid / 10000;
        item.next_min_bid = next_min_bid;
    }

    default fn get_next_minimum_bid(&self, address: AccountId, token_id: Id) -> Balance {
        self.data::<Data>().items.get(&(address, token_id)).unwrap().next_min_bid
    }

    default fn update_highest_bid(&self, address: AccountId, token_id: Id, new_bidder: AccountId, new_bid_amount: Balance)
    -> Result<(),MarketplaceError> {
        let mut item = self.data::<Data>().items.get(&(address, token_id)).unwrap();
        let prev_bidder = item.highest_bidder;
        let prev_bid = item.highest_bid;

        Self::env().transfer(prev_bidder,prev_bid).map_err(|_| MarketplaceError::TransferToBidderFailed)?;

        item.highest_bid = new_bid_amount;
        item.highest_bidder = new_bidder;
        Ok(())
    }
}