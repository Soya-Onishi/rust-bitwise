use num_bigint::Sign;

pub enum Error {
    NotEnoughLengthToConcat(usize),
    NotEnoughLengthToExt(usize, usize),
    OverBitLength(usize, usize),
    UpperLowerThanLower(usize, usize),
    TooLongToCast(usize, usize),
    TooShortToConstruct(usize, usize),
    SignNotPlus(Sign),

}