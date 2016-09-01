use parse::error::ParseError;
use parse::types::ParseResultI8;

///Converts `0_u8` to `false` and all other numbers to `true`.
/// # Example
///
/// ```
/// use rustils::parse::ubyte;
///
/// let x = 1_u8;
/// let y = 42_u8;
/// let z = 0_u8;
///
/// assert_eq!(ubyte::to_bool(x), true);
/// assert_eq!(ubyte::to_bool(y), true);
/// assert_eq!(ubyte::to_bool(z), false);
/// ```
pub fn to_bool(b:u8) -> bool {
    if b == 0 { false }
    else { true }
}

pub fn toi8(b:u8) -> ParseResultI8 {
    let max = i8::max_value() as u8;

    if b > max {
        Err(ParseError::InvalidNumber(b.to_string()))
    } else { Ok(b as i8) }
}

pub fn toi8p(b:u8) -> i8 {
    match toi8(b) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}
