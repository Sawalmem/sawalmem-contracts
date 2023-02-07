use ink_storage::traits::{
    PackedLayout,
    SpreadLayout,
};
use openbrush::{
    contracts::{
        ownable::OwnableError,
        psp34::Id,
        reentrancy_guard::ReentrancyGuardError,
    },
    storage::Mapping,
    traits::{
        AccountId,
        Balance,
        String,
        Timestamp,
    },
};
use scale::{
    Decode,
    Encode,
};

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct Data {
    pub items: Mapping<(AccountId, Id), AuctionItem>,
    pub fee: u16,
    pub market_fee_recipient: AccountId,
    pub bid_inc_percent: u128,
}

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum MarketplaceError {
    //Token Already on Sale
    TokenAlreadyOnSale,
    //Not The Owner
    NotTheOwner,
    // Minimum Bid Not Met
    MinimumBidNotMet,
    // Ineligible Bid Duration 
    IneligibleBidDuration,
    // Ineligible Buy Price
    IneligibleBuyPrice,
    // Token Not For Sale
    TokenNotForSale,
    // Auction Expired
    AuctionExpired,
    // Auction Ongoing,
    AuctionOngoing,
    // No Valid Bids
    NoValidBids,
    // Not Authorized
    NotAuthorized,
    //Token Not For Direct Sale
    TokenNotForDirectSale,
    // Incorrect Buy Price
    IncorrectBuyPrice,
    // Transfer To Bidder Failed
    TransferToBidderFailed,
}

#[derive(Encode, Decode, SpreadLayout, PackedLayout, Default, Debug)]
#[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout)
)]
pub struct AuctionItem {
    pub owner: AccountId,
    pub buy_price: Balance,
    pub seller: AccountId,
    pub highest_bid: Balance,
    pub highest_bidder: AccountId,
    pub min_bid: Balance,
    pub next_min_bid: Balance,
    pub bid_end_time: Timestamp,
    pub royalties: u16,
    pub on_sale: bool,
}