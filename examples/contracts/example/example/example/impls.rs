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
    pub state: state,
    pub pid: i32,
    pub reaped: u32,
    pub card_1: card,
    pub card_2: card,
    pub _reserved: Option<()>,
}


impl<T: Storage<Data>> example for T {
    fn is_zombie_reaper(&self) -> Result<bool, Error> {
        return Ok((self.data().pid == FIRST_PID && self.data().state != state.zombie))
    }

    fn systemd_pid(&self) -> Result<u32, Error> {
        return Ok(u32::from(FIRST_PID))
    }

    fn celcius_2_fahrenheit(&self, celcius: i32) -> Result<i32, Error> {
        let mut fahrenheit: i32 = celcius * 9 / 5 + 32;
        return Ok(fahrenheit)
    }

    fn fahrenheit_2_celcius(&self, fahrenheit: i32) -> Result<i32, Error> {
        return Ok((fahrenheit - 32) * 5 / 9)
    }

    fn is_power_of_2(&self, n: u128) -> Result<bool, Error> {
        return Ok(n != 0 && (n & (n - 1)) == 0)
    }

    fn population_count(&self, n: u128) -> Result<u128, Error> {
        let mut count = Default::default();
        count = 0;
        while n != 0 {
            n &= (n - 1);
            count += 1;
        }
        Ok(count)
    }

    fn power(&self, base: u128, exp: u128) -> Result<u128, Error> {
        return Ok(base.pow(exp))
    }

    fn is_address_zero(&self, a: AccountId) -> Result<bool, Error> {
        return Ok(a == ZERO_ADDRESS.into())
    }

    fn byte_8_reverse(&self, input: [u8; 8]) -> Result<[u8; 8], Error> {
        let mut out = Default::default();
        out = ((input << 56) & &hex::decode("ff00000000000000"))
            | ((input << 40) & &hex::decode("00ff000000000000"))
            | ((input << 24) & &hex::decode("0000ff0000000000"))
            | ((input << 8) & &hex::decode("000000ff00000000"))
            | ((input >> 8) & &hex::decode("00000000ff000000"))
            | ((input >> 24) & &hex::decode("0000000000ff0000"))
            | ((input >> 40) & &hex::decode("000000000000ff00"))
            | ((input >> 56) & &hex::decode("00000000000000ff"));
        Ok(out)
    }

    fn reap_processes(&mut self) -> Result<(), Error> {
        let mut n: u32 = 0;
        while n < 100 {
            if self._get_pid_state(n)? == state.zombie {
                self.data().reaped += 1;
            }
            n += 1;
        }
        Ok(())
    }

    fn run_queue(&self) -> Result<u16, Error> {
        let mut count: u16 = 0;
        let mut n: u32 = 0;
        loop {
            if self._get_pid_state(n)? == state.waiting {
                count += 1;
            }
            if !n += 1 < 1000 {
                break
            }
        }
        return Ok(count)
    }

    fn set_card_1(&mut self, c: card) -> Result<card, Error> {
        let mut previous = Default::default();
        previous = self.data().card_1;
        self.data().card_1 = c;
        Ok(previous)
    }

    fn ace_of_spaces(&self) -> Result<card, Error> {
        return Ok(card {
            s: suit.spades,
            v: value.ace,
        })
    }

    fn score_card(&self, c: card) -> Result<u32, Error> {
        let mut score = Default::default();
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
        Ok(score)
    }

}

pub trait Internal {
    fn _get_pid_state(&self, pid: u64) -> Result<state, Error>;

    fn _get_pid_state(&self) -> Result<u32, Error>;

}

impl<T: Storage<Data>> Internal for T {
    default fn _get_pid_state(&self, pid: u64) -> Result<state, Error> {
        let mut n: u64 = 8;
        let mut i: u16 = 1;
        while i < 10 {
            if (i % 3) == 0 {
                n *= pid / u64::from(i);
            } else {
                n /= 3;
            }
            i += 1;
        }
        return Ok(state(n % u64::from(state.state_count))?)
    }

    default fn _get_pid_state(&self) -> Result<u32, Error> {
        return Ok(self.data().reaped)
    }

}
