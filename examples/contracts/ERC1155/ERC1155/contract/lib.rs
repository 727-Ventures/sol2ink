#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

// Generated with Sol2Ink v2.0.0-beta
// https://github.com/727-Ventures/sol2ink

#[openbrush::contract]
pub mod erc_1155 {
    use erc_1155::*;
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
    pub struct TransferSingle {
        #[ink(topic)]
        operator: AccountId,
        #[ink(topic)]
        from: AccountId,
        #[ink(topic)]
        to: AccountId,
        id: u128,
        value: u128,
    }

    #[ink(event)]
    pub struct TransferBatch {
        #[ink(topic)]
        operator: AccountId,
        #[ink(topic)]
        from: AccountId,
        #[ink(topic)]
        to: AccountId,
        ids: Vec<u128>,
        values: Vec<u128>,
    }

    #[ink(event)]
    pub struct ApprovalForAll {
        #[ink(topic)]
        account: AccountId,
        #[ink(topic)]
        operator: AccountId,
        approved: bool,
    }

    #[ink(event)]
    pub struct URI {
        value: String,
        #[ink(topic)]
        id: u128,
    }

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, Storage)]
    pub struct ERC1155Contract {
        #[storage_field]
        data: impls::Data,
    }

    impl ERC1155 for ERC1155Contract {}

    impl erc_1155::Internal for ERC1155Contract {
        fn _emit_transfer_single(
            &self,
            operator: AccountId,
            from: AccountId,
            to: AccountId,
            id: u128,
            value: u128,
        ) {
            self.env().emit_event(TransferSingle {
                operator,
                from,
                to,
                id,
                value,
            });
        }

        fn _emit_transfer_batch(
            &self,
            operator: AccountId,
            from: AccountId,
            to: AccountId,
            ids: Vec<u128>,
            values: Vec<u128>,
        ) {
            self.env().emit_event(TransferBatch {
                operator,
                from,
                to,
                ids,
                values,
            });
        }

        fn _emit_approval_for_all(&self, account: AccountId, operator: AccountId, approved: bool) {
            self.env().emit_event(ApprovalForAll {
                account,
                operator,
                approved,
            });
        }

        fn _emit_uri(&self, value: String, id: u128) {
            self.env().emit_event(URI { value, id });
        }

    }

    impl ERC1155Contract {
        #[ink(constructor)]
        pub fn new(uri: String) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {
                instance._set_uri(uri)?;
            })
        }

    }
}
