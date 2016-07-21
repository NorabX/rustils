use parse::error::ParseError;
use parse::types::{
    ParseResultI8,ParseResultI16,
    ParseResultU8,ParseResultU16,ParseResultU32,ParseResultU64,
};

pub fn tobool(i:i32) -> bool {
    if i == 0 { false }
    else { true }
}

pub fn toi8(i:i32) -> ParseResultI8 {
    let min = i8::min_value() as i32;
    let max = i8::max_value() as i32;

    if i < min || i > max {
        Err(ParseError::InvalidNumber(i.to_string()))
    } else { Ok(i as i8) }
}

pub fn toi16(i:i32) -> ParseResultI16 {
    let min = i16::min_value() as i32;
    let max = i16::max_value() as i32;

    if i < min || i > max {
        Err(ParseError::InvalidNumber(i.to_string()))
    } else { Ok(i as i16) }
}

pub fn tou8(i:i32) -> ParseResultU8 {
    let max = u8::max_value() as i32;

    if i < 0 || i > max {
        Err(ParseError::InvalidNumber(i.to_string()))
    } else { Ok(i as u8) }
}

pub fn tou16(i:i32) -> ParseResultU16 {
    let max = u16::max_value() as i32;

    if i < 0 || i > max {
        Err(ParseError::InvalidNumber(i.to_string()))
    } else { Ok(i as u16) }
}

pub fn tou32(i:i32) -> ParseResultU32 {
    if i < 0 {
        Err(ParseError::InvalidNumber(i.to_string()))
    } else { Ok(i as u32) }
}

pub fn tou64(i:i32) -> ParseResultU64 {
    if i < 0 {
        Err(ParseError::InvalidNumber(i.to_string()))
    } else { Ok(i as u64) }
}

pub fn toi8p(i:i32) -> i8 {
    match toi8(i) {
        Ok(n) => n,
        Err(err) => panic!("{}",err)
    }
}

pub fn toi16p(i:i32) -> i16 {
    match toi16(i) {
        Ok(n) => n,
        Err(err) => panic!("{}",err)
    }
}

pub fn tou8p(i:i32) -> u8 {
    match tou8(i) {
        Ok(n) => n,
        Err(err) => panic!("{}",err)
    }
}

pub fn tou16p(i:i32) -> u16 {
    match tou16(i) {
        Ok(n) => n,
        Err(err) => panic!("{}",err)
    }
}

pub fn tou32p(i:i32) -> u32 {
    match tou32(i) {
        Ok(n) => n,
        Err(err) => panic!("{}",err)
    }
}

pub fn tou64p(i:i32) -> u64 {
    match tou64(i) {
        Ok(n) => n,
        Err(err) => panic!("{}",err)
    }
}
