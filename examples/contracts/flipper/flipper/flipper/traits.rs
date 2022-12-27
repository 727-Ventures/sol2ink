// Generated with Sol2Ink v1.1.0
// https://github.com/Supercolony-net/sol2ink

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
pub type flipperRef = dyn flipper;

#[openbrush::trait_definition]
pub trait flipper {
    ///A message that can be called on instantiated contracts.
    ///This one flips the value of the stored `bool` from `true`
    ///to `false` and vice versa.
    #[ink(message)]
    fn flip(&mut self) -> Result<(), Error>;

    ///Simply returns the current value of our `bool`.
    #[ink(message)]
    fn get(&self) -> Result<bool, Error>;

    #[ink(message)]
    fn value(&self) -> bool;

}
