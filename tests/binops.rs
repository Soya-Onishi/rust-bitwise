use bitwise::*;

#[test]
fn add() {
    let a = Bit::new(2);
    let b = Bit::new(3);

    assert_eq!(a + b, Bit::new(5));
}

#[test]
fn sub() {
    let a = Bit::new(2);
    let b = Bit::new(3);

    assert_eq!(a - b, Bit::new(std::u32::MAX));
}