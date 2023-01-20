// Generated with Sol2Ink v2.0.0-beta
// https://github.com/727-Ventures/sol2ink

pub use crate::{
    impls,
    traits::*,
};
use openbrush::traits::Storage;

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct Data {
    pub name: String,
    pub symbol: String,
    pub owners: Mapping<u128, AccountId>,
    pub balances: Mapping<AccountId, u128>,
    pub token_approvals: Mapping<u128, AccountId>,
    pub operator_approvals: Mapping<(AccountId, AccountId), bool>,
    pub _reserved: Option<()>,
}


impl<T: Storage<Data>> ERC721 for T {
    fn supports_interface(&self, interface_id: [u8; 4]) -> Result<bool, Error> {
        return Ok(interface_id == type_of(ierc_721)?.interface_id
            || interface_id == type_of(ierc_721_metadata)?.interface_id
            || super.supports_interface(interface_id)?)
    }

    fn balance_of(&self, owner: AccountId) -> Result<u128, Error> {
        if !(owner != ZERO_ADDRESS.into()) {
            return Err(Error::Custom(String::from(
                "ERC721: address zero is not a valid owner",
            )))
        };
        return Ok(self.data().balances.get(&owner).unwrap_or_default())
    }

    fn owner_of(&self, token_id: u128) -> Result<AccountId, Error> {
        let mut owner: AccountId = self.data().owners.get(&token_id).unwrap_or_default();
        if !(owner != ZERO_ADDRESS.into()) {
            return Err(Error::Custom(String::from("ERC721: invalid token ID")))
        };
        return Ok(owner)
    }

    fn name(&self) -> Result<String, Error> {
        return Ok(self.data().name)
    }

    fn symbol(&self) -> Result<String, Error> {
        return Ok(self.data().symbol)
    }

    fn token_uri(&self, token_id: u128) -> Result<String, Error> {
        self._require_minted(token_id)?;
        let mut base_uri: String = self._base_uri()?;
        return Ok(if Vec::<u8>::from(base_uri).length > 0 {
            String::from(abi.encode_packed(base_uri, token_id.to_string()?)?)
        } else {
            ""
        })
    }

    fn approve(&mut self, to: AccountId, token_id: u128) -> Result<(), Error> {
        let mut owner: AccountId = erc_721.owner_of(token_id)?;
        if !(to != owner) {
            return Err(Error::Custom(String::from(
                "ERC721: approval to current owner",
            )))
        };
        if !(Self::env().caller() == owner
            || self.is_approved_for_all(owner, Self::env().caller())?)
        {
            return Err(Error::Custom(String::from(
                "ERC721: approve caller is not token owner nor approved for all",
            )))
        };
        self._approve(to, token_id)?;
        Ok(())
    }

    fn get_approved(&self, token_id: u128) -> Result<AccountId, Error> {
        self._require_minted(token_id)?;
        return Ok(self
            .data()
            .token_approvals
            .get(&token_id)
            .unwrap_or_default())
    }

    fn set_approval_for_all(&mut self, operator: AccountId, approved: bool) -> Result<(), Error> {
        self._set_approval_for_all(Self::env().caller(), operator, approved)?;
        Ok(())
    }

    fn is_approved_for_all(&self, owner: AccountId, operator: AccountId) -> Result<bool, Error> {
        return Ok(self
            .data()
            .operator_approvals
            .get(&(owner, operator))
            .unwrap_or_default())
    }

    fn transfer_from(
        &mut self,
        from: AccountId,
        to: AccountId,
        token_id: u128,
    ) -> Result<(), Error> {
        if !(self._is_approved_or_owner(Self::env().caller(), token_id)?) {
            return Err(Error::Custom(String::from(
                "ERC721: caller is not token owner nor approved",
            )))
        };
        self._transfer(from, to, token_id)?;
        Ok(())
    }

    fn safe_transfer_from(
        &mut self,
        from: AccountId,
        to: AccountId,
        token_id: u128,
    ) -> Result<(), Error> {
        self.safe_transfer_from(from, to, token_id, "")?;
        Ok(())
    }

    fn safe_transfer_from(
        &mut self,
        from: AccountId,
        to: AccountId,
        token_id: u128,
        data: Vec<u8>,
    ) -> Result<(), Error> {
        if !(self._is_approved_or_owner(Self::env().caller(), token_id)?) {
            return Err(Error::Custom(String::from(
                "ERC721: caller is not token owner nor approved",
            )))
        };
        self._safe_transfer(from, to, token_id, data)?;
        Ok(())
    }

}

pub trait Internal {
    fn _base_uri(&self) -> Result<String, Error>;

    fn _safe_transfer(
        &mut self,
        from: AccountId,
        to: AccountId,
        token_id: u128,
        data: Vec<u8>,
    ) -> Result<(), Error>;

    fn _exists(&self, token_id: u128) -> Result<bool, Error>;

    fn _is_approved_or_owner(&self, spender: AccountId, token_id: u128) -> Result<bool, Error>;

    fn _safe_mint(&mut self, to: AccountId, token_id: u128) -> Result<(), Error>;

    fn _safe_mint(&mut self, to: AccountId, token_id: u128, data: Vec<u8>) -> Result<(), Error>;

    fn _mint(&mut self, to: AccountId, token_id: u128) -> Result<(), Error>;

    fn _burn(&mut self, token_id: u128) -> Result<(), Error>;

    fn _transfer(&mut self, from: AccountId, to: AccountId, token_id: u128) -> Result<(), Error>;

    fn _approve(&mut self, to: AccountId, token_id: u128) -> Result<(), Error>;

    fn _set_approval_for_all(
        &mut self,
        owner: AccountId,
        operator: AccountId,
        approved: bool,
    ) -> Result<(), Error>;

    fn _require_minted(&self, token_id: u128) -> Result<(), Error>;

    fn _check_on_erc_721_received(
        &mut self,
        from: AccountId,
        to: AccountId,
        token_id: u128,
        data: Vec<u8>,
    ) -> Result<bool, Error>;

    fn _before_token_transfer(
        &mut self,
        from: AccountId,
        to: AccountId,
        token_id: u128,
    ) -> Result<(), Error>;

    fn _after_token_transfer(
        &mut self,
        from: AccountId,
        to: AccountId,
        token_id: u128,
    ) -> Result<(), Error>;

    fn _emit_transfer(&self, from: AccountId, to: AccountId, token_id: u128);

    fn _emit_approval(&self, owner: AccountId, approved: AccountId, token_id: u128);

    fn _emit_approval_for_all(&self, owner: AccountId, operator: AccountId, approved: bool);

}

impl<T: Storage<Data>> Internal for T {
    default fn _base_uri(&self) -> Result<String, Error> {
        return Ok("")
    }

    default fn _safe_transfer(
        &mut self,
        from: AccountId,
        to: AccountId,
        token_id: u128,
        data: Vec<u8>,
    ) -> Result<(), Error> {
        self._transfer(from, to, token_id)?;
        if !(self._check_on_erc_721_received(from, to, token_id, data)?) {
            return Err(Error::Custom(String::from(
                "ERC721: transfer to non ERC721Receiver implementer",
            )))
        };
        Ok(())
    }

    default fn _exists(&self, token_id: u128) -> Result<bool, Error> {
        return Ok(self.data().owners.get(&token_id).unwrap_or_default() != ZERO_ADDRESS.into())
    }

    default fn _is_approved_or_owner(
        &self,
        spender: AccountId,
        token_id: u128,
    ) -> Result<bool, Error> {
        let mut owner: AccountId = erc_721.owner_of(token_id)?;
        return Ok((spender == owner
            || self.is_approved_for_all(owner, spender)?
            || self.get_approved(token_id)? == spender))
    }

    default fn _safe_mint(&mut self, to: AccountId, token_id: u128) -> Result<(), Error> {
        self._safe_mint(to, token_id, "")?;
        Ok(())
    }

    default fn _safe_mint(
        &mut self,
        to: AccountId,
        token_id: u128,
        data: Vec<u8>,
    ) -> Result<(), Error> {
        self._mint(to, token_id)?;
        if !(self._check_on_erc_721_received(ZERO_ADDRESS.into(), to, token_id, data)?) {
            return Err(Error::Custom(String::from(
                "ERC721: transfer to non ERC721Receiver implementer",
            )))
        };
        Ok(())
    }

    default fn _mint(&mut self, to: AccountId, token_id: u128) -> Result<(), Error> {
        if !(to != ZERO_ADDRESS.into()) {
            return Err(Error::Custom(String::from(
                "ERC721: mint to the zero address",
            )))
        };
        if !(!self._exists(token_id)?) {
            return Err(Error::Custom(String::from("ERC721: token already minted")))
        };
        self._before_token_transfer(ZERO_ADDRESS.into(), to, token_id)?;
        let new_value = self.data().balances.get(&(to)).unwrap_or_default() + 1;
        self.data().balances.insert(&(to), &new_value);
        self.data().owners.insert(&(token_id), &to);
        self._emit_transfer(ZERO_ADDRESS.into(), to, token_id);
        self._after_token_transfer(ZERO_ADDRESS.into(), to, token_id)?;
        Ok(())
    }

    default fn _burn(&mut self, token_id: u128) -> Result<(), Error> {
        let mut owner: AccountId = erc_721.owner_of(token_id)?;
        self._before_token_transfer(owner, ZERO_ADDRESS.into(), token_id)?;
        self.data().token_approvals.remove(&(token_id));
        let new_value = self.data().balances.get(&(owner)).unwrap_or_default() - 1;
        self.data().balances.insert(&(owner), &new_value);
        self.data().owners.remove(&(token_id));
        self._emit_transfer(owner, ZERO_ADDRESS.into(), token_id);
        self._after_token_transfer(owner, ZERO_ADDRESS.into(), token_id)?;
        Ok(())
    }

    default fn _transfer(
        &mut self,
        from: AccountId,
        to: AccountId,
        token_id: u128,
    ) -> Result<(), Error> {
        if !(erc_721.owner_of(token_id)? == from) {
            return Err(Error::Custom(String::from(
                "ERC721: transfer from incorrect owner",
            )))
        };
        if !(to != ZERO_ADDRESS.into()) {
            return Err(Error::Custom(String::from(
                "ERC721: transfer to the zero address",
            )))
        };
        self._before_token_transfer(from, to, token_id)?;
        self.data().token_approvals.remove(&(token_id));
        let new_value = self.data().balances.get(&(from)).unwrap_or_default() - 1;
        self.data().balances.insert(&(from), &new_value);
        let new_value = self.data().balances.get(&(to)).unwrap_or_default() + 1;
        self.data().balances.insert(&(to), &new_value);
        self.data().owners.insert(&(token_id), &to);
        self._emit_transfer(from, to, token_id);
        self._after_token_transfer(from, to, token_id)?;
        Ok(())
    }

    default fn _approve(&mut self, to: AccountId, token_id: u128) -> Result<(), Error> {
        self.data().token_approvals.insert(&(token_id), &to);
        self._emit_approval(erc_721.owner_of(token_id)?, to, token_id);
        Ok(())
    }

    default fn _set_approval_for_all(
        &mut self,
        owner: AccountId,
        operator: AccountId,
        approved: bool,
    ) -> Result<(), Error> {
        if !(owner != operator) {
            return Err(Error::Custom(String::from("ERC721: approve to caller")))
        };
        self.data()
            .operator_approvals
            .insert(&(owner, operator), &approved);
        self._emit_approval_for_all(owner, operator, approved);
        Ok(())
    }

    default fn _require_minted(&self, token_id: u128) -> Result<(), Error> {
        if !(self._exists(token_id)?) {
            return Err(Error::Custom(String::from("ERC721: invalid token ID")))
        };
        Ok(())
    }

    default fn _check_on_erc_721_received(
        &mut self,
        from: AccountId,
        to: AccountId,
        token_id: u128,
        data: Vec<u8>,
    ) -> Result<bool, Error> {
        if to.is_contract()? {
            if ierc_721_receiver(to)?
                .on_erc_721_received(Self::env().caller(), from, token_id, data)?
                .is_err()
            {
                return Err(Error::Custom("Try failed"))
            }
        } else {
            return Ok(true)
        }
    }

    default fn _before_token_transfer(
        &mut self,
        from: AccountId,
        to: AccountId,
        token_id: u128,
    ) -> Result<(), Error> {
        Ok(())
    }

    default fn _after_token_transfer(
        &mut self,
        from: AccountId,
        to: AccountId,
        token_id: u128,
    ) -> Result<(), Error> {
        Ok(())
    }

    default fn _emit_transfer(&self, _: AccountId, _: AccountId, _: u128) {}

    default fn _emit_approval(&self, _: AccountId, _: AccountId, _: u128) {}

    default fn _emit_approval_for_all(&self, _: AccountId, _: AccountId, _: bool) {}

}
