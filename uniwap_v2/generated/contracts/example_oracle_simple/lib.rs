#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

// Generated with Sol2Ink v2.1.0
// https://github.com/Brushfam/sol2ink

/// fixed window oracle that recomputes the average price for the entire period once every period
/// note that the price average is only guaranteed to be over at least 1 period, but may be over a longer period
#[openbrush::contract]
pub mod example_oracle_simple {
    use generated::*;
    use ink::lang::codegen::{
        EmitEvent,
        Env,
    };
    use openbrush::traits::Storage;

    pub const PERIOD: u128 = 24 * 3600i128;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct ExampleOracleSimpleContract {
        #[storage_field]
        data: impls::Data,
    }

    impl ExampleOracleSimple for ExampleOracleSimpleContract {}

    impl ExampleOracleSimpleContract {
        #[ink(constructor)]
        pub fn new(factory: AccountId, token_a: AccountId, token_b: AccountId) -> Self {
            let mut instance = Self::default();
            let mut pair: IUniswapV2Pair =
                i_uniswap_v_2_pair(uniswap_v_2_library.pair_for(factory, token_a, token_b)?)?;
            instance.data.pair = pair;
            instance.data.token_0 = pair.token_0()?;
            instance.data.token_1 = pair.token_1()?;
            instance.data.price_0_cumulative_last = pair.price_0_cumulative_last()?;
            instance.data.price_1_cumulative_last = pair.price_1_cumulative_last()?;
            (_, _, _) = pair.get_reserves()?;
            if !(reserve_0 != 0 && reserve_1 != 0) {
                return Err(Error::Custom(String::from(
                    "ExampleOracleSimple: NO_RESERVES",
                )))
            };
            instance
        }

    }
}
