#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod marketplace {
    use ink::env::DefaultEnvironment;
    use openbrush::contracts::ownable::*;
    use openbrush::contracts::psp34::Id;
    use openbrush::contracts::reentrancy_guard::*;
    use openbrush::traits::Storage;
    
    use marketplace_pkg::{
        impls::marketplace::*,
        traits::marketplace::*,
    };

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct MarketplaceContract {
        #[storage_field]
        ownable: ownable::Data,
        #[storage_field]
        guard: reentrancy_guard::Data,
        #[storage_field]
        marketplace: types::Data,
    }

    impl MarketplaceContract {
        #[ink(constructor)]
        pub fn new(market_fee_recipient: AccountId) -> Self {
            
                let mut instance = Self::default();
                instance.marketplace.fee = 100; // 1%
                instance.marketplace.market_fee_recipient = market_fee_recipient;

                let caller = instance.env().caller();
                instance._init_with_owner(caller);
                instance
        }
        
    }

    impl NFTMarketplace for MarketplaceContract {}

    #[cfg(test)]
    mod Tests {
        use super::*;
        use crate::marketplace::MarketplaceContract;
        use ink::env::test;
        use openbrush::{
            contracts::psp34::Id,
            traits::String,
        };
        use marketplace_pkg::impls::marketplace::types::MarketplaceError;

        #[ink::test]
        fn new_works() {
            let marketplace = init_contract();
            assert_eq!(marketplace.get_marketplace_fee(), 100);
            assert_eq!(marketplace.get_fee_recipient(), fee_recipient());
        }

        #[ink::test]
        fn set_marketplacefees_works() {
            let mut marketplace = init_contract();
            assert_eq!(marketplace.get_marketplace_fee(), 100);
            assert!(marketplace.set_marketplace_fee(200).is_ok());
            assert_eq!(marketplace.get_marketplace_fee(), 200);
        }

        #[ink::test]
        fn add_collection_works() {
            let mut marketplace = init_contract();
            //add_collection(&mut self, address: AccountId, name: String, symbol: String, collection_hash: String, royalty: u16 )

            assert_eq!(marketplace.get_collection_count(),0);

            let name = String::from("Test Collection");
            let symbol = String::from("TST");
            let hash = String::from("https://ipfs.io/aaa");
            let royalty: u16 = 150;

            assert!(marketplace.add_collection(contract_address(),name,symbol,hash,royalty).is_ok());

            assert_eq!(marketplace.get_collection_count(),1);
        }

        #[ink::test]
        fn create_market_item_works() {
            let mut marketplace = init_contract();

            let name = String::from("Test Collection");
            let symbol = String::from("TST");
            let hash = String::from("https://ipfs.io/aaa");
            let royalty: u16 = 150;

            assert!(marketplace.add_collection(contract_address(),name,symbol,hash,royalty).is_ok());
            assert!(marketplace.create_market_item(contract_address(),Id::U64(3)).is_ok());
            assert_eq!(marketplace.get_all_market_items(),vec![(contract_address(),Id::U64(3))]);
        }

        #[ink::test]
        fn create_get_multiple_market_item_works() {
            let mut marketplace = init_contract();

            let name = String::from("Test Collection");
            let symbol = String::from("TST");
            let hash = String::from("https://ipfs.io/aaa");
            let royalty: u16 = 150;

            assert!(marketplace.add_collection(contract_address(),name,symbol,hash,royalty).is_ok());
            assert!(marketplace.create_market_item(contract_address(),Id::U64(3)).is_ok());
            assert!(marketplace.create_market_item(contract_address(),Id::U64(4)).is_ok());
            assert_eq!(marketplace.get_all_market_items(),vec![(contract_address(),Id::U64(3)),(contract_address(),Id::U64(4))]);
        }

        #[ink::test]
        fn set_get_contract_hash_works() {
            let mut marketplace = init_contract();
            let hash = Hash::try_from([1; 32]).unwrap();

            assert!(marketplace.set_contract_hash(hash).is_ok());
            assert_eq!(marketplace.get_contract_hash(),hash);
        }

        #[ink::test]
        fn get_timestamp_works() {
            let mut marketplace = init_contract();

            let name = String::from("Test Collection");
            let symbol = String::from("TST");
            let hash = String::from("https://ipfs.io/aaa");
            let royalty: u16 = 150;

            assert!(marketplace.add_collection(contract_address(),name,symbol,hash,royalty).is_ok());
            assert_eq!(marketplace.get_timestamp(),0);
            assert_eq!(marketplace.get_blocknumber(),0);
        }

        fn init_contract() -> MarketplaceContract {
            MarketplaceContract::new(fee_recipient())
        }

        fn default_accounts() -> test::DefaultAccounts<ink::env::DefaultEnvironment> {
            test::default_accounts::<Environment>()
        }

        fn set_sender(sender: AccountId) {
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(sender);
        }

        fn fee_recipient() -> AccountId {
            AccountId::from([0x1; 32])
        }

        fn contract_address() -> AccountId {
            AccountId::from([0x2; 32])
        }

    }

}
