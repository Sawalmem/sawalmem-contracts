### Contracts

1. Token : Token contract expands PSP34 (Openbrush's ERC721 equivalent). It overrides PSP34's methods

```
Constructor : #[ink(constructor)] pub fn new(name: String,symbol: String,base_uri: String) -> Self 
```

```
Mint : fn mint(&mut self, to: AccountId, token_uri: String, marketplace: AccountId) -> Result<(), PSP34Error>;
```

Other methods

```
    #[ink(message)]
    fn set_base_uri(&mut self, uri: PreludeString) -> Result<(), PSP34Error>;
    #[ink(message)]
    fn get_token_uri(&mut self, token_id: u64) -> Result<PreludeString, PSP34Error>;

    
```

_Deployed contracts :_

```
TOKEN_ADDRESS_SHIBUYA = "Xc5P6n26FkToLvLzJJQJcewa3doT2RctHMkGsoXCo2hCJbx"
TOKEN_ADDRESS_ROCOCO = "5CVdCzwvWU4wPELHBXWoFjBzbkC2eEsKDYAnP4sc6G4diJuh"
```

2. Marketplace

_Deployed contracts :_

```
MARKETPLACE_ADDRESS_SHIBUYA = "bVL23KRxB9U5kNcNWgvAJwfHT9SGogDLJRrCKYcTmbmpurL";
MARKETPLACE_ADDRESS_ROCOCO = "5GpBMe1vfMVqRXBFt4kZkCJvKFrZ5Qr9ZFwZkkDbCBo825GY";
```

```
Constructor pub fn new(market_fee_recipient: AccountId) -> Self 
```

```

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

    
 ```
