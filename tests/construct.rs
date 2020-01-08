use bitwise::*;

#[test]
fn construct_bit() {
    let _a = Bit::new(1);
    let _b = Bit::new_with_length(2, 3).unwrap();
}

