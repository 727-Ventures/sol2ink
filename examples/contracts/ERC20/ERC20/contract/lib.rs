#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

// Generated with Sol2Ink v2.0.0-beta
// https://github.com/727-Ventures/sol2ink

#[openbrush::contract]
pub mod erc_20 {
    use erc_20::*;
    use ink_lang::codegen::{
        EmitEvent,
        Env,
    };
    use ink_storage::traits::SpreadAllocate;
    use openbrush::traits::Storage;
    use scale::{
        Decode,
        Encode,
    };


    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: AccountId,
        #[ink(topic)]
        to: AccountId,
        value: u128,
    }

    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        spender: AccountId,
        value: u128,
    }

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, Storage)]
    pub struct ERC20Contract {
        #[storage_field]
        data: impls::Data,
    }

    impl ERC20 for ERC20Contract {}

    impl erc_20::Internal for ERC20Contract {
        fn _emit_transfer(&self, from: AccountId, to: AccountId, value: u128) {
            self.env().emit_event(Transfer { from, to, value });
        }

        fn _emit_approval(&self, owner: AccountId, spender: AccountId, value: u128) {
            self.env().emit_event(Approval {
                owner,
                spender,
                value,
            });
        }

    }

    impl ERC20Contract {
        #[ink(constructor)]
        pub fn new(name: String, symbol: String) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {
                self.data().name = name;
                self.data().symbol = symbol;
            })
        }

    }
}
