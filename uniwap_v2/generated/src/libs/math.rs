#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]


// Generated with Sol2Ink v2.1.0
// https://github.com/Brushfam/sol2ink

/// a library for performing various math operations

pub enum Error {
    Custom(String),
}


pub fn min(&self, x: u128, y: u128) -> Result<u128, Error> {
    let mut z = Default::default();
    z = if x < y { x } else { y };
    Ok(z)
}

/// babylonian method (https://en.wikipedia.org/wiki/Methods_of_computing_square_roots#Babylonian_method)
pub fn sqrt(&self, y: u128) -> Result<u128, Error> {
    let mut z = Default::default();
    if y > 3 {
        z = y;
        let mut x: u128 = y / 2 + 1;
        while x < z {
            z = x;
            x = (y / x + x) / 2;
        }
    } else if y != 0 {
        z = 1;
    }
    Ok(z)
}

