use bitwise::*;

#[test]
fn add() {
    let a = Bit::new(2).unwrap();
    let b = Bit::new(3).unwrap();

    assert_eq!(a + b, Bit::new(5).unwrap());
}

#[test]
fn sub() {
    let a = Bit::new(2).unwrap();
    let b = Bit::new(3).unwrap();

    assert_eq!(a - b, Bit::new(std::u32::MAX).unwrap());
}