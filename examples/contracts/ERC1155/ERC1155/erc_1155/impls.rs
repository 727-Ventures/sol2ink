// Generated with Sol2Ink v2.0.0-beta
// https://github.com/Supercolony-net/sol2ink

pub use crate::{
    impls,
    traits::*,
};
use openbrush::traits::Storage;

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct Data {
    pub balances: Mapping<(u128, AccountId), u128>,
    pub operator_approvals: Mapping<(AccountId, AccountId), bool>,
    pub uri: String,
    pub _reserved: Option<()>,
}


impl<T: Storage<Data>> ERC1155 for T {
    fn supports_interface(&self, interface_id: [u8; 4]) -> Result<bool, Error> {
        return Ok(interface_id == type_of(ierc_1155)?.interface_id
            || interface_id == type_of(ierc_1155_metadata_uri)?.interface_id
            || super.supports_interface(interface_id)?)
    }

    fn uri(&self, _: u128) -> Result<String, Error> {
        return Ok(self.data().uri)
    }

    fn balance_of(&self, account: AccountId, id: u128) -> Result<u128, Error> {
        if !(account != ZERO_ADDRESS.into()) {
            return Err(Error::Custom(String::from(
                "ERC1155: address zero is not a valid owner",
            )))
        };
        return Ok(self.data().balances.get(&(id, account)).unwrap_or_default())
    }

    fn balance_of_batch(
        &self,
        accounts: Vec<AccountId>,
        ids: Vec<u128>,
    ) -> Result<Vec<u128>, Error> {
        if !(accounts.length == ids.length) {
            return Err(Error::Custom(String::from(
                "ERC1155: accounts and ids length mismatch",
            )))
        };
        let mut batch_balances: Vec<u128> = vec![u128::default(); accounts.length];
        let mut i: u128 = 0;
        while i < accounts.length {
            batch_balances[i] = self.balance_of(accounts[i], ids[i])?;
            i += 1;
        }
        return Ok(batch_balances)
    }

    fn set_approval_for_all(&mut self, operator: AccountId, approved: bool) -> Result<(), Error> {
        self._set_approval_for_all(Self::env().caller(), operator, approved)?;
        Ok(())
    }

    fn is_approved_for_all(&self, account: AccountId, operator: AccountId) -> Result<bool, Error> {
        return Ok(self
            .data()
            .operator_approvals
            .get(&(account, operator))
            .unwrap_or_default())
    }

    fn safe_transfer_from(
        &mut self,
        from: AccountId,
        to: AccountId,
        id: u128,
        amount: u128,
        data: Vec<u8>,
    ) -> Result<(), Error> {
        if !(from == Self::env().caller()
            || self.is_approved_for_all(from, Self::env().caller())?)
        {
            return Err(Error::Custom(String::from(
                "ERC1155: caller is not token owner nor approved",
            )))
        };
        self._safe_transfer_from(from, to, id, amount, data)?;
        Ok(())
    }

    fn safe_batch_transfer_from(
        &mut self,
        from: AccountId,
        to: AccountId,
        ids: Vec<u128>,
        amounts: Vec<u128>,
        data: Vec<u8>,
    ) -> Result<(), Error> {
        if !(from == Self::env().caller()
            || self.is_approved_for_all(from, Self::env().caller())?)
        {
            return Err(Error::Custom(String::from(
                "ERC1155: caller is not token owner nor approved",
            )))
        };
        self._safe_batch_transfer_from(from, to, ids, amounts, data)?;
        Ok(())
    }

}

pub trait Internal {
    fn _safe_transfer_from(
        &mut self,
        from: AccountId,
        to: AccountId,
        id: u128,
        amount: u128,
        data: Vec<u8>,
    ) -> Result<(), Error>;

    fn _safe_batch_transfer_from(
        &mut self,
        from: AccountId,
        to: AccountId,
        ids: Vec<u128>,
        amounts: Vec<u128>,
        data: Vec<u8>,
    ) -> Result<(), Error>;

    fn _set_uri(&mut self, newuri: String) -> Result<(), Error>;

    fn _mint(&mut self, to: AccountId, id: u128, amount: u128, data: Vec<u8>) -> Result<(), Error>;

    fn _mint_batch(
        &mut self,
        to: AccountId,
        ids: Vec<u128>,
        amounts: Vec<u128>,
        data: Vec<u8>,
    ) -> Result<(), Error>;

    fn _burn(&mut self, from: AccountId, id: u128, amount: u128) -> Result<(), Error>;

    fn _burn_batch(
        &mut self,
        from: AccountId,
        ids: Vec<u128>,
        amounts: Vec<u128>,
    ) -> Result<(), Error>;

    fn _set_approval_for_all(
        &mut self,
        owner: AccountId,
        operator: AccountId,
        approved: bool,
    ) -> Result<(), Error>;

    fn _before_token_transfer(
        &mut self,
        operator: AccountId,
        from: AccountId,
        to: AccountId,
        ids: Vec<u128>,
        amounts: Vec<u128>,
        data: Vec<u8>,
    ) -> Result<(), Error>;

    fn _after_token_transfer(
        &mut self,
        operator: AccountId,
        from: AccountId,
        to: AccountId,
        ids: Vec<u128>,
        amounts: Vec<u128>,
        data: Vec<u8>,
    ) -> Result<(), Error>;

    fn _do_safe_transfer_acceptance_check(
        &mut self,
        operator: AccountId,
        from: AccountId,
        to: AccountId,
        id: u128,
        amount: u128,
        data: Vec<u8>,
    ) -> Result<(), Error>;

    fn _do_safe_batch_transfer_acceptance_check(
        &mut self,
        operator: AccountId,
        from: AccountId,
        to: AccountId,
        ids: Vec<u128>,
        amounts: Vec<u128>,
        data: Vec<u8>,
    ) -> Result<(), Error>;

    fn _as_singleton_array(&self, element: u128) -> Result<Vec<u128>, Error>;

    fn _emit_transfer_single(
        &self,
        operator: AccountId,
        from: AccountId,
        to: AccountId,
        id: u128,
        value: u128,
    );

    fn _emit_transfer_batch(
        &self,
        operator: AccountId,
        from: AccountId,
        to: AccountId,
        ids: Vec<u128>,
        values: Vec<u128>,
    );

    fn _emit_approval_for_all(&self, account: AccountId, operator: AccountId, approved: bool);

    fn _emit_uri(&self, value: String, id: u128);

}

impl<T: Storage<Data>> Internal for T {
    default fn _safe_transfer_from(
        &mut self,
        from: AccountId,
        to: AccountId,
        id: u128,
        amount: u128,
        data: Vec<u8>,
    ) -> Result<(), Error> {
        if !(to != ZERO_ADDRESS.into()) {
            return Err(Error::Custom(String::from(
                "ERC1155: transfer to the zero address",
            )))
        };
        let mut operator: AccountId = Self::env().caller();
        let mut ids: Vec<u128> = self._as_singleton_array(id)?;
        let mut amounts: Vec<u128> = self._as_singleton_array(amount)?;
        self._before_token_transfer(operator, from, to, ids, amounts, data)?;
        let mut from_balance: u128 = self.data().balances.get(&(id, from)).unwrap_or_default();
        if !(from_balance >= amount) {
            return Err(Error::Custom(String::from(
                "ERC1155: insufficient balance for transfer",
            )))
        };
        // Please handle unchecked blocks manually >>>
        self.data()
            .balances
            .insert(&(id, from), &(from_balance - amount));
        // <<< Please handle unchecked blocks manually
        let new_value = self.data().balances.get(&(id, to)).unwrap_or_default();
        self.data().balances.insert(&(id, to), &new_value);
        self._emit_transfer_single(operator, from, to, id, amount);
        self._after_token_transfer(operator, from, to, ids, amounts, data)?;
        self._do_safe_transfer_acceptance_check(operator, from, to, id, amount, data)?;
        Ok(())
    }

    default fn _safe_batch_transfer_from(
        &mut self,
        from: AccountId,
        to: AccountId,
        ids: Vec<u128>,
        amounts: Vec<u128>,
        data: Vec<u8>,
    ) -> Result<(), Error> {
        if !(ids.length == amounts.length) {
            return Err(Error::Custom(String::from(
                "ERC1155: ids and amounts length mismatch",
            )))
        };
        if !(to != ZERO_ADDRESS.into()) {
            return Err(Error::Custom(String::from(
                "ERC1155: transfer to the zero address",
            )))
        };
        let mut operator: AccountId = Self::env().caller();
        self._before_token_transfer(operator, from, to, ids, amounts, data)?;
        let mut i: u128 = 0;
        while i < ids.length {
            let mut id: u128 = ids[i];
            let mut amount: u128 = amounts[i];
            let mut from_balance: u128 = self.data().balances.get(&(id, from)).unwrap_or_default();
            if !(from_balance >= amount) {
                return Err(Error::Custom(String::from(
                    "ERC1155: insufficient balance for transfer",
                )))
            };
            // Please handle unchecked blocks manually >>>
            self.data()
                .balances
                .insert(&(id, from), &(from_balance - amount));
            // <<< Please handle unchecked blocks manually
            let new_value = self.data().balances.get(&(id, to)).unwrap_or_default();
            self.data().balances.insert(&(id, to), &new_value);
            i += 1;
        }
        self._emit_transfer_batch(operator, from, to, ids, amounts);
        self._after_token_transfer(operator, from, to, ids, amounts, data)?;
        self._do_safe_batch_transfer_acceptance_check(operator, from, to, ids, amounts, data)?;
        Ok(())
    }

    default fn _set_uri(&mut self, newuri: String) -> Result<(), Error> {
        self.data().uri = newuri;
        Ok(())
    }

    default fn _mint(
        &mut self,
        to: AccountId,
        id: u128,
        amount: u128,
        data: Vec<u8>,
    ) -> Result<(), Error> {
        if !(to != ZERO_ADDRESS.into()) {
            return Err(Error::Custom(String::from(
                "ERC1155: mint to the zero address",
            )))
        };
        let mut operator: AccountId = Self::env().caller();
        let mut ids: Vec<u128> = self._as_singleton_array(id)?;
        let mut amounts: Vec<u128> = self._as_singleton_array(amount)?;
        self._before_token_transfer(operator, ZERO_ADDRESS.into(), to, ids, amounts, data)?;
        let new_value = self.data().balances.get(&(id, to)).unwrap_or_default();
        self.data().balances.insert(&(id, to), &new_value);
        self._emit_transfer_single(operator, ZERO_ADDRESS.into(), to, id, amount);
        self._after_token_transfer(operator, ZERO_ADDRESS.into(), to, ids, amounts, data)?;
        self._do_safe_transfer_acceptance_check(
            operator,
            ZERO_ADDRESS.into(),
            to,
            id,
            amount,
            data,
        )?;
        Ok(())
    }

    default fn _mint_batch(
        &mut self,
        to: AccountId,
        ids: Vec<u128>,
        amounts: Vec<u128>,
        data: Vec<u8>,
    ) -> Result<(), Error> {
        if !(to != ZERO_ADDRESS.into()) {
            return Err(Error::Custom(String::from(
                "ERC1155: mint to the zero address",
            )))
        };
        if !(ids.length == amounts.length) {
            return Err(Error::Custom(String::from(
                "ERC1155: ids and amounts length mismatch",
            )))
        };
        let mut operator: AccountId = Self::env().caller();
        self._before_token_transfer(operator, ZERO_ADDRESS.into(), to, ids, amounts, data)?;
        let mut i: u128 = 0;
        while i < ids.length {
            let new_value = self.data().balances.get(&(ids[i], to)).unwrap_or_default();
            self.data().balances.insert(&(ids[i], to), &new_value);
            i += 1;
        }
        self._emit_transfer_batch(operator, ZERO_ADDRESS.into(), to, ids, amounts);
        self._after_token_transfer(operator, ZERO_ADDRESS.into(), to, ids, amounts, data)?;
        self._do_safe_batch_transfer_acceptance_check(
            operator,
            ZERO_ADDRESS.into(),
            to,
            ids,
            amounts,
            data,
        )?;
        Ok(())
    }

    default fn _burn(&mut self, from: AccountId, id: u128, amount: u128) -> Result<(), Error> {
        if !(from != ZERO_ADDRESS.into()) {
            return Err(Error::Custom(String::from(
                "ERC1155: burn from the zero address",
            )))
        };
        let mut operator: AccountId = Self::env().caller();
        let mut ids: Vec<u128> = self._as_singleton_array(id)?;
        let mut amounts: Vec<u128> = self._as_singleton_array(amount)?;
        self._before_token_transfer(operator, from, ZERO_ADDRESS.into(), ids, amounts, "")?;
        let mut from_balance: u128 = self.data().balances.get(&(id, from)).unwrap_or_default();
        if !(from_balance >= amount) {
            return Err(Error::Custom(String::from(
                "ERC1155: burn amount exceeds balance",
            )))
        };
        // Please handle unchecked blocks manually >>>
        self.data()
            .balances
            .insert(&(id, from), &(from_balance - amount));
        // <<< Please handle unchecked blocks manually
        self._emit_transfer_single(operator, from, ZERO_ADDRESS.into(), id, amount);
        self._after_token_transfer(operator, from, ZERO_ADDRESS.into(), ids, amounts, "")?;
        Ok(())
    }

    default fn _burn_batch(
        &mut self,
        from: AccountId,
        ids: Vec<u128>,
        amounts: Vec<u128>,
    ) -> Result<(), Error> {
        if !(from != ZERO_ADDRESS.into()) {
            return Err(Error::Custom(String::from(
                "ERC1155: burn from the zero address",
            )))
        };
        if !(ids.length == amounts.length) {
            return Err(Error::Custom(String::from(
                "ERC1155: ids and amounts length mismatch",
            )))
        };
        let mut operator: AccountId = Self::env().caller();
        self._before_token_transfer(operator, from, ZERO_ADDRESS.into(), ids, amounts, "")?;
        let mut i: u128 = 0;
        while i < ids.length {
            let mut id: u128 = ids[i];
            let mut amount: u128 = amounts[i];
            let mut from_balance: u128 = self.data().balances.get(&(id, from)).unwrap_or_default();
            if !(from_balance >= amount) {
                return Err(Error::Custom(String::from(
                    "ERC1155: burn amount exceeds balance",
                )))
            };
            // Please handle unchecked blocks manually >>>
            self.data()
                .balances
                .insert(&(id, from), &(from_balance - amount));
            // <<< Please handle unchecked blocks manually
            i += 1;
        }
        self._emit_transfer_batch(operator, from, ZERO_ADDRESS.into(), ids, amounts);
        self._after_token_transfer(operator, from, ZERO_ADDRESS.into(), ids, amounts, "")?;
        Ok(())
    }

    default fn _set_approval_for_all(
        &mut self,
        owner: AccountId,
        operator: AccountId,
        approved: bool,
    ) -> Result<(), Error> {
        if !(owner != operator) {
            return Err(Error::Custom(String::from(
                "ERC1155: setting approval status for self",
            )))
        };
        self.data()
            .operator_approvals
            .insert(&(owner, operator), &approved);
        self._emit_approval_for_all(owner, operator, approved);
        Ok(())
    }

    default fn _before_token_transfer(
        &mut self,
        operator: AccountId,
        from: AccountId,
        to: AccountId,
        ids: Vec<u128>,
        amounts: Vec<u128>,
        data: Vec<u8>,
    ) -> Result<(), Error> {
        Ok(())
    }

    default fn _after_token_transfer(
        &mut self,
        operator: AccountId,
        from: AccountId,
        to: AccountId,
        ids: Vec<u128>,
        amounts: Vec<u128>,
        data: Vec<u8>,
    ) -> Result<(), Error> {
        Ok(())
    }

    default fn _do_safe_transfer_acceptance_check(
        &mut self,
        operator: AccountId,
        from: AccountId,
        to: AccountId,
        id: u128,
        amount: u128,
        data: Vec<u8>,
    ) -> Result<(), Error> {
        if to.is_contract()? {
            if ierc_1155_receiver(to)?
                .on_erc_1155_received(operator, from, id, amount, data)?
                .is_err()
            {
                return Err(Error::Custom("Try failed"))
            }
        }
        Ok(())
    }

    default fn _do_safe_batch_transfer_acceptance_check(
        &mut self,
        operator: AccountId,
        from: AccountId,
        to: AccountId,
        ids: Vec<u128>,
        amounts: Vec<u128>,
        data: Vec<u8>,
    ) -> Result<(), Error> {
        if to.is_contract()? {
            if ierc_1155_receiver(to)?
                .on_erc_1155_batch_received(operator, from, ids, amounts, data)?
                .is_err()
            {
                return Err(Error::Custom("Try failed"))
            }
        }
        Ok(())
    }

    default fn _as_singleton_array(&self, element: u128) -> Result<Vec<u128>, Error> {
        let mut array: Vec<u128> = vec![u128::default(); 1];
        array[0] = element;
        return Ok(array)
    }

    default fn _emit_transfer_single(
        &self,
        _: AccountId,
        _: AccountId,
        _: AccountId,
        _: u128,
        _: u128,
    ) {
    }

    default fn _emit_transfer_batch(
        &self,
        _: AccountId,
        _: AccountId,
        _: AccountId,
        _: Vec<u128>,
        _: Vec<u128>,
    ) {
    }

    default fn _emit_approval_for_all(&self, _: AccountId, _: AccountId, _: bool) {}

    default fn _emit_uri(&self, _: String, _: u128) {}

}
