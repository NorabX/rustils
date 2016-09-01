use std::{ f32, f64 };
use std::cmp::Ordering;
use parse::string;
use parse::error::ParseError;
use parse::types::{
    ParseResultI8,ParseResultI16,ParseResultI32,ParseResultI64,
    ParseResultU8,ParseResultU16,ParseResultU32,ParseResultU64,
    ParseResultF32
};

///Converts `0_f64` to `false` and all other numbers to `true`.
/// # Example
///
/// ```
/// use rustils::parse::double;
///
/// let x = 1_f64;
/// let y = 42_f64;
/// let z = 0_f64;
///
/// assert_eq!(double::to_bool(x), true);
/// assert_eq!(double::to_bool(y), true);
/// assert_eq!(double::to_bool(z), false);
/// ```
pub fn to_bool(d:f64) -> bool {
    if d.is_nan() || d == 0.0 { false }
    else { true }
}

pub fn toi8(d:f64) -> ParseResultI8 {
    let min = i8::min_value() as f64;
    let max = i8::max_value() as f64;

    if d.is_nan() || d < min || d > max {
        Err(ParseError::InvalidNumber(d.to_string()))
    } else { Ok(d as i8) }
}

pub fn toi16(d:f64) -> ParseResultI16 {
    let min = i16::min_value() as f64;
    let max = i16::max_value() as f64;

    if d.is_nan() || d < min || d > max {
        Err(ParseError::InvalidNumber(d.to_string()))
    } else { Ok(d as i16) }
}

pub fn toi32(d:f64) -> ParseResultI32 {
    let min = i32::min_value() as f64;
    let max = i32::max_value() as f64;

    if d.is_nan() || d < min || d > max {
        Err(ParseError::InvalidNumber(d.to_string()))
    } else { Ok(d as i32) }
}

//TODO!!!!!!!!!!!!!!!!!!!!!!!!!!!!
pub fn toi64(d:f64) -> ParseResultI64 {
    let min = i64::min_value() as f64;
    let max = i64::max_value() as f64;

    if d.is_nan() || d < min || d > max {
        Err(ParseError::InvalidNumber(d.to_string()))
    } else { Ok(d as i64) }
}

pub fn tou8(d:f64) -> ParseResultU8 {
    let max = u8::max_value() as f64;

    if d.is_nan() || d < 0.0 || d > max {
        Err(ParseError::InvalidNumber(d.to_string()))
    } else { Ok(d as u8) }
}

pub fn tou16(d:f64) -> ParseResultU16 {
    let max = u16::max_value() as f64;

    if d.is_nan() || d < 0.0 || d > max {
        Err(ParseError::InvalidNumber(d.to_string()))
    } else { Ok(d as u16) }
}

pub fn tou32(d:f64) -> ParseResultU32 {
    let max = u32::max_value() as f64;

    if d.is_nan() || d < 0.0 || d > max {
        Err(ParseError::InvalidNumber(d.to_string()))
    } else { Ok(d as u32) }
}

pub fn tou64(d:f64) -> ParseResultU64{
    let max = u64::max_value().to_string();
    let dstr = d.to_string();

    if d.is_nan() || d < 0.0 {
        Err(ParseError::InvalidNumber(d.to_string()))
    } else {
        match dstr.cmp(&max){
            Ordering::Less | Ordering::Equal => Ok(string::tou64p(&dstr)),
            Ordering::Greater => Err(ParseError::InvalidNumber(d.to_string()))
        }
    }
}

pub fn tof32(d:f64) -> ParseResultF32{
    let min = f32::MIN as f64;
    let max = f32::MAX as f64;

    if d.is_nan() || d < min || d > max {
        Err(ParseError::InvalidNumber(d.to_string()))
    } else { Ok(d as f32) }
}

pub fn toi8p(d:f64) -> i8 {
    match toi8(d) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub fn toi16p(d:f64) -> i16 {
    match toi16(d) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub fn toi32p(d:f64) -> i32 {
    match toi32(d) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub fn toi64p(d:f64) -> i64 {
    match toi64(d) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub fn tou8p(d:f64) -> u8 {
    match tou8(d) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub fn tou16p(d:f64) -> u16 {
    match tou16(d) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub fn tou32p(d:f64) -> u32 {
    match tou32(d) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub fn tou64p(d:f64) -> u64 {
    match tou64(d) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub fn tof32p(d:f64) -> f32 {
    match tof32(d) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}
