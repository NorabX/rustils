use parse::error::ParseError;
use parse::types::{
    ParseResultU8, ParseResultU16, ParseResultU32, ParseResultU64, ParseResultUsize
};

/// Converts `0_i8` to `false` and all other numbers to `true`.
/// # Example
///
/// ```
/// use rustils::parse::byte::to_bool;
///
/// assert_eq!(to_bool(1_i8), true);
/// assert_eq!(to_bool(42_i8), true);
/// assert_eq!(to_bool(0_i8), false);
/// ```
pub fn to_bool(b:i8) -> bool {
    if b == 0 { false }
    else { true }
}

/// Converts `i8` to `u8`.
/// Returns `Err` if `b` is negative,
/// otherwise retruns the `Ok` value of `b` as `u8`.
/// # Example
///
/// ```
/// use rustils::parse::byte::to_u8;
///
/// assert_eq!(to_u8(0_i8).unwrap(), 0_u8);
/// assert_eq!(to_u8(127_i8).unwrap(), 127_u8);
/// assert!(to_u8(-1_i8).is_err());
/// ```
pub fn to_u8(b: i8) -> ParseResultU8 {
    if b < 0 {
        Err(ParseError::InvalidNumber(b.to_string()))
    } else { Ok(b as u8) }
}

/// Converts `i8` to `u16`.
/// Returns `Err` if `b` is negative,
/// otherwise retruns the `Ok` value of `b` as `u16`.
/// # Example
///
/// ```
/// use rustils::parse::byte::to_u16;
///
/// assert_eq!(to_u16(0_i8).unwrap(), 0_u16);
/// assert_eq!(to_u16(127_i8).unwrap(), 127_u16);
/// assert!(to_u16(-1_i8).is_err());
/// ```
pub fn to_u16(b:i8) -> ParseResultU16 {
    if b < 0 {
        Err(ParseError::InvalidNumber(b.to_string()))
    } else { Ok(b as u16) }
}

/// Converts `i8` to `u32`.
/// Returns `Err` if `b` is negative,
/// otherwise retruns the `Ok` value of `b` as `u32`.
/// # Example
///
/// ```
/// use rustils::parse::byte::to_u32;
///
/// assert_eq!(to_u32(0_i8).unwrap(), 0_u32);
/// assert_eq!(to_u32(127_i8).unwrap(), 127_u32);
/// assert!(to_u32(-1_i8).is_err());
/// ```
pub fn to_u32(b:i8) -> ParseResultU32 {
    if b < 0 {
        Err(ParseError::InvalidNumber(b.to_string()))
    } else { Ok(b as u32) }
}

/// Converts `i8` to `u64`.
/// Returns `Err` if `b` is negative,
/// otherwise retruns the `Ok` value of `b` as `u64`.
/// # Example
///
/// ```
/// use rustils::parse::byte::to_u64;
///
/// assert_eq!(to_u64(0_i8).unwrap(), 0_u64);
/// assert_eq!(to_u64(127_i8).unwrap(), 127_u64);
/// assert!(to_u64(-1_i8).is_err());
/// ```
pub fn to_u64(b:i8) -> ParseResultU64 {
    if b < 0 {
        Err(ParseError::InvalidNumber(b.to_string()))
    } else { Ok(b as u64) }
}

/// Converts `i8` to `usize`.
/// Returns `Err` if `b` is negative,
/// otherwise retruns the `Ok` value of `b` as `usize`.
/// # Example
///
/// ```
/// use rustils::parse::byte::to_usize;
///
/// assert_eq!(to_usize(0_i8).unwrap(), 0_usize);
/// assert_eq!(to_usize(127_i8).unwrap(), 127_usize);
/// assert!(to_usize(-1_i8).is_err());
/// ```
pub fn to_usize(b:i8) -> ParseResultUsize {
    if b < 0 {
        Err(ParseError::InvalidNumber(b.to_string()))
    } else { Ok(b as usize) }
}

/// Converts `i8` to `u8`.
/// Returns `panic!` if `b` is negative,
/// otherwise retruns the value of `b` as `u8`.
/// # Example
///
/// ```
/// use rustils::parse::byte::to_u8p;
///
/// assert_eq!(to_u8p(0_i8), 0_u8);
/// assert_eq!(to_u8p(127_i8), 127_u8);
/// ```
/// # Panics
///
/// ```rust,should_panic
/// rustils::parse::byte::to_u8p(-1_i8);
/// ```
pub fn to_u8p(b:i8) -> u8 {
    match to_u8(b) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

/// Converts `i8` to `u16`.
/// Returns `panic!` if `b` is negative,
/// otherwise retruns the value of `b` as `u16`.
/// # Example
///
/// ```
/// use rustils::parse::byte::to_u16p;
///
/// assert_eq!(to_u16p(0_i8), 0_u16);
/// assert_eq!(to_u16p(127_i8), 127_u16);
/// ```
/// # Panics
///
/// ```rust,should_panic
/// rustils::parse::byte::to_u16p(-1_i8);
/// ```
pub fn to_u16p(b:i8) -> u16 {
    match to_u16(b) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

/// Converts `i8` to `u32`.
/// Returns `panic!` if `b` is negative,
/// otherwise retruns the value of `b` as `u32`.
/// # Example
///
/// ```
/// use rustils::parse::byte::to_u32p;
///
/// assert_eq!(to_u32p(0_i8), 0_u32);
/// assert_eq!(to_u32p(127_i8), 127_u32);
/// ```
/// # Panics
///
/// ```rust,should_panic
/// rustils::parse::byte::to_u32p(-1_i8);
/// ```
pub fn to_u32p(b:i8) -> u32 {
    match to_u32(b) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

/// Converts `i8` to `u64`.
/// Returns `panic!` if `b` is negative,
/// otherwise retruns the value of `b` as `u64`.
/// # Example
///
/// ```
/// use rustils::parse::byte::to_u64p;
///
/// assert_eq!(to_u64p(0_i8), 0_u64);
/// assert_eq!(to_u64p(127_i8), 127_u64);
/// ```
/// # Panics
///
/// ```rust,should_panic
/// rustils::parse::byte::to_u64p(-1_i8);
/// ```
pub fn to_u64p(b:i8) -> u64 {
    match to_u64(b) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

/// Converts `i8` to `usize`.
/// Returns `panic!` if `b` is negative,
/// otherwise retruns the value of `b` as `usize`.
/// # Example
///
/// ```
/// use rustils::parse::byte::to_usizep;
///
/// assert_eq!(to_usizep(0_i8), 0_usize);
/// assert_eq!(to_usizep(127_i8), 127_usize);
/// ```
/// # Panics
///
/// ```rust,should_panic
/// rustils::parse::byte::to_usizep(-1_i8);
/// ```
pub fn to_usizep(b:i8) -> usize {
    match to_usize(b) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}
