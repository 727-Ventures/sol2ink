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

pub enum State {
    Running,
    Sleeping,
    Waiting,
    Stopped,
    Zombie,
    StateCount,
}

pub enum Suit {
    Club,
    Diamonds,
    Hearts,
    Spades,
}

pub enum Value {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}


#[derive(Default, Encode, Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct Card {
    v: value,
    s: suit,
}


#[openbrush::wrapper]
pub type exampleRef = dyn example;

#[openbrush::trait_definition]
pub trait example {
    #[ink(message)]
    fn is_zombie_reaper(&self) -> Result<bool, Error>;

    #[ink(message)]
    fn systemd_pid(&self) -> Result<u32, Error>;

    #[ink(message)]
    fn celcius_2_fahrenheit(&self, celcius: i32) -> Result<i32, Error>;

    #[ink(message)]
    fn fahrenheit_2_celcius(&self, fahrenheit: i32) -> Result<i32, Error>;

    #[ink(message)]
    fn is_power_of_2(&self, n: u128) -> Result<bool, Error>;

    #[ink(message)]
    fn population_count(&self, n: u128) -> Result<u128, Error>;

    #[ink(message)]
    fn power(&self, base: u128, exp: u128) -> Result<u128, Error>;

    #[ink(message)]
    fn is_address_zero(&self, a: AccountId) -> Result<bool, Error>;

    #[ink(message)]
    fn byte_8_reverse(&self, input: [u8; 8]) -> Result<[u8; 8], Error>;

    #[ink(message)]
    fn reap_processes(&mut self) -> Result<(), Error>;

    #[ink(message)]
    fn run_queue(&self) -> Result<u16, Error>;

    #[ink(message)]
    fn set_card_1(&mut self, c: card) -> Result<card, Error>;

    #[ink(message)]
    fn ace_of_spaces(&self) -> Result<card, Error>;

    #[ink(message)]
    fn score_card(&self, c: card) -> Result<u32, Error>;

    #[ink(message)]
    fn state(&self) -> state;

    #[ink(message)]
    fn pid(&self) -> i32;

    #[ink(message)]
    fn reaped(&self) -> u32;

    #[ink(message)]
    fn card_1(&self) -> card;

    #[ink(message)]
    fn card_2(&self) -> card;

}
