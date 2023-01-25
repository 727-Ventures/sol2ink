#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

// Generated with Sol2Ink v2.0.0-beta
// https://github.com/727-Ventures/sol2ink

/// example.sol
#[openbrush::contract]
pub mod example {
    use example::*;
    use ink_lang::codegen::{
        EmitEvent,
        Env,
    };
    use ink_storage::traits::SpreadAllocate;
    use openbrush::traits::{
        AccountId,
        AccountIdExt,
        Storage,
        ZERO_ADDRESS,
    };
    use scale::{
        Decode,
        Encode,
    };

    /// Constants
    pub const BAD_STATE: State = state.zombie;
    pub const FIRST_PID: i32 = 1;

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, Storage)]
    pub struct exampleContract {
        #[storage_field]
        data: impls::Data,
    }

    impl example for exampleContract {}

    impl example::Internal for exampleContract {}

    impl exampleContract {
        /// Our constructors
        #[ink(constructor)]
        pub fn new(pid: i32) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {
                instance.data().pid = pid;
                self.reaped = 3;
                self.card_1 = card(value.two, suit.club)?;
                self.card_2 = card {
                    s: suit.club,
                    v: value.two,
                };
            })
        }

    }
}
