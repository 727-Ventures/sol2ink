#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

// Generated with Sol2Ink v2.1.0
// https://github.com/Brushfam/sol2ink

#[openbrush::contract]
pub mod struct_contract {
    use generated::*;
    use ink::lang::codegen::{
        EmitEvent,
        Env,
    };
    use openbrush::traits::Storage;


    #[ink(event)]
    pub struct Log {
        #[ink(topic)]
        sender: AccountId,
        message: String,
        priority: u8,
        status: Status,
    }

    #[ink(event)]
    pub struct AnotherLog {}

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct StructContractContract {
        #[storage_field]
        data: impls::Data,
    }

    impl StructContract for StructContractContract {}
    impl generated::impls::struct_contract::Internal for StructContractContract {

        fn _emit_log(&self, sender: AccountId, message: String, priority: u8, status: Status) {
            self.env().emit_event(Log {
                sender,
                message,
                priority,
                status,
            });
        }

        fn _emit_another_log(&self) {
            self.env().emit_event(AnotherLog {});
        }

    }

    impl StructContractContract {
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut instance = Self::default();
            instance
        }

    }
}
