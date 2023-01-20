#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

// Generated with Sol2Ink v2.0.0-beta
// https://github.com/727-Ventures/sol2ink


pub enum Error {
    Custom(String),
}


pub fn try_add(&self, a: u128, b: u128) -> Result<(bool, u128), Error> {
    let mut c: u128 = a + b;
    if c < a {
        return Ok((false, 0))
    }
    return Ok((true, c))
}

pub fn try_sub(&self, a: u128, b: u128) -> Result<(bool, u128), Error> {
    if b > a {
        return Ok((false, 0))
    }
    return Ok((true, a - b))
}

pub fn try_mul(&self, a: u128, b: u128) -> Result<(bool, u128), Error> {
    if a == 0 {
        return Ok((true, 0))
    }
    let mut c: u128 = a * b;
    if c / a != b {
        return Ok((false, 0))
    }
    return Ok((true, c))
}

pub fn try_div(&self, a: u128, b: u128) -> Result<(bool, u128), Error> {
    if b == 0 {
        return Ok((false, 0))
    }
    return Ok((true, a / b))
}

pub fn try_mod(&self, a: u128, b: u128) -> Result<(bool, u128), Error> {
    if b == 0 {
        return Ok((false, 0))
    }
    return Ok((true, a % b))
}

pub fn add(&self, a: u128, b: u128) -> Result<u128, Error> {
    return Ok(a + b)
}

pub fn sub(&self, a: u128, b: u128) -> Result<u128, Error> {
    return Ok(a - b)
}

pub fn mul(&self, a: u128, b: u128) -> Result<u128, Error> {
    return Ok(a * b)
}

pub fn div(&self, a: u128, b: u128) -> Result<u128, Error> {
    return Ok(a / b)
}

pub fn mod_is_rust_keyword(&self, a: u128, b: u128) -> Result<u128, Error> {
    return Ok(a % b)
}

pub fn sub(&self, a: u128, b: u128, error_message: String) -> Result<u128, Error> {
    if !(b <= a) {
        return Err(Error::Custom(error_message))
    };
    return Ok(a - b)
}

pub fn div(&self, a: u128, b: u128, error_message: String) -> Result<u128, Error> {
    if !(b > 0) {
        return Err(Error::Custom(error_message))
    };
    return Ok(a / b)
}

pub fn mod_is_rust_keyword(&self, a: u128, b: u128, error_message: String) -> Result<u128, Error> {
    if !(b > 0) {
        return Err(Error::Custom(error_message))
    };
    return Ok(a % b)
}

