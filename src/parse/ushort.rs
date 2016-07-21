use parse::error::ParseError;
use parse::types::{
    ParseResultI8,ParseResultI16,
    ParseResultU8
};

pub fn tobool(s:u16) -> bool {
    if s == 0 { false }
    else { true }
}

pub fn toi8(s:u16) -> ParseResultI8 {
    let max = i8::max_value() as u16;

    if s > max {
        Err(ParseError::InvalidNumber(s.to_string()))
    } else { Ok(s as i8) }
}

pub fn toi16(s:u16) -> ParseResultI16 {
    let max = i16::max_value() as u16;

    if s > max {
        Err(ParseError::InvalidNumber(s.to_string()))
    } else { Ok(s as i16) }
}

pub fn tou8(s:u16) -> ParseResultU8 {
    let max = u8::max_value() as u16;

    if s > max {
        Err(ParseError::InvalidNumber(s.to_string()))
    } else { Ok(s as u8) }
}

pub fn toi8p(s:u16) -> i8 {
    match toi8(s) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub fn toi16p(s:u16) -> i16 {
    match toi16(s) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub fn tou8p(s:u16) -> u8 {
    match tou8(s) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}
