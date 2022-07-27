#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

// Generated with Sol2Ink v0.4.1
// https://github.com/Supercolony-net/sol2ink

///example.sol
#[brush::contract]
pub mod example {
    use brush::traits::{
        AccountId,
        AcountIdExt,
        ZERO_ADDRESS,
    };

    #[derive(Debug, Encode, Decode, PartialEq)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        Custom(String),
    }

    ///Constants
    pub const bad_state: State = State.Zombie;
    pub const first_pid: i32 = 1;

    ///Process state
    pub enum State {
        Running,
        Sleeping,
        Waiting,
        Stopped,
        Zombie,
        StateCount,
    }

    ///cards
    pub enum suit {
        club,
        diamonds,
        hearts,
        spades,
    }

    pub enum value {
        two,
        three,
        four,
        five,
        six,
        seven,
        eight,
        nine,
        ten,
        jack,
        queen,
        king,
        ace,
    }

    #[derive(Default, Encode, Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct card {
        v: value,
        s: suit,
    }

    #[ink(storage)]
    #[derive(Default, SpreadAllocate)]
    pub struct example {
        ///Variables in contract storage
        state: State,
        pid: i32,
        reaped: u32,
    }

    impl example {
        ///Our constructors
        #[ink(constructor)]
        pub fn new(pid: i32) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {
                // Set contract storage
                instance.pid = pid;
                self.reaped = 3;
            })
        }

        ///Reading but not writing contract storage means function
        ///can be declared view
        #[ink(message)]
        pub fn is_zombie_reaper(&self) -> Result<bool, Error> {
            // must be pid 1 and not zombie ourselves *
            return Ok((pid == self.first_pid && self.state != state.zombie))
        }

        ///Returning a constant does not access storage at all, so
        ///function can be declared pure
        #[ink(message)]
        pub fn systemd_pid(&self) -> Result<u32, Error> {
            // Note that cast is required to change sign from
            // int32 to uint32
            return Ok((self.first_pid as u32))
        }

        ///Convert celcius to fahrenheit
        #[ink(message)]
        pub fn celcius_2_fahrenheit(&self, celcius: i32) -> Result<i32, Error> {
            let fahrenheit: i32 = celcius * 9 / 5 + 32;
            return Ok(fahrenheit)
        }

        ///Convert fahrenheit to celcius
        #[ink(message)]
        pub fn fahrenheit_2_celcius(&self, fahrenheit: i32) -> Result<i32, Error> {
            return Ok((fahrenheit - 32) * 5 / 9)
        }

        ///is this number a power-of-two
        #[ink(message)]
        pub fn is_power_of_2(&self, n: u128) -> Result<bool, Error> {
            return Ok(n != 0 && (n & (n - 1)) == 0)
        }

        ///calculate the population count (number of set bits) using Brian Kerningham's way
        #[ink(message)]
        pub fn population_count(&self, n: u128) -> Result<u128, Error> {
            // Sol2Ink Not Implemented yet: for(count = 0; n != 0; count++){
            n &= (n - 1);
            // Sol2Ink Not Implemented yet End Block here
        }

        ///calculate the power of base to exp
        #[ink(message)]
        pub fn power(&self, base: u128, exp: u128) -> Result<u128, Error> {
            return Ok(base.pow(exp as u32))
        }

        ///returns true if the address is 0
        #[ink(message)]
        pub fn is_address_zero(&self, a: AccountId) -> Result<bool, Error> {
            return Ok(a.is_zero())
        }

        ///reverse the bytes in an array of 8 (endian swap)
        ///TODO: parse this
        ///function byte8reverse(bytes8 input) public pure returns (bytes8 out) {
        ///out = ((input << 56) & hex"ff00_0000_0000_0000") |
        ///((input << 40) & hex"00ff_0000_0000_0000") |
        ///((input << 24) & hex"0000_ff00_0000_0000") |
        ///((input <<  8) & hex"0000_00ff_0000_0000") |
        ///((input >>  8) & hex"0000_0000_ff00_0000") |
        ///((input >> 24) & hex"0000_0000_00ff_0000") |
        ///((input >> 40) & hex"0000_0000_0000_ff00") |
        ///((input >> 56) & hex"0000_0000_0000_00ff");
        ///}
        ///This mocks a pid state
        fn _get_pid_state(&self, pid: u64) -> Result<State, Error> {
            let n: u64 = 8;
            // Sol2Ink Not Implemented yet: for(uint16 i = 1; i < 10; ++i){
            if (i_ % _3) == 0 {
                n *= pid / (i as u64);
            } else {
                n /= 3;
            }
            // Sol2Ink Not Implemented yet End Block here
            return Ok(self._state(n_ % _uint_64(state.state_count))?)
        }

        ///Overloaded function with different return value!
        fn _get_pid_state(&self) -> Result<u32, Error> {
            return Ok(self.reaped)
        }

        #[ink(message)]
        pub fn reap_processes(&mut self) -> Result<(), Error> {
            let n: u32 = 0;
            // Sol2Ink Not Implemented yet: while (n < 100) {
            if self._get_pid_state(n)? == state.zombie {
                // reap!
                self.reaped += 1;
            }
            // Sol2Ink Not Implemented yet: n++;
            // Sol2Ink Not Implemented yet End Block here
            Ok(())
        }

        #[ink(message)]
        pub fn run_queue(&self) -> Result<u16, Error> {
            let count: u16 = 0;
            // no initializer means its 0.
            let n: u32 = 0;
            // Sol2Ink Not Implemented yet: do {
            if self._get_pid_state(n)? == state.waiting {
                // Sol2Ink Not Implemented yet: count++;
            }
            // Sol2Ink Not Implemented yet End Block here
            // while (++n < 1000);
            return Ok(count)
        }

        ///card card1 = card(value.two, suit.club);
        ///card card2 = card({s: suit.club, v: value.two});
        ///This function does a lot of copying
        #[ink(message)]
        pub fn set_card_1(&mut self, c: card) -> Result<card, Error> {
            previous = card_1;
            card_1 = c;
        }

        ///return the ace of spades
        #[ink(message)]
        pub fn ace_of_spaces(&self) -> Result<card, Error> {
            // return card({s: suit.spades, v: value.ace });
        }

        ///score card
        #[ink(message)]
        pub fn score_card(&self, c: card) -> Result<u32, Error> {
            if c.s == suit.hearts {
                if c.v == value.ace {
                    score = 14;
                }
                if c.v == value.king {
                    score = 13;
                }
                if c.v == value.queen {
                    score = 12;
                }
                if c.v == value.jack {
                    score = 11;
                }
            }
            // all others score 0
        }

    }
}
