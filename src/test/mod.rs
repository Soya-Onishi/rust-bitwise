extern crate rand;

use super::{Bit, BitConstructor, Truncate};
use num_bigint::{Sign, BigInt};

use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

#[test]
#[should_panic]
fn constructor_cause_panic() {
    Bit::new_with_length(15, 2).unwrap();
}

#[test]
fn create_instance_without_length() {
    let a = Bit::new(3);
    let value = a.value();
    let length = a.length();

    assert_eq!(value.clone(), BigInt::new(Sign::Plus, vec![3]));
    assert_eq!(length, 32)
}

#[test]
fn create_instance_with_length() {
    let a = Bit::new_with_length(3, 2).unwrap();
    let length = a.length();

    assert_eq!(length, 2);
}

#[test]
fn concatenate0() {
    let a = Bit::new_with_length(3, 2).unwrap();
    let b = Bit::new_with_length(3, 2).unwrap();
    let bit = Bit::concat(vec![&a, &b]).unwrap();

    assert_eq!(bit.value().clone(), BigInt::new(Sign::Plus, vec![15]));
    assert_eq!(bit.length(), 4);
}

#[test]
fn concatenate1() {
    let a = Bit::new(3);
    let b = Bit::new(4);
    let bit = Bit::concat(vec![&a, &b]).unwrap();

    assert_eq!(bit.value().clone(), BigInt::new(Sign::Plus, vec![4, 3]));
    assert_eq!(bit.length(), 64);
}

#[test]
fn add0() {
    let a = Bit::new(3);
    let b = Bit::new(4);

    assert_eq!(a + b, Bit::new(7));
}

#[test]
fn add1() {
    let a = Bit::new_with_length(3, 8).unwrap();
    let b = Bit::new_with_length(4, 4).unwrap();

    assert_eq!(a + b, Bit::new_with_length(7, 8).unwrap());
}

#[test]
fn add2() {
    let a = Bit::new_with_length(3, 8).unwrap();
    let b = Bit::new(4);

    assert_eq!(a + b, Bit::new_with_length(7, 32).unwrap());
}

#[test]
fn add_overflow() {
    let a = Bit::new(std::u32::MAX);
    let b = Bit::new(1);

    assert_eq!(a + b, Bit::new(0));
}

#[test]
fn add_with_zero() {
    let a = Bit::new(0);
    let b = Bit::new(1);

    assert_eq!(a + b, Bit::new(1));
}

#[test]
fn sub0() {
    let a = Bit::new(15);
    let b = Bit::new(4);

    assert_eq!(a - b, Bit::new(11));
}

#[test]
fn sub1() {
    let a = Bit::new(1);
    let b = Bit::new(2);

    assert_eq!(a - b, Bit::new(std::u32::MAX))
}

#[test]
fn sub_from_zero() {
    let a = Bit::new(0);
    let b = Bit::new(1);

    assert_eq!(a - b, Bit::new(std::u32::MAX))
}

#[test]
fn compare() {
    let a = Bit::new(0);
    let b = Bit::new(1);
    let d = Bit::new(1);

    assert!(a < b);
    assert_eq!(b, d);
    assert_ne!(a, b);
    assert!(a <= b);
    assert!(b > a);
    assert!(b >= a);
}

#[test]
fn shift() {
    let a = Bit::new_with_length(15, 4).unwrap();

    assert_eq!(a.clone() << 2, Bit::new_with_length(12, 4).unwrap());
    assert_eq!(a.clone() >> 2, Bit::new_with_length(3, 4).unwrap());

}

#[test]
fn truncate_bit() {
    let a = Bit::new(10);

    assert_eq!(a.truncate(0).unwrap(), Bit::new_with_length(0, 1).unwrap());
    assert_eq!(a.truncate(1).unwrap(), Bit::new_with_length(1, 1).unwrap());
    assert_eq!(a.truncate(2).unwrap(), Bit::new_with_length(0, 1).unwrap());
    assert_eq!(a.truncate(3).unwrap(), Bit::new_with_length(1, 1).unwrap());
}

#[test]
fn truncate_range() {
    let a = Bit::new(10);

    assert_eq!(a.truncate((1, 0)).unwrap(), Bit::new(2));
    assert_eq!(a.truncate((2, 1)).unwrap(), Bit::new(1));
    assert_eq!(a.truncate((2, 2)).unwrap(), Bit::new(0));
}

#[test]
#[should_panic]
fn truncate_invalid_range0() {
    let a = Bit::new_with_length(10, 4).unwrap();
    a.truncate((4, 3)).unwrap();
}

#[test]
#[should_panic]
fn truncate_invalid_range1() {
    let a = Bit::new_with_length(10, 4).unwrap();
    a.truncate((2, 3)).unwrap();
}

#[test]
#[should_panic]
fn truncate_invalid_range2() {
    let a = Bit::new_with_length(10, 4).unwrap();
    a.truncate((5, 4)).unwrap();
}

#[test]
#[should_panic]
fn truncate_invalid_index() {
    let a = Bit::new_with_length(10, 4).unwrap();
    a.truncate(4).unwrap();
}

#[test]
fn zero_extension() {
    let a = Bit::new_with_length(10, 4).unwrap().zero_ext(8).unwrap();
    let a_high = a.truncate((7, 4)).unwrap();
    let a_low = a.truncate((3, 0)).unwrap();

    assert_eq!(a_high, Bit::new(0));
    assert_eq!(a_low, Bit::new(10));
}

#[test]
fn sign_extension() {
    let a = Bit::new_with_length(10, 4).unwrap().sign_ext(8).unwrap();
    let a_high = a.truncate((7, 4)).unwrap();
    let a_low = a.truncate((3, 0)).unwrap();

    assert_eq!(a_high, Bit::new(15));
    assert_eq!(a_low, Bit::new(10));
}

#[test]
fn extension_same_length() {
    Bit::new_with_length(10, 4).unwrap().zero_ext(4).unwrap();
    Bit::new_with_length(10, 4).unwrap().sign_ext(4).unwrap();
}

#[test]
#[should_panic]
fn zero_extension_causes_panic() {
    Bit::new_with_length(10, 4).unwrap().zero_ext(2).unwrap();
}

#[test]
#[should_panic]
fn sign_extension_causes_panic() {
    Bit::new_with_length(10, 4).unwrap().sign_ext(2).unwrap();
}

#[test]
fn cast_into_u32() {
    let mut rng: StdRng = SeedableRng::seed_from_u64(0);

    let byte_test = |rng: &mut StdRng| {
        let value: u8 = rng.gen_range(0, std::u8::MAX);
        let byte = Bit::new_with_length(value as u32, 8).unwrap();
        assert_eq!(byte.as_u32().unwrap(), value as u32);
    };

    let short_test = |rng: &mut StdRng| {
        let value: u16 = rng.gen_range(0, std::u16::MAX);
        let short = Bit::new_with_length(value as u32, 16).unwrap();
        assert_eq!(short.as_u32().unwrap(), value as u32);
    };

    let word_test = |rng: &mut StdRng| {
        let value: u32 = rng.gen_range(0, std::u32::MAX);
        let word = Bit::new_with_length(value as u32, 32).unwrap();
        assert_eq!(word.as_u32().unwrap(), value as u32);
    };

    for _ in 0..1000 {
        byte_test(&mut rng);
        short_test(&mut rng);
        word_test(&mut rng);
    }
}

#[test]
fn cast_into_u8() {
    let byte = Bit::new_with_length(15, 8).unwrap();

    assert_eq!(byte.as_u8().unwrap(), 15);
}