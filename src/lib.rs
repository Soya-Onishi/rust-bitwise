#[cfg(test)]
mod test;
pub mod ops;

extern crate num_bigint;

use num_bigint::{Sign, BigInt};

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

    pub fn concat(bits: Vec<&Bit>) -> Bit {
        assert!(bits.len() >= 2);

        let value = bits[1..].iter().fold(bits[0].value().clone(), |acc, &bit| {
            (acc << bit.length()) | bit.value()
        });

        let length = bits.iter().fold(0, |acc, &bit| {
           acc + bit.length()
        });

        Bit { value, length }
    }

    pub fn zero_ext(&self, length: usize) -> Bit {
        assert!(self.length() <= length);

        Bit { value: self.value().clone(), length }
    }

    pub fn sign_ext(&self, length: usize) -> Bit {
        assert!(self.length() <= length);

        let top_is_zero = || {
            let zero = BigInt::new(Sign::NoSign, vec![0]);
            let truncated = self.truncate(self.length() - 1);
            let value = truncated.value();

            value == &zero
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

        Bit { value, length }
    }
}

pub trait BitConstructor<T> {
    fn new(value: T) -> Bit;
}

impl BitConstructor<u32> for Bit {
    fn new(value: u32) -> Bit {
        let value = BigInt::new(Sign::Plus, vec![value]);
        let length = 32;

        Bit { value, length }
    }
}

impl BitConstructor<(u32, usize)> for Bit {
    fn new((value, length): (u32, usize)) -> Bit {
        let mut at_least = 1;
        for shamt in 1..32 {
            if (value >> shamt) & 1 == 1 {
                at_least = shamt + 1;
            }
        }

        assert!(length >= at_least as usize);

        let value = BigInt::new(Sign::Plus, vec![value]);
        Bit { value, length }
    }
}

impl BitConstructor<BigInt> for Bit {
    fn new(value: BigInt) -> Bit {
        assert_eq!(value.sign(), Sign::Plus);
        let (_, bytes) = value.to_bytes_be();
        let length = bytes.len() * 8;

        Bit{ value, length }
    }
}

impl BitConstructor<(BigInt, usize)> for Bit {
    fn new((value, length): (BigInt, usize)) -> Bit {
        assert_eq!(value.sign(), Sign::Plus);

        let (_, bytes) = value.to_bytes_be();
        let top = &bytes[0];
        let mut at_least = 1;

        for shamt in 1..8 {
            if(top >> shamt) & 1 == 1 {
                at_least = shamt + 1
            }
        }

        let at_least_length = (bytes.len() - 1) * 8 + at_least;
        assert!(at_least_length <= length);

        return Bit { value, length }
    }
}

pub trait Truncate<T> {
    fn truncate(&self, range: T) -> Self;
}

impl Truncate<usize> for Bit {
    fn truncate(&self, index: usize) -> Bit {
        assert!(index < self.length);

        let mask = &BigInt::new(Sign::Plus, vec![1]);
        let value = (self.value() >> index) & mask;

        Bit { value, length: 1 }
    }
}

impl Truncate<(usize, usize)> for Bit {
    fn truncate(&self, (upper, lower): (usize, usize)) -> Bit {
        assert!(upper >= lower);
        assert!(upper < self.length());

        let length = upper - lower + 1;
        let mask = (BigInt::new(Sign::Plus, vec![1]) << length) - 1;
        let value = (self.value() >> lower) & mask;

        Bit { value, length }
    }
}