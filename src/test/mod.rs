use super::{Bit, BitConstructor, Add, Truncate};
use num_bigint::{Sign, BigInt};

#[test]
#[should_panic]
fn constructor_cause_panic() {
    Bit::new((15, 2));
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
    let a = Bit::new((3, 2));
    let length = a.length();

    assert_eq!(length, 2);
}

#[test]
fn concatenate0() {
    let a = Bit::new((3, 2));
    let b = Bit::new((3, 2));
    let bit = a.concat(&b);

    assert_eq!(bit.value().clone(), BigInt::new(Sign::Plus, vec![15]));
    assert_eq!(bit.length(), 4);
}

#[test]
fn concatenate1() {
    let a = Bit::new(3);
    let b = Bit::new(4);
    let bit = a.concat(&b);

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
    let a = Bit::new((3, 8));
    let b = Bit::new((4, 4));

    assert_eq!(a + b, Bit::new((7, 8)));
}

#[test]
fn add2() {
    let a = Bit::new((3, 8));
    let b = Bit::new(4);

    assert_eq!(a + b, Bit::new((7, 32)));
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
    let c = Bit::new(2);
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
    let a = Bit::new((15, 4));

    assert_eq!(a.clone() << 2, Bit::new((12, 4)));
    assert_eq!(a.clone() >> 2, Bit::new((3, 4)));

}

#[test]
fn truncate_bit() {
    let a = Bit::new(10);

    assert_eq!(a.truncate(0), Bit::new((0, 1)));
    assert_eq!(a.truncate(1), Bit::new((1, 1)));
    assert_eq!(a.truncate(2), Bit::new((0, 1)));
    assert_eq!(a.truncate(3), Bit::new((1, 1)));
}

#[test]
fn truncate_range() {
    let a = Bit::new(10);

    assert_eq!(a.truncate((1, 0)), Bit::new(2));
    assert_eq!(a.truncate((2, 1)), Bit::new(1));
    assert_eq!(a.truncate((2, 2)), Bit::new(0));
}

#[test]
#[should_panic]
fn truncate_invalid_range0() {
    let a = Bit::new((10, 4));
    a.truncate((4, 3));
}

#[test]
#[should_panic]
fn truncate_invalid_range1() {
    let a = Bit::new((10, 4));
    a.truncate((2, 3));
}

#[test]
#[should_panic]
fn truncate_invalid_range2() {
    let a = Bit::new((10, 4));
    a.truncate((5, 4));
}

#[test]
#[should_panic]
fn truncate_invalid_index() {
    let a = Bit::new((10, 4));
    a.truncate(4);
}