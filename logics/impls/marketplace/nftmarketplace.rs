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

    fn check_token_exists(&self, address: AccountId, token_id: Id) -> bool;

    fn get_sales_breakdown(&self, address: AccountId, token_id: Id, sales_price: Balance) -> Result<(),MarketplaceError>;

    fn set_auction_end(&mut self, address: AccountId, token_id: Id) -> Result<(),MarketplaceError>;
}

impl<T> NFTMarketplace for T
where
    T: Storage<Data> + Storage<ownable::Data> + Storage<reentrancy_guard::Data>,
{
    default fn create_market_item(&mut self,address: AccountId, token_id: Id) -> Result<(), MarketplaceError> {
        if self.check_token_exists(address,token_id.clone()) {
            return Err(MarketplaceError::TokenAlreadyExists)
        }

        self.data::<Data>().items.insert(&(address, token_id),
        &AuctionItem{
            owner: Self::env().caller(),
            buy_price: 0,
            seller: None,
            highest_bid: 0,
            highest_bidder: None,
            min_bid: 0,
            next_min_bid: 0,
            bid_end_time: 0,
            royalties: 0,
            on_sale: false,
            direct: false,
        });
        Ok(())
    }

    default fn create_direct_sale(&mut self,address: AccountId, token_id: Id, price: Balance) -> Result<(), MarketplaceError> {
        let mut item = self.data::<Data>().items.get(&(address, token_id.clone())).unwrap();
        if item.owner != Self::env().caller() {
            return Err(MarketplaceError::NotTheOwner)
        }
        if item.on_sale == true {
            return Err(MarketplaceError::TokenAlreadyOnSale)
        }
        if price == 0 {
            return Err(MarketplaceError::IneligibleBuyPrice)
        }
        // Register NFT contract to marketplace and enable approval to all
        let this = Self::env().account_id();

        match PSP34Ref::transfer(&address,this,token_id.clone(),ink_prelude::vec::Vec::new()) {
            Ok(()) => {
                item.buy_price = price;
                item.seller = Some(Self::env().caller());
                item.on_sale = true;
                item.direct = true;
            },
            Err(_) => return Err(MarketplaceError::TransferToContractFailed)
        }
        Ok(())
    }

    default fn create_auction(&mut self,address: AccountId, token_id: Id, price: Balance, min_bid: Balance, duration: Timestamp) -> Result<(), MarketplaceError> {
        let mut item = self.data::<Data>().items.get(&(address, token_id.clone())).unwrap();
        if item.owner != Self::env().caller() {
            return Err(MarketplaceError::NotTheOwner)
        }
        if item.on_sale == true {
            return Err(MarketplaceError::TokenAlreadyOnSale)
        }
        if price == 0 {
            return Err(MarketplaceError::IneligibleBuyPrice)
        }
        if duration == 0 {
            return Err(MarketplaceError::IneligibleBidDuration)
        }
        // Register NFT contract to marketplace and enable approval to all
        let this = Self::env().account_id();
        match PSP34Ref::transfer(&address,this,token_id.clone(),ink_prelude::vec::Vec::new()) {
            Ok(()) => {
                item.buy_price = price;
                item.seller = Some(Self::env().caller());
                item.on_sale = true;
                item.direct = false;
                item.min_bid = min_bid;
                item.bid_end_time = duration + Timestamp::default();
            },
            Err(_) => return Err(MarketplaceError::TransferToContractFailed)
        }
        Ok(())
    }

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
        let prev_bidder = item.highest_bidder.unwrap();
        let prev_bid = item.highest_bid;

        Self::env().transfer(prev_bidder,prev_bid).map_err(|_| MarketplaceError::TransferToBidderFailed)?;

        item.highest_bid = new_bid_amount;
        item.highest_bidder = Some(new_bidder);
        Ok(())
    }

    default fn check_token_exists(&self, address: AccountId, token_id: Id) -> bool {
        self.data::<Data>().items.get(&(address, token_id)).is_some()
    }

    default fn get_sales_breakdown(&self, address: AccountId, token_id: Id, sales_price: Balance) -> Result<(),MarketplaceError> {
        let market_fees: Balance = u128::from(self.data::<Data>().fee) * sales_price / 10000;
        
        Ok(())
    }

    default fn set_auction_end(&mut self, address: AccountId, token_id: Id) -> Result<(),MarketplaceError> {
        let token_owner = PSP34Ref::owner_of(&address.clone(), token_id.clone())
            .ok_or(MarketplaceError::TokenDoesNotExist)?;
        self.data::<Data>().items.insert(&(address, token_id),
        &AuctionItem{
            owner: token_owner,
            buy_price: 0,
            seller: None,
            highest_bid: 0,
            highest_bidder: None,
            min_bid: 0,
            next_min_bid: 0,
            bid_end_time: 0,
            royalties: 0,
            on_sale: false,
            direct: false,
        });

        Ok(())
    }
}