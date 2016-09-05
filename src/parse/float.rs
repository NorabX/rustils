use std::f32;
use std::cmp::Ordering;
use parse::string;
use parse::error::ParseError;
use parse::types::{
    ParseResultI8,ParseResultI16,ParseResultI32,ParseResultI64,
    ParseResultU8,ParseResultU16,ParseResultU32,ParseResultU64,
};

///Converts `0_f32` to `false` and all other numbers to `true`.
/// # Example
///
/// ```
/// use rustils::parse::float;
///
/// let x = 1_f32;
/// let y = 42_f32;
/// let z = 0_f32;
///
/// assert_eq!(float::to_bool(x), true);
/// assert_eq!(float::to_bool(y), true);
/// assert_eq!(float::to_bool(z), false);
/// ```
pub fn to_bool(f:f32) -> bool {
    if f.is_nan() || f == 0.0 { false }
    else { true }
}

pub fn toi8(f:f32) -> ParseResultI8 {
    let min = i8::min_value() as f32;
    let max = i8::max_value() as f32;

    if f.is_nan() || f < min || f > max {
        Err(ParseError::InvalidNumber(f.to_string()))
    } else { Ok(f as i8) }
}

pub fn toi16(f:f32) -> ParseResultI16 {
    let min = i16::min_value() as f32;
    let max = i16::max_value() as f32;

    if f.is_nan() || f < min || f > max {
        Err(ParseError::InvalidNumber(f.to_string()))
    } else { Ok(f as i16) }
}

//TODO!!!!!!!!!!!!!!!!!!!!!!!!!!!!
pub fn toi32(f:f32) -> ParseResultI32 {
    let min = i32::min_value() as f32;
    let max = i32::max_value() as f32;

    if f.is_nan() || f < min || f > max {
        Err(ParseError::InvalidNumber(f.to_string()))
    } else { Ok(f as i32) }
}

//TODO!!!!!!!!!!!!!!!!!!!!!!!!!!!!
pub fn toi64(f:f32) -> ParseResultI64 {
    let min = i64::min_value() as f32;
    let max = i64::max_value() as f32;

    if f.is_nan() || f < min || f > max {
        Err(ParseError::InvalidNumber(f.to_string()))
    } else { Ok(f as i64) }
}

pub fn tou8(f:f32) -> ParseResultU8 {
    let max = u8::max_value() as f32;

    if f.is_nan() || f < 0.0 || f > max {
        Err(ParseError::InvalidNumber(f.to_string()))
    } else { Ok(f as u8) }
}

pub fn tou16(f:f32) -> ParseResultU16 {
    let max = u16::max_value() as f32;

    if f.is_nan() || f < 0.0 || f > max {
        Err(ParseError::InvalidNumber(f.to_string()))
    } else { Ok(f as u16) }
}


pub fn tou32(f:f32) -> ParseResultU32{
    let max = u32::max_value().to_string();
    let fstr = f.to_string();

    if f.is_nan() || f < 0.0 {
        Err(ParseError::InvalidNumber(f.to_string()))
    } else {
        match fstr.cmp(&max){
            Ordering::Less | Ordering::Equal => Ok(string::to_u32p(&fstr)),
            Ordering::Greater => Err(ParseError::InvalidNumber(f.to_string()))
        }
    }
}

pub fn tou64(f:f32) -> ParseResultU64{
    let max = u64::max_value().to_string();
    let fstr = f.to_string();

    if f.is_nan() || f < 0.0 {
        Err(ParseError::InvalidNumber(f.to_string()))
    } else {
        match fstr.cmp(&max){
            Ordering::Less | Ordering::Equal => Ok(string::to_u64p(&fstr)),
            Ordering::Greater => Err(ParseError::InvalidNumber(f.to_string()))
        }
    }
}

pub fn toi8p(f:f32) -> i8 {
    match toi8(f) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub fn toi16p(f:f32) -> i16 {
    match toi16(f) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub fn toi32p(f:f32) -> i32 {
    match toi32(f) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub fn toi64p(f:f32) -> i64 {
    match toi64(f) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub fn tou8p(f:f32) -> u8 {
    match tou8(f) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub fn tou16p(f:f32) -> u16 {
    match tou16(f) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub fn tou32p(f:f32) -> u32 {
    match tou32(f) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub fn tou64p(f:f32) -> u64 {
    match tou64(f) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}
