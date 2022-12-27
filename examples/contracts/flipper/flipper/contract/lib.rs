#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

// Generated with Sol2Ink v1.1.0
// https://github.com/Supercolony-net/sol2ink

#[openbrush::contract]
pub mod flipper {
    use flipper::*;
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


    #[ink(storage)]
    #[derive(Default, SpreadAllocate, Storage)]
    pub struct flipperContract {
        #[storage_field]
        data: impls::Data,
    }

    impl flipper for flipperContract {}

    impl flipper::Internal for flipperContract {}

    impl flipperContract {
        ///Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(initvalue: bool) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {
                instance.data().value = initvalue;
            })
        }

    }
}
