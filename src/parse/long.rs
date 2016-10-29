/*use parse::error::ParseError;
use parse::types::{
    ParseResultI8,ParseResultI16,ParseResultI32,
    ParseResultU8,ParseResultU16,ParseResultU32,ParseResultU64,
    ParseResultUsize
};

///Converts `0_i64` to `false` and all other numbers to `true`.
/// # Example
///
/// ```
/// use rustils::parse::long::to_bool;
///
/// assert_eq!(to_bool(1), true);
/// assert_eq!(to_bool(42), true);
/// assert_eq!(to_bool(0), false);
/// ```
pub fn to_bool(l:i64) -> bool {
    if l == 0 { false }
    else { true }
}

/// Converts `i64` to `i8`.
/// Returns `Err` if `l` is less then `i8` min value or
/// `l` is greater then `i8` max value.
/// Otherwise returns the `Ok` value of `l` as `i8`.
/// # Example
///
/// ```
/// use rustils::parse::long::to_i8;
///
/// assert_eq!(to_i8(-128_i64).unwrap(), -128_i8);
/// assert_eq!(to_i8(127_i64).unwrap(), 127_i8);
/// assert!(to_i8(-129_i64).is_err());
/// assert!(to_i8(128_i64).is_err());
/// ```
pub fn to_i8(l:i64) -> ParseResultI8 {
    let min = i8::min_value() as i64;
    let max = i8::max_value() as i64;

    if l < min || l > max {
        Err(ParseError::InvalidNumber(l.to_string()))
    } else { Ok(l as i8) }
}

/// Converts `i64` to `i16`.
/// Returns `Err` if `l` is less then `i16` min value or
/// `l` is greater then `i16` max value.
/// Otherwise returns the `Ok` value of `l` as `i16`.
/// # Example
///
/// ```
/// use rustils::parse::long::to_i16;
///
/// assert_eq!(to_i16(-32768_i64).unwrap(), -32768_i16);
/// assert_eq!(to_i16(32767_i64).unwrap(), 32767_i16);
/// assert!(to_i16(-32769_i64).is_err());
/// assert!(to_i16(32768_i64).is_err());
/// ```
pub fn to_i16(l:i64) -> ParseResultI16 {
    let min = i16::min_value() as i64;
    let max = i16::max_value() as i64;

    if l < min || l > max {
        Err(ParseError::InvalidNumber(l.to_string()))
    } else { Ok(l as i16) }
}

/// Converts `i64` to `i32`.
/// Returns `Err` if `l` is less then `i32` min value or
/// `l` is greater then `i32` max value.
/// Otherwise returns the `Ok` value of `l` as `i32`.
/// # Example
///
/// ```
/// use rustils::parse::long::to_i32;
///
/// assert_eq!(to_i32(-2147483648_i64).unwrap(), -2147483648_i32);
/// assert_eq!(to_i32(2147483647_i64).unwrap(), 2147483647_i32);
/// assert!(to_i32(-2147483649_i64).is_err());
/// assert!(to_i32(2147483648_i64).is_err());
/// ```
pub fn to_i32(l:i64) -> ParseResultI32 {
    let min = i32::min_value() as i64;
    let max = i32::max_value() as i64;

    if l < min || l > max {
        Err(ParseError::InvalidNumber(l.to_string()))
    } else { Ok(l as i32) }
}

/// Converts `i64` to `u8`.
/// Returns `Err` if `l` is less then 0 or
/// `l` is greater then `u8` max value.
/// Otherwise returns the `Ok` value of `l` as `u8`.
/// # Example
///
/// ```
/// use rustils::parse::long::to_u8;
///
/// assert_eq!(to_u8(0_i64).unwrap(), 0_u8);
/// assert_eq!(to_u8(255_i64).unwrap(), 255_u8);
/// assert!(to_u8(-1_i64).is_err());
/// assert!(to_u8(256_i64).is_err());
/// ```
pub fn to_u8(l:i64) -> ParseResultU8 {
    let max = u8::max_value() as i64;

    if l < 0 || l > max {
        Err(ParseError::InvalidNumber(l.to_string()))
    } else { Ok(l as u8) }
}

/// Converts `i64` to `u16`.
/// Returns `Err` if `l` is less then 0 or
/// `l` is greater then `u16` max value.
/// Otherwise returns the `Ok` value of `l` as `u16`.
/// # Example
///
/// ```
/// use rustils::parse::long::to_u16;
///
/// assert_eq!(to_u16(0_i64).unwrap(), 0_u16);
/// assert_eq!(to_u16(65535_i64).unwrap(), 65535_u16);
/// assert!(to_u16(-1_i64).is_err());
/// assert!(to_u16(65536_i64).is_err());
/// ```
pub fn to_u16(l:i64) -> ParseResultU16 {
    let max = u16::max_value() as i64;

    if l < 0 || l > max {
        Err(ParseError::InvalidNumber(l.to_string()))
    } else { Ok(l as u16) }
}

/// Converts `i64` to `u32`.
/// Returns `Err` if `l` is less then 0.
/// Otherwise returns the `Ok` value of `l` as `u32`.
/// # Example
///
/// ```
/// use rustils::parse::long::to_u32;
///
/// assert_eq!(to_u32(0_i64).unwrap(), 0_u32);
/// assert_eq!(to_u32(4294967295_i64).unwrap(), 4294967295_u32);
/// assert!(to_u32(-1_i64).is_err());
/// assert!(to_u32(4294967296_i64).is_err());
/// ```
pub fn to_u32(l:i64) -> ParseResultU32 {
    let max = u32::max_value() as i64;

    if l < 0 || l > max {
        Err(ParseError::InvalidNumber(l.to_string()))
    } else { Ok(l as u32) }
}

/// Converts `i64` to `u64`.
/// Returns `Err` if `l` is less then 0.
/// Otherwise returns the `Ok` value of `l` as `u64`.
/// # Example
///
/// ```
/// use rustils::parse::long::to_u64;
///
/// assert_eq!(to_u64(0_i64).unwrap(), 0_u64);
/// assert_eq!(to_u64(9223372036854775807_i64).unwrap(), 9223372036854775807_u64);
/// assert!(to_u64(-1_i64).is_err());
/// assert!(to_u64(9223372036854775808_i64).is_err());
/// ```
pub fn to_u64(l:i64) -> ParseResultU64 {
    if l < 0 {
        Err(ParseError::InvalidNumber(l.to_string()))
    } else { Ok(l as u64) }
}

/// Converts `i64` to `usize`.
/// Returns `Err` if `l` is less then 0.
/// Otherwise returns the `Ok` value of `l` as `usize`.
/// # Example
///
/// ```
/// use rustils::parse::long::to_usize;
///
/// assert_eq!(to_usize(0_i64).unwrap(), 0_usize);
/// assert_eq!(to_usize(9223372036854775807_i64).unwrap(), 9223372036854775807_usize);
/// assert!(to_usize(-1_i64).is_err());
/// assert!(to_usize(9223372036854775808_i64).is_err());
/// ```
pub fn to_usize(l:i64) -> ParseResultUsize {
    if l < 0 {
        Err(ParseError::InvalidNumber(l.to_string()))
    } else { Ok(l as usize) }
}

/// Converts `i64` to `i8`.
/// Returns `panic!` if `l` is less then `i8` min value or
/// `l` is greater then `i8` max value.
/// Otherwise returns the value of `l` as `i8`.
/// # Example
///
/// ```
/// use rustils::parse::long::to_i8p;
///
/// assert_eq!(to_i8p(-128_i64), -128_i8);
/// assert_eq!(to_i8p(127_i64), 127_i8);
/// ```
/// # Panics
///
/// ```rust,should_panic
/// rustils::parse::long::to_i8p(-129_i64);
/// ```
/// ```rust,should_panic
/// rustils::parse::long::to_i8p(128_i64);
/// ```
pub fn to_i8p(l:i64) -> i8 {
    match to_i8(l) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

/// Converts `i64` to `i16`.
/// Returns `panic!` if `l` is less then `i16` min value or
/// `l` is greater then `i16` max value.
/// Otherwise returns the value of `l` as `i16`.
/// # Example
///
/// ```
/// use rustils::parse::long::to_i16p;
///
/// assert_eq!(to_i16p(-32768_i64), -32768_i16);
/// assert_eq!(to_i16p(32767_i64), 32767_i16);
/// ```
/// # Panics
///
/// ```rust,should_panic
/// rustils::parse::long::to_i16p(-32769_i64);
/// ```
/// ```rust,should_panic
/// rustils::parse::long::to_i16p(32768_i64);
/// ```
pub fn to_i16p(l:i64) -> i16 {
    match to_i16(l) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

/// Converts `i64` to `i32`.
/// Returns `panic!` if `l` is less then `i16` min value or
/// `l` is greater then `i32` max value.
/// Otherwise returns the value of `l` as `i32`.
/// # Example
///
/// ```
/// use rustils::parse::long::to_i32p;
///
/// assert_eq!(to_i32p(-2147483648_i64), -2147483648_i32);
/// assert_eq!(to_i32p(2147483647_i64), 2147483647_i32);
/// ```
/// # Panics
///
/// ```rust,should_panic
/// rustils::parse::long::to_i32p(-2147483649_i64);
/// ```
/// ```rust,should_panic
/// rustils::parse::long::to_i32p(2147483648_i64);
/// ```
pub fn to_i32p(l:i64) -> i32 {
    match to_i32(l) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

/// Converts `i64` to `u8`.
/// Returns `panic!` if `l` is less then 0 or
/// `l` is greater then `u8` max value.
/// Otherwise returns the the value of `l` as `u8`.
/// # Example
///
/// ```
/// use rustils::parse::long::to_u8p;
///
/// assert_eq!(to_u8p(0_i64), 0_u8);
/// assert_eq!(to_u8p(255_i64), 255_u8);
/// ```
/// # Panics
///
/// ```rust,should_panic
/// rustils::parse::long::to_u8p(-1_i64);
/// ```
/// ```rust,should_panic
/// rustils::parse::long::to_u8p(256_i64);
/// ```
pub fn to_u8p(l:i64) -> u8 {
    match to_u8(l) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub fn to_u16p(l:i64) -> u16 {
    match to_u16(l) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub fn to_u32p(l:i64) -> u32 {
    match to_u32(l) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub fn to_u64p(l:i64) -> u64 {
    match to_u64(l) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}*/
