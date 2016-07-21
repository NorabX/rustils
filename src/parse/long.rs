use parse::error::ParseError;
use parse::types::{
    ParseResultI8,ParseResultI16,ParseResultI32,
    ParseResultU8,ParseResultU16,ParseResultU32,ParseResultU64,
};

pub fn tobool(l:i64) -> bool {
    if l == 0 { false }
    else { true }
}

pub fn toi8(l:i64) -> ParseResultI8 {
    let min = i8::min_value() as i64;
    let max = i8::max_value() as i64;

    if l < min || l > max {
        Err(ParseError::InvalidNumber(l.to_string()))
    } else { Ok(l as i8) }
}

pub fn toi16(l:i64) -> ParseResultI16 {
    let min = i16::min_value() as i64;
    let max = i16::max_value() as i64;

    if l < min || l > max {
        Err(ParseError::InvalidNumber(l.to_string()))
    } else { Ok(l as i16) }
}

pub fn toi32(l:i64) -> ParseResultI32 {
    let min = i32::min_value() as i64;
    let max = i32::max_value() as i64;

    if l < min || l > max {
        Err(ParseError::InvalidNumber(l.to_string()))
    } else { Ok(l as i32) }
}

pub fn tou8(l:i64) -> ParseResultU8 {
    let max = u8::max_value() as i64;

    if l < 0 || l > max {
        Err(ParseError::InvalidNumber(l.to_string()))
    } else { Ok(l as u8) }
}

pub fn tou16(l:i64) -> ParseResultU16 {
    let max = u16::max_value() as i64;

    if l < 0 || l > max {
        Err(ParseError::InvalidNumber(l.to_string()))
    } else { Ok(l as u16) }
}

pub fn tou32(l:i64) -> ParseResultU32 {
    let max = u32::max_value() as i64;

    if l < 0 || l > max {
        Err(ParseError::InvalidNumber(l.to_string()))
    } else { Ok(l as u32) }
}

pub fn tou64(l:i64) -> ParseResultU64 {
    if l < 0 {
        Err(ParseError::InvalidNumber(l.to_string()))
    } else { Ok(l as u64) }
}

pub fn toi8p(l:i64) -> i8 {
    match toi8(l) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub fn toi16p(l:i64) -> i16 {
    match toi16(l) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub fn toi32p(l:i64) -> i32 {
    match toi32(l) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub fn tou8p(l:i64) -> u8 {
    match tou8(l) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub fn tou16p(l:i64) -> u16 {
    match tou16(l) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub fn tou32p(l:i64) -> u32 {
    match tou32(l) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub fn tou64p(l:i64) -> u64 {
    match tou64(l) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}
