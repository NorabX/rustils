use parse::error::ParseError;
use parse::types::{
    ParseResultI8,ParseResultI16,ParseResultI32,
    ParseResultU8,ParseResultU16
};

pub fn tobool(i:u32) -> bool {
    if i == 0 { false }
    else { true }
}

pub fn toi8(i:u32) -> ParseResultI8 {
    let max = i8::max_value() as u32;

    if i > max {
        Err(ParseError::InvalidNumber(i.to_string()))
    } else { Ok(i as i8) }
}

pub fn toi16(i:u32) -> ParseResultI16 {
    let max = i16::max_value() as u32;

    if i > max {
        Err(ParseError::InvalidNumber(i.to_string()))
    } else { Ok(i as i16) }
}

pub fn toi32(i:u32) -> ParseResultI32 {
    let max = i32::max_value() as u32;

    if i > max {
        Err(ParseError::InvalidNumber(i.to_string()))
    } else { Ok(i as i32) }
}

pub fn tou8(i:u32) -> ParseResultU8 {
    let max = u8::max_value() as u32;

    if i > max {
        Err(ParseError::InvalidNumber(i.to_string()))
    } else { Ok(i as u8) }
}

pub fn tou16(i:u32) -> ParseResultU16 {
    let max = u16::max_value() as u32;

    if i > max {
        Err(ParseError::InvalidNumber(i.to_string()))
    } else { Ok(i as u16) }
}

pub fn toi8p(i:u32) -> i8 {
    match toi8(i) {
        Ok(n) => n,
        Err(err) => panic!("{}",err)
    }
}

pub fn toi16p(i:u32) -> i16 {
    match toi16(i) {
        Ok(n) => n,
        Err(err) => panic!("{}",err)
    }
}

pub fn toi32p(i:u32) -> i32 {
    match toi32(i) {
        Ok(n) => n,
        Err(err) => panic!("{}",err)
    }
}

pub fn tou8p(i:u32) -> u8 {
    match tou8(i) {
        Ok(n) => n,
        Err(err) => panic!("{}",err)
    }
}

pub fn tou16p(i:u32) -> u16 {
    match tou16(i) {
        Ok(n) => n,
        Err(err) => panic!("{}",err)
    }
}
