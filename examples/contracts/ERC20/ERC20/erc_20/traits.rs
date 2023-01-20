// Generated with Sol2Ink v2.0.0-beta
// https://github.com/727-Ventures/sol2ink

use scale::{
    Decode,
    Encode,
};

#[derive(Debug, Encode, Decode, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum Error {
    Custom(String),
}

pub enum Enum {
    First,
    Second,
}


#[derive(Default, Encode, Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct Struct {
    field_1: u128,
    field_2: u128,
}


#[openbrush::wrapper]
pub type ERC20Ref = dyn ERC20;

#[openbrush::trait_definition]
pub trait ERC20 {
    #[ink(message)]
    fn name(&self) -> Result<String, Error>;

    #[ink(message)]
    fn symbol(&self) -> Result<String, Error>;

    #[ink(message)]
    fn decimals(&self) -> Result<u8, Error>;

    #[ink(message)]
    fn total_supply(&self) -> Result<u128, Error>;

    #[ink(message)]
    fn balance_of(&self, account: AccountId) -> Result<u128, Error>;

    #[ink(message)]
    fn transfer(&mut self, to: AccountId, amount: u128) -> Result<bool, Error>;

    #[ink(message)]
    fn allowance(&self, owner: AccountId, spender: AccountId) -> Result<u128, Error>;

    #[ink(message)]
    fn approve(&mut self, spender: AccountId, amount: u128) -> Result<bool, Error>;

    #[ink(message)]
    fn transfer_from(
        &mut self,
        from: AccountId,
        to: AccountId,
        amount: u128,
    ) -> Result<bool, Error>;

    #[ink(message)]
    fn increase_allowance(&mut self, spender: AccountId, added_value: u128) -> Result<bool, Error>;

    #[ink(message)]
    fn decrease_allowance(
        &mut self,
        spender: AccountId,
        subtracted_value: u128,
    ) -> Result<bool, Error>;

    #[ink(message)]
    fn balances(&self) -> Mapping<AccountId, u128>;

    #[ink(message)]
    fn allowances(&self) -> Mapping<(AccountId, AccountId), u128>;

    #[ink(message)]
    fn total_supply(&self) -> u128;

    #[ink(message)]
    fn name(&self) -> String;

    #[ink(message)]
    fn symbol(&self) -> String;

}
