// Generated with Sol2Ink v2.0.0-beta
// https://github.com/Supercolony-net/sol2ink


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

#[openbrush::wrapper]
pub type IERC1155Ref = dyn IERC1155;

#[openbrush::trait_definition]
pub trait IERC1155 {
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

}
