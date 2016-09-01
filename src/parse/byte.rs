use parse::error::ParseError;
use parse::types::{
    ParseResultU8, ParseResultU16, ParseResultU32, ParseResultU64
};

///Converts `0_i8` to `false` and all other numbers to `true`.
/// # Example
///
/// ```
/// use rustils::parse::byte;
///
/// let x = 1_i8;
/// let y = 42_i8;
/// let z = 0_i8;
///
/// assert_eq!(byte::to_bool(x), true);
/// assert_eq!(byte::to_bool(y), true);
/// assert_eq!(byte::to_bool(z), false);
/// ```
pub fn to_bool(b:i8) -> bool {
    if b == 0 { false }
    else { true }
}

pub fn tou8(b:i8) -> ParseResultU8 {
    if b < 0 {
        Err(ParseError::InvalidNumber(b.to_string()))
    } else { Ok(b as u8) }
}

pub fn tou16(b:i8) -> ParseResultU16 {
    if b < 0 {
        Err(ParseError::InvalidNumber(b.to_string()))
    } else { Ok(b as u16) }
}

pub fn tou32(b:i8) -> ParseResultU32 {
    if b < 0 {
        Err(ParseError::InvalidNumber(b.to_string()))
    } else { Ok(b as u32) }
}

pub fn tou64(b:i8) -> ParseResultU64 {
    if b < 0 {
        Err(ParseError::InvalidNumber(b.to_string()))
    } else { Ok(b as u64) }
}

pub fn tou8p(b:i8) -> u8 {
    match tou8(b) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub fn tou16p(b:i8) -> u16 {
    match tou16(b) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub fn tou32p(b:i8) -> u32 {
    match tou32(b) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub fn tou64p(b:i8) -> u64 {
    match tou64(b) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}
