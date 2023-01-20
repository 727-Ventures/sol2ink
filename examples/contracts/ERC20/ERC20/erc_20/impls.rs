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
    pub balances: Mapping<AccountId, u128>,
    pub allowances: Mapping<(AccountId, AccountId), u128>,
    pub total_supply: u128,
    pub name: String,
    pub symbol: String,
    pub _reserved: Option<()>,
}


impl<T: Storage<Data>> ERC20 for T {
    fn name(&self) -> Result<String, Error> {
        return Ok(self.data().name)
    }

    fn symbol(&self) -> Result<String, Error> {
        return Ok(self.data().symbol)
    }

    fn decimals(&self) -> Result<u8, Error> {
        return Ok(18)
    }

    fn total_supply(&self) -> Result<u128, Error> {
        return Ok(self.data().total_supply)
    }

    fn balance_of(&self, account: AccountId) -> Result<u128, Error> {
        return Ok(self.data().balances[account])
    }

    fn transfer(&mut self, to: AccountId, amount: u128) -> Result<bool, Error> {
        let mut owner: AccountId = Self::env().caller();
        self._transfer(owner, to, amount)?;
        return Ok(true)
    }

    fn allowance(&self, owner: AccountId, spender: AccountId) -> Result<u128, Error> {
        return Ok(self
            .data()
            .allowances
            .get(&(owner, spender))
            .unwrap_or_default())
    }

    fn approve(&mut self, spender: AccountId, amount: u128) -> Result<bool, Error> {
        let mut owner: AccountId = Self::env().caller();
        self._approve(owner, spender, amount)?;
        return Ok(true)
    }

    fn transfer_from(
        &mut self,
        from: AccountId,
        to: AccountId,
        amount: u128,
    ) -> Result<bool, Error> {
        let mut spender: AccountId = Self::env().caller();
        self._spend_allowance(from, spender, amount)?;
        self._transfer(from, to, amount)?;
        return Ok(true)
    }

    fn increase_allowance(&mut self, spender: AccountId, added_value: u128) -> Result<bool, Error> {
        let mut owner: AccountId = Self::env().caller();
        self._approve(
            owner,
            spender,
            self.allowance(owner, spender)? + added_value,
        )?;
        return Ok(true)
    }

    fn decrease_allowance(
        &mut self,
        spender: AccountId,
        subtracted_value: u128,
    ) -> Result<bool, Error> {
        let mut owner: AccountId = Self::env().caller();
        let mut current_allowance: u128 = self.allowance(owner, spender)?;
        if !(current_allowance >= subtracted_value) {
            return Err(Error::Custom(String::from(
                "ERC20: decreased allowance below zero",
            )))
        };
        self._approve(owner, spender, current_allowance - subtracted_value)?;
        return Ok(true)
    }

}

pub trait Internal {
    fn _transfer(&mut self, from: AccountId, to: AccountId, amount: u128) -> Result<(), Error>;

    fn _mint(&mut self, account: AccountId, amount: u128) -> Result<(), Error>;

    fn _burn(&mut self, account: AccountId, amount: u128) -> Result<(), Error>;

    fn _approve(&mut self, owner: AccountId, spender: AccountId, amount: u128)
        -> Result<(), Error>;

    fn _spend_allowance(
        &mut self,
        owner: AccountId,
        spender: AccountId,
        amount: u128,
    ) -> Result<(), Error>;

    fn _before_token_transfer(
        &mut self,
        from: AccountId,
        to: AccountId,
        amount: u128,
    ) -> Result<(), Error>;

    fn _after_token_transfer(
        &mut self,
        from: AccountId,
        to: AccountId,
        amount: u128,
    ) -> Result<(), Error>;

    fn _emit_transfer(&self, from: AccountId, to: AccountId, value: u128);

    fn _emit_approval(&self, owner: AccountId, spender: AccountId, value: u128);

}

impl<T: Storage<Data>> Internal for T {
    default fn _transfer(
        &mut self,
        from: AccountId,
        to: AccountId,
        amount: u128,
    ) -> Result<(), Error> {
        if !(from != ZERO_ADDRESS.into()) {
            return Err(Error::Custom(String::from(
                "ERC20: transfer from the zero address",
            )))
        };
        if !(to != ZERO_ADDRESS.into()) {
            return Err(Error::Custom(String::from(
                "ERC20: transfer to the zero address",
            )))
        };
        self._before_token_transfer(from, to, amount)?;
        let mut from_balance: u128 = self.data().balances[from];
        if !(from_balance >= amount) {
            return Err(Error::Custom(String::from(
                "ERC20: transfer amount exceeds balance",
            )))
        };
        self.data().balances[from] = from_balance - amount;
        self.data().balances[to] += amount;
        self._emit_transfer(from, to, amount);
        self._after_token_transfer(from, to, amount)?;
        Ok(())
    }

    default fn _mint(&mut self, account: AccountId, amount: u128) -> Result<(), Error> {
        if !(account != ZERO_ADDRESS.into()) {
            return Err(Error::Custom(String::from(
                "ERC20: mint to the zero address",
            )))
        };
        self._before_token_transfer(ZERO_ADDRESS.into(), account, amount)?;
        self.data().total_supply += amount;
        self.data().balances[account] += amount;
        self._emit_transfer(ZERO_ADDRESS.into(), account, amount);
        self._after_token_transfer(ZERO_ADDRESS.into(), account, amount)?;
        Ok(())
    }

    default fn _burn(&mut self, account: AccountId, amount: u128) -> Result<(), Error> {
        if !(account != ZERO_ADDRESS.into()) {
            return Err(Error::Custom(String::from(
                "ERC20: burn from the zero address",
            )))
        };
        self._before_token_transfer(account, ZERO_ADDRESS.into(), amount)?;
        let mut account_balance: u128 = self.data().balances[account];
        if !(account_balance >= amount) {
            return Err(Error::Custom(String::from(
                "ERC20: burn amount exceeds balance",
            )))
        };
        self.data().balances[account] = account_balance - amount;
        self.data().total_supply -= amount;
        self._emit_transfer(account, ZERO_ADDRESS.into(), amount);
        self._after_token_transfer(account, ZERO_ADDRESS.into(), amount)?;
        Ok(())
    }

    default fn _approve(
        &mut self,
        owner: AccountId,
        spender: AccountId,
        amount: u128,
    ) -> Result<(), Error> {
        if !(owner != ZERO_ADDRESS.into()) {
            return Err(Error::Custom(String::from(
                "ERC20: approve from the zero address",
            )))
        };
        if !(spender != ZERO_ADDRESS.into()) {
            return Err(Error::Custom(String::from(
                "ERC20: approve to the zero address",
            )))
        };
        self.data().allowances.insert(&(owner, spender), &amount);
        self._emit_approval(owner, spender, amount);
        Ok(())
    }

    default fn _spend_allowance(
        &mut self,
        owner: AccountId,
        spender: AccountId,
        amount: u128,
    ) -> Result<(), Error> {
        let mut current_allowance: u128 = self.allowance(owner, spender)?;
        if current_allowance != type_of(u128)?.max {
            if !(current_allowance >= amount) {
                return Err(Error::Custom(String::from("ERC20: insufficient allowance")))
            };
            self._approve(owner, spender, current_allowance - amount)?;
        }
        Ok(())
    }

    default fn _before_token_transfer(
        &mut self,
        from: AccountId,
        to: AccountId,
        amount: u128,
    ) -> Result<(), Error> {
        Ok(())
    }

    default fn _after_token_transfer(
        &mut self,
        from: AccountId,
        to: AccountId,
        amount: u128,
    ) -> Result<(), Error> {
        Ok(())
    }

    default fn _emit_transfer(&self, _: AccountId, _: AccountId, _: u128) {}

    default fn _emit_approval(&self, _: AccountId, _: AccountId, _: u128) {}

}
