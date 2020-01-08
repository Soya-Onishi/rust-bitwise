use bitwise::*;

#[test]
fn truncate_range() {
    let a = Bit::new((10, 4)).unwrap();
    assert_eq!(a.truncate((2, 0)).unwrap(), Bit::new(2).unwrap());
}

#[test]
fn truncate_bit() {
    let a = Bit::new((10, 4)).unwrap();
    assert_eq!(a.truncate(1).unwrap(), Bit::new(1).unwrap());
}