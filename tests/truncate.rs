use bitwise::*;

#[test]
fn truncate_range() {
    let a = Bit::new((10, 4));
    assert_eq!(a.truncate((2, 0)), Bit::new(2));
}

#[test]
fn truncate_bit() {
    let a = Bit::new((10, 4));
    assert_eq!(a.truncate(1), Bit::new(1));
}