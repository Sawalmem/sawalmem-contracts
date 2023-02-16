use ink::primitives::Hash;
use ink::prelude::vec::Vec;
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
        ZERO_ADDRESS,
    },
};
use scale::{
    Decode,
    Encode,
};

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[derive(Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct Data {
    pub items: Mapping<(AccountId, Id), AuctionItem>,
    pub collections: Mapping<AccountId, Collection>,
    pub fee: u16,
    pub market_fee_recipient: AccountId,
    pub bid_inc_percent: u128,
    pub contract_hash: Hash,
    pub collection_count: u64,
    pub item_count: u64,
    pub market_items: Vec<(AccountId,Id)>,
}

impl Default for Data {
    fn default() -> Self {
        Self {
            items: Default::default(),
            collections: Default::default(),
            fee: 0,
            market_fee_recipient: ZERO_ADDRESS.into(),
            bid_inc_percent: 0,
            contract_hash: Default::default(),
            collection_count: 0,
            item_count: 0,
            market_items: Default::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum MarketplaceError {
    /// Caller is not a marketplace owner.
    OwnableError(OwnableError),
    /// Caller is tryin to make second call while 1st one is still executing.
    ReentrancyError(ReentrancyGuardError),
    // Token Does Not Exist
    TokenDoesNotExist,
    // Token AlreadyExists
    TokenAlreadyExists,
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
    // Token Only For Direct Sale
    TokenOnlyForDirectSale,
    // Incorrect Buy Price
    IncorrectBuyPrice,
    // Transfer To Bidder Failed
    TransferToBidderFailed,
    // Transfer To Owner Failed
    TransferToOwnerFailed,
    // Transfer To Contract Failed
    TransferToContractFailed,
    // Marketplace Fee Transfer Failed
    MarketplaceFeeTransferFailed,
    // Royalties Transfer Failed
    RoyaltiesTransferFailed,
    // Contract Hash Not Set
    ContractHashNotSet,
    // TokenInstantiationFailed
    TokenInstantiationFailed,
    // Minimum Bid Already Met
    MinimumBidAlreadyMet,
    // Collection Already Exists
    CollectionAlreadyExists,
    // Collection Not Registered To Marketplace
    CollectionNotRegisteredToMarketplace,
    // Approval Failed
    ApprovalFailed,
    // Bid Not Update
    BidNotUpdated,
}

#[derive(Encode, Decode, Debug)]
#[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
)]
pub struct AuctionItem {
    pub owner: AccountId,
    pub buy_price: Balance,
    pub seller: Option<AccountId>,
    pub highest_bid: Balance,
    pub highest_bidder: Option<AccountId>,
    pub min_bid: Balance,
    pub next_min_bid: Balance,
    pub bid_end_time: Timestamp,
    pub on_sale: bool,
    pub direct: bool,
}

impl Default for AuctionItem {
    fn default() -> Self {
        Self {
            owner: ZERO_ADDRESS.into(),
            buy_price: 0,
            seller: None,
            highest_bid: 0,
            highest_bidder: None,
            min_bid: 0,
            next_min_bid: 0,
            bid_end_time: 0,
            on_sale: false,
            direct: false,
        }
    }
}

#[derive(Encode, Decode, Debug)]
#[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
)]
pub struct Collection {
    pub name: String,
    pub symbol: String,
    pub ipfs: String,
    pub creator: Option<AccountId>,
    pub royalty: u16,
}

impl Default for Collection {
    fn default() -> Self {
        Self {
            name: Default::default(),
            symbol: Default::default(),
            ipfs: Default::default(),
            creator: None,
            royalty: 0,
        }
    }
}

impl From<OwnableError> for MarketplaceError {
    fn from(error: OwnableError) -> Self {
        MarketplaceError::OwnableError(error)
    }
}

impl From<ReentrancyGuardError> for MarketplaceError {
    fn from(error: ReentrancyGuardError) -> Self {
        MarketplaceError::ReentrancyError(error)
    }
}
