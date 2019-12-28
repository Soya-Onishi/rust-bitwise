pub use std::ops::{Add, Sub, Mul, Div, Index, Shr, Shl};
pub use std::cmp::{PartialEq, PartialOrd, Eq, Ordering};
use std::cmp::max;
use num_bigint::{Sign, BigInt};
use super::Bit;

impl Add for Bit {
    type Output = Bit;

    fn add(self, other: Bit) -> Bit {
        binops(self, other, |a, b| a.value() + b.value())
    }
}

impl Sub for Bit {
    type Output = Bit;

    fn sub(self, other: Bit) -> Bit {
        binops(self, other, |a, b| a.value - b.value)
    }
}

impl Shl<usize> for Bit {
    type Output = Bit;

    fn shl(self, shamt: usize) -> Bit {
        let length = self.length();
        let mask = (BigInt::new(Sign::Plus, vec![1]) << length) - 1;
        let value = (self.value() << shamt) & mask;

        Bit { value, length }
    }
}

impl Shr<usize> for Bit {
    type Output = Bit;

    fn shr(self, shamt: usize) -> Bit {
        let value = self.value() >> shamt;
        let length = self.length();

        Bit{ value, length }
    }
}

impl Eq for Bit {}
impl PartialEq for Bit {
    fn eq(&self, other: &Bit) -> bool {
        let same_value = || -> bool { self.value() == other.value() };
        let same_length = || -> bool { self.length() == self.length() };

        same_value() && same_length()
    }
}

fn binops(a: Bit, b: Bit, f: impl Fn(Bit, Bit) -> BigInt) -> Bit {
    let length = max(a.length(), b.length());
    let mask = (BigInt::new(Sign::Plus, vec![1]) << length) - 1;
    let value = f(a, b) & mask;

    return Bit{ value, length }
}

impl Ord for Bit {
    fn cmp(&self, other: &Bit) -> Ordering {
        self.value().cmp(other.value())
    }
}

impl PartialOrd for Bit {
    fn partial_cmp(&self, other: &Bit) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}