mod ops;

#[cfg(test)]
mod test;

extern crate num_bigint;

use num_bigint::BigUint;

pub struct Bit {
    value: BigUint,
    length: usize,
}

impl Bit {
    pub fn value(&self) -> &BigUint {
        &self.value
    }

    pub fn length(&self) -> usize {
        self.length
    }

    pub fn truncate(&self, upper: usize, lower: usize) -> Bit {
        assert!(upper >= lower);
        assert!(upper < self.length);

        let length = upper - lower + 1;
        let all_assert_length = length / 32;
        let remain_length = length % 32;
        let mask = if remain_length == 0 {
            BigUint::new(vec![std::u32::MAX; all_assert_length])
        } else {
            let head_mask = std::u32::MAX >> (31 - remain_length);
            let mut mask = vec![std::u32::MAX; all_assert_length];
            mask.insert(0, head_mask);

            BigUint::new(mask)
        };

        let value = (&self.value >> lower) & &mask;

        Bit { value, length: upper - lower + 1 }
    }

    pub fn get_bit(&self, index: usize) -> Bit {
        assert!(index < self.length);

        let value = (&self.value >> index) & &BigUint::new(vec![1]);

        Bit { value, length: 1 }
    }

    pub fn concat(&self, that: &Bit) -> Bit {
        let value = (&self.value << that.length) | &that.value;

        Bit { value, length: self.length + that.length }
    }
}

pub trait BitConstructor<T> {
    fn new(value: T) -> Bit;
}

impl BitConstructor<u32> for Bit {
    fn new(value: u32) -> Bit {
        Bit { value: BigUint::new(vec![value]), length: 32 }
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

        Bit { value: BigUint::new(vec![value]), length }
    }
}