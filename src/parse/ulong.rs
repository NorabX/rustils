use parse::error::ParseError;
use parse::types::{
    ParseResultI8,ParseResultI16,ParseResultI32,ParseResultI64,
    ParseResultU8,ParseResultU16,ParseResultU32
};

///Converts `0_u64` to `false` and all other numbers to `true`.
/// # Example
///
/// ```
/// use rustils::parse::ulong;
///
/// let x = 1_u64;
/// let y = 42_u64;
/// let z = 0_u64;
///
/// assert_eq!(ulong::to_bool(x), true);
/// assert_eq!(ulong::to_bool(y), true);
/// assert_eq!(ulong::to_bool(z), false);
/// ```
pub fn to_bool(l:u64) -> bool {
    if l == 0 { false }
    else { true }
}

pub fn toi8(l:u64) -> ParseResultI8 {
    let max = i8::max_value() as u64;

    if l > max {
        Err(ParseError::InvalidNumber(l.to_string()))
    } else { Ok(l as i8) }
}

pub fn toi16(l:u64) -> ParseResultI16 {
    let max = i16::max_value() as u64;

    if l > max {
        Err(ParseError::InvalidNumber(l.to_string()))
    } else { Ok(l as i16) }
}

pub fn toi32(l:u64) -> ParseResultI32 {
    let max = i32::max_value() as u64;

    if l > max {
        Err(ParseError::InvalidNumber(l.to_string()))
    } else { Ok(l as i32) }
}

pub fn toi64(l:u64) -> ParseResultI64 {
    let max = i64::max_value() as u64;

    if l > max {
        Err(ParseError::InvalidNumber(l.to_string()))
    } else { Ok(l as i64) }
}

pub fn tou8(l:u64) -> ParseResultU8 {
    let max = u8::max_value() as u64;

    if l > max {
        Err(ParseError::InvalidNumber(l.to_string()))
    } else { Ok(l as u8) }
}

pub fn tou16(l:u64) -> ParseResultU16 {
    let max = u16::max_value() as u64;

    if l > max {
        Err(ParseError::InvalidNumber(l.to_string()))
    } else { Ok(l as u16) }
}

pub fn tou32(l:u64) -> ParseResultU32 {
    let max = u32::max_value() as u64;

    if l > max {
        Err(ParseError::InvalidNumber(l.to_string()))
    } else { Ok(l as u32) }
}

pub fn toi8p(l:u64) -> i8 {
    match toi8(l) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub fn toi16p(l:u64) -> i16 {
    match toi16(l) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub fn toi32p(l:u64) -> i32 {
    match toi32(l) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub fn toi64p(l:u64) -> i64 {
    match toi64(l) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub fn tou8p(l:u64) -> u8 {
    match tou8(l) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub fn tou16p(l:u64) -> u16 {
    match tou16(l) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub fn tou32p(l:u64) -> u32 {
    match tou32(l) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}
