### Contracts

1. Token : Token contract expands PSP34 (Openbrush's ERC721 equivalent). It overrides PSP34's methods

```
Constructor : #[ink(constructor)] pub fn new(name: String,symbol: String,base_uri: String) -> Self 
```

```
Mint : fn mint(&mut self, to: AccountId, token_uri: String, royalty: u16) -> Result<(), PSP34Error>;
```

Other methods

```
    #[ink(message)]
    fn set_base_uri(&mut self, uri: PreludeString) -> Result<(), PSP34Error>;
    #[ink(message)]
    fn get_token_uri(&mut self, token_id: u64) -> Result<PreludeString, PSP34Error>;
    #[ink(message)]
    fn get_token_royalty(&mut self, token_id: u64) -> Result<u16, PSP34Error>;
    #[ink(message)]
    fn get_royalty_info(&mut self, token_id: u64) -> Result<(u16,AccountId),PSP34Error>;
    
```


2. Marketplace

```
Constructor pub fn new(market_fee_recipient: AccountId) -> Self 
```

```
    #[ink(message)]
    fn create_market_item(&mut self,address: AccountId, token_id: Id)  -> Result<(), MarketplaceError>;

    #[ink(message)]
    fn create_direct_sale(&mut self,address: AccountId, token_id: Id, price: Balance) -> Result<(), MarketplaceError>;

    #[ink(message)]
    fn create_auction(&mut self,address: AccountId, token_id: Id, price: Balance, min_bid: Balance, duration: Timestamp) -> Result<(), MarketplaceError>;

    #[ink(message)]
    fn get_fee_recipient(&self) -> AccountId;

    #[ink(message)]
    fn get_marketplace_fee(&self) -> u16;
    
 ```
