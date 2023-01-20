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



#[openbrush::wrapper]
pub type ERC721Ref = dyn ERC721;

#[openbrush::trait_definition]
pub trait ERC721 {
    #[ink(message)]
    fn supports_interface(&self, interface_id: [u8; 4]) -> Result<bool, Error>;

    #[ink(message)]
    fn balance_of(&self, owner: AccountId) -> Result<u128, Error>;

    #[ink(message)]
    fn owner_of(&self, token_id: u128) -> Result<AccountId, Error>;

    #[ink(message)]
    fn name(&self) -> Result<String, Error>;

    #[ink(message)]
    fn symbol(&self) -> Result<String, Error>;

    #[ink(message)]
    fn token_uri(&self, token_id: u128) -> Result<String, Error>;

    #[ink(message)]
    fn approve(&mut self, to: AccountId, token_id: u128) -> Result<(), Error>;

    #[ink(message)]
    fn get_approved(&self, token_id: u128) -> Result<AccountId, Error>;

    #[ink(message)]
    fn set_approval_for_all(&mut self, operator: AccountId, approved: bool) -> Result<(), Error>;

    #[ink(message)]
    fn is_approved_for_all(&self, owner: AccountId, operator: AccountId) -> Result<bool, Error>;

    #[ink(message)]
    fn transfer_from(
        &mut self,
        from: AccountId,
        to: AccountId,
        token_id: u128,
    ) -> Result<(), Error>;

    #[ink(message)]
    fn safe_transfer_from(
        &mut self,
        from: AccountId,
        to: AccountId,
        token_id: u128,
    ) -> Result<(), Error>;

    #[ink(message)]
    fn safe_transfer_from(
        &mut self,
        from: AccountId,
        to: AccountId,
        token_id: u128,
        data: Vec<u8>,
    ) -> Result<(), Error>;

    #[ink(message)]
    fn name(&self) -> String;

    #[ink(message)]
    fn symbol(&self) -> String;

    #[ink(message)]
    fn owners(&self) -> Mapping<u128, AccountId>;

    #[ink(message)]
    fn balances(&self) -> Mapping<AccountId, u128>;

    #[ink(message)]
    fn token_approvals(&self) -> Mapping<u128, AccountId>;

    #[ink(message)]
    fn operator_approvals(&self) -> Mapping<(AccountId, AccountId), bool>;

}
