#[cfg(test)]
mod test;
pub mod ops;
pub mod errors;

extern crate num_bigint;

use num_bigint::{Sign, BigInt};
use errors::Error;

#[derive(Debug, Clone)]
pub struct Bit {
    value: BigInt,
    length: usize,
}

impl Bit {
    pub fn value(&self) -> &BigInt {
        &self.value
    }

    pub fn length(&self) -> usize {
        self.length
    }

    pub fn concat(bits: Vec<&Bit>) -> Result<Bit, Error> {
        assert!(bits.len() >= 2);

        if bits.len() < 2 { Err(Error::NotEnoughLengthToConcat(bits.len())) }
        else {
            let value = bits[1..].iter().fold(bits[0].value().clone(), |acc, &bit| {
                (acc << bit.length()) | bit.value()
            });

            let length = bits.iter().fold(0, |acc, &bit| {
                acc + bit.length()
            });

            Ok(Bit { value, length })
        }
    }

    pub fn zero_ext(&self, length: usize) -> Result<Bit, Error> {
        assert!(self.length() <= length);

        if (self.length() > length) { Err(Error::NotEnoughLengthToExt(self.length(), length)) }
        else { Ok(Bit { value: self.value().clone(), length }) }
    }

    pub fn sign_ext(&self, length: usize) -> Result<Bit, Error> {
        if self.length() > length { Err(Error::NotEnoughLengthToExt(self.length(), length)) }
        else {
            let top_is_zero = || {
                let zero = BigInt::new(Sign::NoSign, vec![0]);
                let truncated = self.truncate(self.length() - 1);
                let value = match truncated {
                    Ok(bit) => bit.value().clone(),
                    Err(err) => panic!("implementation error occured"),
                };

                &value == &zero
            };

            let is_no_length_diff = || {
                (length - self.length()) == 0
            };

            let mask =
                if top_is_zero() || is_no_length_diff() {
                    BigInt::new(Sign::NoSign, vec![0])
                } else {
                    let diff = length - self.length();
                    let allone = (BigInt::new(Sign::Plus, vec![1]) << diff) - 1;
                    allone << self.length()
                };

            let value = mask | self.value();

            Ok(Bit { value, length })
        }
    }

    pub fn as_u32(&self) -> Result<u32, Error> {
        if self.length() > 32 { Err(Error::TooLongToCast(32, self.length())) }
        else {
            let value = self.value();
            let (_, bytes) = value.to_bytes_le();

            let value = bytes.iter().zip(0..4).fold(0, |acc, (&byte, index)| {
                acc + ((byte as u32) << (index * 8))
            });

            Ok(value)
        }
    }

    pub fn as_u8(&self) -> Result<u8, Error> {
        if self.length() > 8 { Err(Error::TooLongToCast(8, self.length())) }
        else {
            let (_, value) = self.value().to_bytes_be();
            Ok(value[0])
        }
    }
}

pub trait BitConstructor<T>: Sized {
    fn new(value: T) -> Self;
    fn new_with_length(value: T, length: usize) -> Result<Self, Error>;
}

impl BitConstructor<u32> for Bit {
    fn new(value: u32) -> Bit {
        let value = BigInt::new(Sign::Plus, vec![value]);
        let length = 32;

        Bit { value, length }
    }

    fn new_with_length(value: u32, length: usize) -> Result<Bit, Error> {
        let mut at_least = 1;
        for shamt in 1..32 {
            if (value >> shamt) & 1 == 1 {
                at_least = shamt + 1;
            }
        }

        let at_least = at_least as usize;
        if length < at_least { Err(Error::TooShortToConstruct(at_least, length)) }
        else {
            let value = BigInt::new(Sign::Plus, vec![value]);
            Ok(Bit { value, length })
        }
    }
}

impl BitConstructor<BigInt> for Bit {
    fn new(value: BigInt) -> Bit {
        let value= minus_into_plus(value);

        let (_, bytes) = value.to_bytes_be();
        let length = bytes.len() * 8;
        Bit{ value, length }

    }

    fn new_with_length(value: BigInt, length: usize) -> Result<Bit, Error> {
        let value = minus_into_plus(value);

        let (_, bytes) = value.to_bytes_be();
        let top = &bytes[0];
        let mut at_least = 1;

        for shamt in 1..8 {
            if(top >> shamt) & 1 == 1 {
                at_least = shamt + 1
            }
        }

        let at_least_length = (bytes.len() - 1) * 8 + at_least;

        if at_least_length > length { Err(Error::TooShortToConstruct(at_least_length, length)) }
        else { Ok(Bit { value, length }) }
    }
}

fn minus_into_plus(value: BigInt) -> BigInt {
    match value.sign() {
        Sign::Plus | Sign::NoSign => value,
        Sign::Minus => !value + 1,
    }
}

pub trait Truncate<T>: Sized {
    fn truncate(&self, range: T) -> Result<Self, Error>;
}

impl Truncate<usize> for Bit {
    fn truncate(&self, index: usize) -> Result<Bit, Error> {
        if index >= self.length { return Err(Error::OverBitLength(self.length, index)) }

        let mask = &BigInt::new(Sign::Plus, vec![1]);
        let value = (self.value() >> index) & mask;

        Ok(Bit { value, length: 1 })
    }
}

impl Truncate<(usize, usize)> for Bit {
    fn truncate(&self, (upper, lower): (usize, usize)) -> Result<Bit, Error> {
        if upper < lower { return Err(Error::UpperLowerThanLower(upper, lower)) }
        if upper >= self.length { return Err(Error::OverBitLength(self.length, upper)) }

        let length = upper - lower + 1;
        let mask = (BigInt::new(Sign::Plus, vec![1]) << length) - 1;
        let value = (self.value() >> lower) & mask;

        Ok(Bit { value, length })
    }
}