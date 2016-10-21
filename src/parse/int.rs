use parse::error::ParseError;
use parse::types::{
    ParseResultI8,ParseResultI16,
    ParseResultU8,ParseResultU16,ParseResultU32,ParseResultU64,
    ParseResultUsize
};

///Converts `0_i32` to `false` and all other numbers to `true`.
/// # Example
///
/// ```
/// use rustils::parse::int::to_bool;
///
/// assert_eq!(to_bool(1), true);
/// assert_eq!(to_bool(42), true);
/// assert_eq!(to_bool(0), false);
/// ```
pub fn to_bool(i:i32) -> bool {
    if i == 0 { false }
    else { true }
}

/// Converts `i32` to `i8`.
/// Returns `Err` if `i` is less then `i8` min value or
/// `i` is greater then `i8` max value.
/// Otherwise returns the `Ok` value of `i` as `i8`.
/// # Example
///
/// ```
/// use rustils::parse::int::to_i8;
///
/// assert_eq!(to_i8(-128_i32).unwrap(), -128_i8);
/// assert_eq!(to_i8(127_i32).unwrap(), 127_i8);
/// assert!(to_i8(-129_i32).is_err());
/// assert!(to_i8(128_i32).is_err());
/// ```
pub fn to_i8(i:i32) -> ParseResultI8 {
    let min = i8::min_value() as i32;
    let max = i8::max_value() as i32;

    if i < min || i > max {
        Err(ParseError::InvalidNumber(i.to_string()))
    } else { Ok(i as i8) }
}

/// Converts `i32` to `i16`.
/// Returns `Err` if `i` is less then `i16` min value or
/// `i` is greater then `i16` max value.
/// Otherwise returns the `Ok` value of `i` as `i16`.
/// # Example
///
/// ```
/// use rustils::parse::int::to_i16;
///
/// assert_eq!(to_i16(-32768_i32).unwrap(), -32768_i16);
/// assert_eq!(to_i16(32767_i32).unwrap(), 32767_i16);
/// assert!(to_i16(-32769_i32).is_err());
/// assert!(to_i16(32768_i32).is_err());
/// ```
pub fn to_i16(i:i32) -> ParseResultI16 {
    let min = i16::min_value() as i32;
    let max = i16::max_value() as i32;

    if i < min || i > max {
        Err(ParseError::InvalidNumber(i.to_string()))
    } else { Ok(i as i16) }
}

/// Converts `i32` to `u8`.
/// Returns `Err` if `i` is less then 0 or
/// `i` is greater then `u8` max value.
/// Otherwise returns the `Ok` value of `i` as `u8`.
/// # Example
///
/// ```
/// use rustils::parse::int::to_u8;
///
/// assert_eq!(to_u8(0_i32).unwrap(), 0_u8);
/// assert_eq!(to_u8(255_i32).unwrap(), 255_u8);
/// assert!(to_u8(-1_i32).is_err());
/// assert!(to_u8(256_i32).is_err());
/// ```
pub fn to_u8(i:i32) -> ParseResultU8 {
    let max = u8::max_value() as i32;

    if i < 0 || i > max {
        Err(ParseError::InvalidNumber(i.to_string()))
    } else { Ok(i as u8) }
}

/// Converts `i32` to `u16`.
/// Returns `Err` if `i` is less then 0 or
/// `i` is greater then `u16` max value.
/// Otherwise returns the `Ok` value of `i` as `u16`.
/// # Example
///
/// ```
/// use rustils::parse::int::to_u16;
///
/// assert_eq!(to_u16(0_i32).unwrap(), 0_u16);
/// assert_eq!(to_u16(65535_i32).unwrap(), 65535_u16);
/// assert!(to_u16(-1_i32).is_err());
/// assert!(to_u16(65536_i32).is_err());
/// ```
pub fn to_u16(i:i32) -> ParseResultU16 {
    let max = u16::max_value() as i32;

    if i < 0 || i > max {
        Err(ParseError::InvalidNumber(i.to_string()))
    } else { Ok(i as u16) }
}

/// Converts `i32` to `u32`.
/// Returns `Err` if `i` is less then 0.
/// Otherwise returns the `Ok` value of `i` as `u32`.
/// # Example
///
/// ```
/// use rustils::parse::int::to_u32;
///
/// assert_eq!(to_u32(0_i32).unwrap(), 0_u32);
/// assert_eq!(to_u32(2147483647_i32).unwrap(), 2147483647_u32);
/// assert!(to_u32(-1_i32).is_err());
/// assert!(to_u32(2147483648_i32).is_err());
/// ```
pub fn to_u32(i:i32) -> ParseResultU32 {
    if i < 0 {
        Err(ParseError::InvalidNumber(i.to_string()))
    } else { Ok(i as u32) }
}

/// Converts `i32` to `u64`.
/// Returns `Err` if `i` is less then 0.
/// Otherwise returns the `Ok` value of `i` as `u64`.
/// # Example
///
/// ```
/// use rustils::parse::int::to_u64;
///
/// assert_eq!(to_u64(0_i32).unwrap(), 0_u64);
/// assert_eq!(to_u64(2147483647_i32).unwrap(), 2147483647_u64);
/// assert!(to_u64(-1_i32).is_err());
/// ```
pub fn to_u64(i:i32) -> ParseResultU64 {
    if i < 0 {
        Err(ParseError::InvalidNumber(i.to_string()))
    } else { Ok(i as u64) }
}

/// Converts `i32` to `usize`.
/// Returns `Err` if `i` is less then 0.
/// Otherwise returns the `Ok` value of `i` as `usize`.
/// # Example
///
/// ```
/// use rustils::parse::int::to_usize;
///
/// assert_eq!(to_usize(0_i32).unwrap(), 0_usize);
/// assert_eq!(to_usize(2147483647_i32).unwrap(), 2147483647_usize);
/// assert!(to_usize(-1_i32).is_err());
/// ```
pub fn to_usize(i:i32) -> ParseResultUsize {
    if i < 0 {
        Err(ParseError::InvalidNumber(i.to_string()))
    } else { Ok(i as usize) }
}


/// Converts `i32` to `i8`.
/// Returns `panic!` if `i` is less then `i8` min value or
/// `i` is greater then `i8` max value.
/// Otherwise returns the value of `i` as `i8`.
/// # Example
///
/// ```
/// use rustils::parse::int::to_i8p;
///
/// assert_eq!(to_i8p(-128_i32), -128_i8);
/// assert_eq!(to_i8p(127_i32), 127_i8);
/// ```
/// # Panics
///
/// ```rust,should_panic
/// rustils::parse::int::to_i8p(-129_i32);
/// ```
/// ```rust,should_panic
/// rustils::parse::int::to_i8p(128_i32);
/// ```
pub fn to_i8p(i:i32) -> i8 {
    match to_i8(i) {
        Ok(n) => n,
        Err(err) => panic!("{}",err)
    }
}

/// Converts `i32` to `i16`.
/// Returns `panic!` if `i` is less then `i16` min value or
/// `i` is greater then `i16` max value.
/// Otherwise returns the value of `i` as `i16`.
/// # Example
///
/// ```
/// use rustils::parse::int::to_i16p;
///
/// assert_eq!(to_i16p(-32768_i32), -32768_i16);
/// assert_eq!(to_i16p(32767_i32), 32767_i16);
/// ```
/// # Panics
///
/// ```rust,should_panic
/// rustils::parse::int::to_i16p(-32769_i32);
/// ```
/// ```rust,should_panic
/// rustils::parse::int::to_i16p(32768_i32);
/// ```
pub fn to_i16p(i:i32) -> i16 {
    match to_i16(i) {
        Ok(n) => n,
        Err(err) => panic!("{}",err)
    }
}

/// Converts `i32` to `u8`.
/// Returns `panic!` if `i` is less then 0 or
/// `i` is greater then `u8` max value.
/// Otherwise returns the the value of `i` as `u8`.
/// # Example
///
/// ```
/// use rustils::parse::int::to_u8p;
///
/// assert_eq!(to_u8p(0_i32), 0_u8);
/// assert_eq!(to_u8p(255_i32), 255_u8);
/// ```
/// # Panics
///
/// ```rust,should_panic
/// rustils::parse::int::to_u8p(-1_i32);
/// ```
/// ```rust,should_panic
/// rustils::parse::int::to_u8p(256_i32);
/// ```
pub fn to_u8p(i:i32) -> u8 {
    match to_u8(i) {
        Ok(n) => n,
        Err(err) => panic!("{}",err)
    }
}

/// Converts `i32` to `u16`.
/// Returns `panic!` if `i` is less then 0 or
/// `i` is greater then `u16` max value.
/// Otherwise returns the the value of `i` as `u16`.
/// # Example
///
/// ```
/// use rustils::parse::int::to_u16p;
///
/// assert_eq!(to_u16p(0_i32), 0_u16);
/// assert_eq!(to_u16p(65535_i32), 65535_u16);
/// ```
/// # Panics
///
/// ```rust,should_panic
/// rustils::parse::int::to_u16p(-1_i32);
/// ```
/// ```rust,should_panic
/// rustils::parse::int::to_u16p(65536_i32);
/// ```
pub fn to_u16p(i:i32) -> u16 {
    match to_u16(i) {
        Ok(n) => n,
        Err(err) => panic!("{}",err)
    }
}

/// Converts `i32` to `u32`.
/// Returns `panic!` if `i` is less then 0 or
/// `i` is greater then `u32` max value.
/// Otherwise returns the the value of `i` as `u32`.
/// # Example
///
/// ```
/// use rustils::parse::int::to_u32p;
///
/// assert_eq!(to_u32p(0_i32), 0_u32);
/// assert_eq!(to_u32p(2147483647_i32), 2147483647_u32);
/// ```
/// # Panics
///
/// ```rust,should_panic
/// rustils::parse::int::to_u32p(-1_i32);
/// ```
pub fn to_u32p(i:i32) -> u32 {
    match to_u32(i) {
        Ok(n) => n,
        Err(err) => panic!("{}",err)
    }
}

/// Converts `i32` to `u64`.
/// Returns `panic!` if `i` is less then 0 or
/// `i` is greater then `u64` max value.
/// Otherwise returns the the value of `i` as `u64`.
/// # Example
///
/// ```
/// use rustils::parse::int::to_u64p;
///
/// assert_eq!(to_u64p(0_i32), 0_u64);
/// assert_eq!(to_u64p(2147483647_i32), 2147483647_u64);
/// ```
/// # Panics
///
/// ```rust,should_panic
/// rustils::parse::int::to_u64p(-1_i32);
/// ```
pub fn to_u64p(i:i32) -> u64 {
    match to_u64(i) {
        Ok(n) => n,
        Err(err) => panic!("{}",err)
    }
}

/// Converts `i32` to `usize`.
/// Returns `panic!` if `i` is less then 0 or
/// `i` is greater then `usize` max value.
/// Otherwise returns the the value of `i` as `usize`.
/// # Example
///
/// ```
/// use rustils::parse::int::to_usizep;
///
/// assert_eq!(to_usizep(0_i32), 0_usize);
/// assert_eq!(to_usizep(2147483647_i32), 2147483647_usize);
/// ```
/// # Panics
///
/// ```rust,should_panic
/// rustils::parse::int::to_usizep(-1_i32);
/// ```
pub fn to_usizep(i:i32) -> usize {
    match to_usize(i) {
        Ok(n) => n,
        Err(err) => panic!("{}",err)
    }
}
