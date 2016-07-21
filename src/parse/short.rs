use parse::error::ParseError;
use parse::types::{
    ParseResultI8,
    ParseResultU8,ParseResultU16,ParseResultU32,ParseResultU64,
};

pub fn tobool(s:i16) -> bool {
    if s == 0 { false }
    else { true }
}

pub fn toi8(s:i16) -> ParseResultI8 {
    let min = i8::min_value() as i16;
    let max = i8::max_value() as i16;

    if s < min || s > max {
        Err(ParseError::InvalidNumber(s.to_string()))
    } else { Ok(s as i8) }
}

pub fn tou8(s:i16) -> ParseResultU8 {
    let max = u8::max_value() as i16;

    if s < 0 || s > max {
        Err(ParseError::InvalidNumber(s.to_string()))
    } else { Ok(s as u8) }
}

pub fn tou16(s:i16) -> ParseResultU16 {
    if s < 0 {
        Err(ParseError::InvalidNumber(s.to_string()))
    } else { Ok(s as u16) }
}

pub fn tou32(s:i16) -> ParseResultU32 {
    if s < 0 {
        Err(ParseError::InvalidNumber(s.to_string()))
    } else { Ok(s as u32) }
}

pub fn tou64(s:i16) -> ParseResultU64 {
    if s < 0 {
        Err(ParseError::InvalidNumber(s.to_string()))
    } else { Ok(s as u64) }
}

pub fn toi8p(s:i16) -> i8 {
    match toi8(s) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub fn tou8p(s:i16) -> u8 {
    match tou8(s) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub fn tou16p(s:i16) -> u16 {
    match tou16(s) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub fn tou32p(s:i16) -> u32 {
    match tou32(s) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub fn tou64p(s:i16) -> u64 {
    match tou64(s) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}
