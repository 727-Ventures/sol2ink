// Generated with Sol2Ink v2.0.0
// https://github.com/727-Ventures/sol2ink

use ink_prelude::vec::*;
use openbrush::{
    storage::Mapping,
    traits::{
        AccountId,
        AccountIdExt,
        String,
        ZERO_ADDRESS,
    },
};

/// @dev Emitted when `tokenId` token is transferred from `from` to `to`.
#[ink(event)]
pub struct Transfer {
    #[ink(topic)]
    from: AccountId,
    #[ink(topic)]
    to: AccountId,
    #[ink(topic)]
    token_id: u128,
}

/// @dev Emitted when `owner` enables `approved` to manage the `tokenId` token.
#[ink(event)]
pub struct Approval {
    #[ink(topic)]
    owner: AccountId,
    #[ink(topic)]
    approved: AccountId,
    #[ink(topic)]
    token_id: u128,
}

/// @dev Emitted when `owner` enables or disables (`approved`) `operator` to manage all of its assets.
#[ink(event)]
pub struct ApprovalForAll {
    #[ink(topic)]
    owner: AccountId,
    #[ink(topic)]
    operator: AccountId,
    approved: bool,
}

#[openbrush::wrapper]
pub type IERC721Ref = dyn IERC721;

#[openbrush::trait_definition]
pub trait IERC721 {
    /// @dev Returns the number of tokens in ``owner``'s account.
    #[ink(message)]
    fn balance_of(&self, owner: AccountId) -> Result<u128, Error>;

    /// @dev Returns the owner of the `tokenId` token.
    ///
    /// Requirements:
    ///
    /// - `tokenId` must exist.
    #[ink(message)]
    fn owner_of(&self, token_id: u128) -> Result<AccountId, Error>;

    /// @dev Safely transfers `tokenId` token from `from` to `to`.
    ///
    /// Requirements:
    ///
    /// - `from` cannot be the zero address.
    /// - `to` cannot be the zero address.
    /// - `tokenId` token must exist and be owned by `from`.
    /// - If the caller is not `from`, it must be approved to move this token by either {approve} or {setApprovalForAll}.
    /// - If `to` refers to a smart contract, it must implement {IERC721Receiver-onERC721Received}, which is called upon a safe transfer.
    ///
    /// Emits a {Transfer} event.
    #[ink(message)]
    fn safe_transfer_from(
        &mut self,
        from: AccountId,
        to: AccountId,
        token_id: u128,
        data: Vec<u8>,
    ) -> Result<(), Error>;

    /// @dev Safely transfers `tokenId` token from `from` to `to`, checking first that contract recipients
    /// are aware of the ERC721 protocol to prevent tokens from being forever locked.
    ///
    /// Requirements:
    ///
    /// - `from` cannot be the zero address.
    /// - `to` cannot be the zero address.
    /// - `tokenId` token must exist and be owned by `from`.
    /// - If the caller is not `from`, it must have been allowed to move this token by either {approve} or {setApprovalForAll}.
    /// - If `to` refers to a smart contract, it must implement {IERC721Receiver-onERC721Received}, which is called upon a safe transfer.
    ///
    /// Emits a {Transfer} event.
    #[ink(message)]
    fn safe_transfer_from(
        &mut self,
        from: AccountId,
        to: AccountId,
        token_id: u128,
    ) -> Result<(), Error>;

    /// @dev Transfers `tokenId` token from `from` to `to`.
    ///
    /// WARNING: Note that the caller is responsible to confirm that the recipient is capable of receiving ERC721
    /// or else they may be permanently lost. Usage of {safeTransferFrom} prevents loss, though the caller must
    /// understand this adds an external call which potentially creates a reentrancy vulnerability.
    ///
    /// Requirements:
    ///
    /// - `from` cannot be the zero address.
    /// - `to` cannot be the zero address.
    /// - `tokenId` token must be owned by `from`.
    /// - If the caller is not `from`, it must be approved to move this token by either {approve} or {setApprovalForAll}.
    ///
    /// Emits a {Transfer} event.
    #[ink(message)]
    fn transfer_from(
        &mut self,
        from: AccountId,
        to: AccountId,
        token_id: u128,
    ) -> Result<(), Error>;

    /// @dev Gives permission to `to` to transfer `tokenId` token to another account.
    /// The approval is cleared when the token is transferred.
    ///
    /// Only a single account can be approved at a time, so approving the zero address clears previous approvals.
    ///
    /// Requirements:
    ///
    /// - The caller must own the token or be an approved operator.
    /// - `tokenId` must exist.
    ///
    /// Emits an {Approval} event.
    #[ink(message)]
    fn approve(&mut self, to: AccountId, token_id: u128) -> Result<(), Error>;

    /// @dev Approve or remove `operator` as an operator for the caller.
    /// Operators can call {transferFrom} or {safeTransferFrom} for any token owned by the caller.
    ///
    /// Requirements:
    ///
    /// - The `operator` cannot be the caller.
    ///
    /// Emits an {ApprovalForAll} event.
    #[ink(message)]
    fn set_approval_for_all(&mut self, operator: AccountId, approved: bool) -> Result<(), Error>;

    /// @dev Returns the account approved for `tokenId` token.
    ///
    /// Requirements:
    ///
    /// - `tokenId` must exist.
    #[ink(message)]
    fn get_approved(&self, token_id: u128) -> Result<AccountId, Error>;

    /// @dev Returns if the `operator` is allowed to manage all of the assets of `owner`.
    ///
    /// See {setApprovalForAll}
    #[ink(message)]
    fn is_approved_for_all(&self, owner: AccountId, operator: AccountId) -> Result<bool, Error>;

}
