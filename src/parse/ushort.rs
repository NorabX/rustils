/*use parse::error::ParseError;
use parse::types::{
    ParseResultI8,ParseResultI16,
    ParseResultU8
};

///Converts `0_u16` to `false` and all other numbers to `true`.
/// # Example
///
/// ```
/// use rustils::parse::ushort;
///
/// let x = 1_u16;
/// let y = 42_u16;
/// let z = 0_u16;
///
/// assert_eq!(ushort::to_bool(x), true);
/// assert_eq!(ushort::to_bool(y), true);
/// assert_eq!(ushort::to_bool(z), false);
/// ```
pub fn to_bool(s:u16) -> bool {
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
*/
