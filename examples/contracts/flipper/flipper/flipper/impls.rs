// Generated with Sol2Ink v1.1.0
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
    pub value: bool,
    pub _reserved: Option<()>,
}


impl<T: Storage<Data>> flipper for T {
    ///A message that can be called on instantiated contracts.
    ///This one flips the value of the stored `bool` from `true`
    ///to `false` and vice versa.
    fn flip(&mut self) -> Result<(), Error> {
        self.data().value = !value;
        Ok(())
    }

    ///Simply returns the current value of our `bool`.
    fn get(&self) -> Result<bool, Error> {
        return Ok(self.data().value)
    }

}

pub trait Internal {}

impl<T: Storage<Data>> Internal for T {}
