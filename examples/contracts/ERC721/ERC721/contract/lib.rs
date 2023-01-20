#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

// Generated with Sol2Ink v2.0.0-beta
// https://github.com/727-Ventures/sol2ink

#[openbrush::contract]
pub mod erc_721 {
    use erc_721::*;
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
        #[ink(topic)]
        token_id: u128,
    }

    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        approved: AccountId,
        #[ink(topic)]
        token_id: u128,
    }

    #[ink(event)]
    pub struct ApprovalForAll {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        operator: AccountId,
        approved: bool,
    }

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, Storage)]
    pub struct ERC721Contract {
        #[storage_field]
        data: impls::Data,
    }

    impl ERC721 for ERC721Contract {}

    impl erc_721::Internal for ERC721Contract {
        fn _emit_transfer(&self, from: AccountId, to: AccountId, token_id: u128) {
            self.env().emit_event(Transfer { from, to, token_id });
        }

        fn _emit_approval(&self, owner: AccountId, approved: AccountId, token_id: u128) {
            self.env().emit_event(Approval {
                owner,
                approved,
                token_id,
            });
        }

        fn _emit_approval_for_all(&self, owner: AccountId, operator: AccountId, approved: bool) {
            self.env().emit_event(ApprovalForAll {
                owner,
                operator,
                approved,
            });
        }

    }

    impl ERC721Contract {
        #[ink(constructor)]
        pub fn new(name: String, symbol: String) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {
                self.data().name = name;
                self.data().symbol = symbol;
            })
        }

    }
}
