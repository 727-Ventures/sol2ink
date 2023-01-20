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
pub type ERC1155Ref = dyn ERC1155;

#[openbrush::trait_definition]
pub trait ERC1155 {
    #[ink(message)]
    fn supports_interface(&self, interface_id: [u8; 4]) -> Result<bool, Error>;

    #[ink(message)]
    fn uri(&self, _: u128) -> Result<String, Error>;

    #[ink(message)]
    fn balance_of(&self, account: AccountId, id: u128) -> Result<u128, Error>;

    #[ink(message)]
    fn balance_of_batch(
        &self,
        accounts: Vec<AccountId>,
        ids: Vec<u128>,
    ) -> Result<Vec<u128>, Error>;

    #[ink(message)]
    fn set_approval_for_all(&mut self, operator: AccountId, approved: bool) -> Result<(), Error>;

    #[ink(message)]
    fn is_approved_for_all(&self, account: AccountId, operator: AccountId) -> Result<bool, Error>;

    #[ink(message)]
    fn safe_transfer_from(
        &mut self,
        from: AccountId,
        to: AccountId,
        id: u128,
        amount: u128,
        data: Vec<u8>,
    ) -> Result<(), Error>;

    #[ink(message)]
    fn safe_batch_transfer_from(
        &mut self,
        from: AccountId,
        to: AccountId,
        ids: Vec<u128>,
        amounts: Vec<u128>,
        data: Vec<u8>,
    ) -> Result<(), Error>;

    #[ink(message)]
    fn balances(&self) -> Mapping<(u128, AccountId), u128>;

    #[ink(message)]
    fn operator_approvals(&self) -> Mapping<(AccountId, AccountId), bool>;

    #[ink(message)]
    fn uri(&self) -> String;

}
