use super::Bit;
use super::BitConstructor;
use num_bigint::BigUint;

#[test]
fn create_instance_without_length() {
    let a = Bit::new(3);
    let value = a.value();
    let length = a.length();

    assert_eq!(value.clone(), BigUint::new(vec![3]));
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

    assert_eq!(bit.value().clone(), BigUint::new(vec![15]));
    assert_eq!(bit.length(), 4);
}

#[test]
fn concatenate1() {
    let a = Bit::new(3);
    let b = Bit::new(4);
    let bit = a.concat(&b);

    assert_eq!(bit.value().clone(), BigUint::new(vec![4, 3]));
    assert_eq!(bit.length(), 64);
}