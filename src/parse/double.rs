use parse::error::ParseError;
use parse::types::{
    ParseResultI8,ParseResultI16,ParseResultI32,ParseResultI64,
    ParseResultU8,ParseResultU16,ParseResultU32,ParseResultU64,
    ParseResultIsize, ParseResultUsize
};
use RoundingMode;
use RoundingMode::{Trunc,Round,Ceil,Floor};

///Converts `0_f64` or `nan` to `false` and all other numbers to `true`.
/// # Example
///
/// ```
/// use rustils::parse::double::to_bool;
///
/// assert_eq!(to_bool(1_f64), true);
/// assert_eq!(to_bool(4.2_f64), true);
/// assert_eq!(to_bool(0_f64), false);
/// ```
pub fn to_bool(d:f64) -> bool {
    if d.is_nan() || d == 0.0 { false }
    else { true }
}

/// Converts `f64` to `i8`.
/// Returns `Err` if `d` is less then `i8` min value,
/// `d` is greater then `i8` max value or `d` is nan.
/// Otherwise returns the `Ok` value of `d` as `i8`.
/// The `RoundingMode` is `Trunc`.
/// # Example
///
/// ```
/// use rustils::parse::double::to_i8;
///
/// assert_eq!(to_i8(-128_f64).unwrap(), -128_i8);
/// assert_eq!(to_i8(127_f64).unwrap(), 127_i8);
/// assert_eq!(to_i8(1.321_f64).unwrap(), 1_i8);
/// assert_eq!(to_i8(1.567_f64).unwrap(), 1_i8);
/// assert!(to_i8(-129_f64).is_err());
/// assert!(to_i8(128_f64).is_err());
/// ```
pub fn to_i8(d:f64) -> ParseResultI8 {
    to_i8r(d,Trunc)
}

/// Converts `f64` to `i8`.
/// Returns `Err` if `d` is less then `i8` min value,
/// `d` is greater then `i8` max value or `d` is nan.
/// Otherwise returns the `Ok` value of `d` as `i8`.
/// # Example
///
/// ```
/// use rustils::utils::RoundingMode::{Trunc,Round,Ceil,Floor};
/// use rustils::parse::double::to_i8r;
///
/// assert_eq!(to_i8r(-128_f64,Trunc).unwrap(), -128_i8);
/// assert_eq!(to_i8r(127_f64,Trunc).unwrap(), 127_i8);
/// assert_eq!(to_i8r(1.321_f64,Ceil).unwrap(), 2_i8);
/// assert_eq!(to_i8r(1.567_f64,Floor).unwrap(), 1_i8);
/// assert!(to_i8r(-128.5_f64,Round).is_err());
/// assert!(to_i8r(128_f64,Trunc).is_err());
/// ```
pub fn to_i8r(d:f64, rmode:RoundingMode) -> ParseResultI8 {
    let min = i8::min_value() as f64;
    let max = i8::max_value() as f64;

    let x = match rmode{
        Round => d.round(),
        Ceil => d.ceil(),
        Floor => d.floor(),
        Trunc => d.trunc()
    };

    if x.is_nan() || x < min || x > max {
        Err(ParseError::InvalidNumber(d.to_string()))
    } else { Ok(x as i8) }
}

/// Converts `f64` to `i8`.
/// Returns `panic!` if `d` is less then `i8` min value,
/// `d` is greater then `i8` max value or `d` is nan.
/// otherwise returns the value of `d` as `i8`.
/// The `RoundingMode` is `Trunc`.
/// # Example
///
/// ```
/// use rustils::parse::double::to_i8p;
///
/// assert_eq!(to_i8p(-128_f64), -128_i8);
/// assert_eq!(to_i8p(127_f64), 127_i8);
/// assert_eq!(to_i8p(1.321_f64), 1_i8);
/// assert_eq!(to_i8p(1.567_f64), 1_i8);
/// ```
/// # Panics
///
/// ```rust,should_panic
/// rustils::parse::double::to_i8p(-129_f64);
/// ```
/// ```rust,should_panic
/// rustils::parse::double::to_i8p(128_f64);
/// ```
pub fn to_i8p(d:f64) -> i8 {
    match to_i8(d) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

/// Converts `f64` to `i8`.
/// Returns `panic!` if `d` is less then `i8` min value,
/// `d` is greater then `i8` max value or `d` is nan.
/// otherwise returns the value of `d` as `i8`.
/// # Example
///
/// ```
/// use rustils::utils::RoundingMode::{Trunc,Ceil,Floor};
/// use rustils::parse::double::to_i8pr;
///
/// assert_eq!(to_i8pr(-128_f64,Trunc), -128_i8);
/// assert_eq!(to_i8pr(127_f64,Trunc), 127_i8);
/// assert_eq!(to_i8pr(1.321_f64,Ceil), 2_i8);
/// assert_eq!(to_i8pr(1.567_f64,Floor), 1_i8);
/// ```
/// # Panics
///
/// ```rust,should_panic
/// use rustils::utils::RoundingMode::{Round};
/// rustils::parse::double::to_i8pr(-128.5_f64,Round);
/// ```
/// ```rust,should_panic
/// use rustils::utils::RoundingMode::{Trunc};
/// rustils::parse::double::to_i8pr(128_f64,Trunc);
/// ```
pub fn to_i8pr(d:f64, rmode:RoundingMode) -> i8 {
    match to_i8r(d,rmode) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

/// Converts `f64` to `i16`.
/// Returns `Err` if `d` is less then `i16` min value,
/// `d` is greater then `i16` max value or `d` is nan.
/// Otherwise returns the `Ok` value of `d` as `i16`.
/// The `RoundingMode` is `Trunc`.
/// # Example
///
/// ```
/// use rustils::parse::double;
///
/// assert_eq!(double::to_i16(-32768_f64).unwrap(), -32768_i16);
/// assert_eq!(double::to_i16(32767_f64).unwrap(), 32767_i16);
/// assert_eq!(double::to_i16(1.321_f64).unwrap(), 1_i16);
/// assert_eq!(double::to_i16(1.567_f64).unwrap(), 1_i16);
/// assert!(double::to_i16(-32769_f64).is_err());
/// assert!(double::to_i16(32768_f64).is_err());
/// ```
pub fn to_i16(d:f64) -> ParseResultI16 {
    to_i16r(d,Trunc)
}

/// Converts `f64` to `i16`.
/// Returns `Err` if `d` is less then `i16` min value,
/// `d` is greater then `i16` max value or `d` is nan.
/// Otherwise returns the `Ok` value of `d` as `i16`.
/// # Example
///
/// ```
/// use rustils::utils::RoundingMode::{Trunc,Round,Ceil,Floor};
/// use rustils::parse::double::to_i16r;
///
/// assert_eq!(to_i16r(-32768_f64,Trunc).unwrap(), -32768_i16);
/// assert_eq!(to_i16r(32767_f64,Trunc).unwrap(), 32767_i16);
/// assert_eq!(to_i16r(1.321_f64,Ceil).unwrap(), 2_i16);
/// assert_eq!(to_i16r(1.567_f64,Floor).unwrap(), 1_i16);
/// assert!(to_i16r(-32768.5_f64,Round).is_err());
/// assert!(to_i16r(32768_f64,Trunc).is_err());
/// ```
pub fn to_i16r(d:f64, rmode:RoundingMode) -> ParseResultI16 {
    let min = i16::min_value() as f64;
    let max = i16::max_value() as f64;

    let x = match rmode{
        Round => d.round(),
        Ceil => d.ceil(),
        Floor => d.floor(),
        Trunc => d.trunc()
    };

    if x.is_nan() || x < min || x > max {
        Err(ParseError::InvalidNumber(d.to_string()))
    } else { Ok(x as i16) }
}

/// Converts `f64` to `i16`.
/// Returns `panic!` if `d` is less then `i16` min value,
/// `d` is greater then `i16` max value or `d` is nan.
/// otherwise returns the value of `d` as `i16`.
/// The `RoundingMode` is `Trunc`.
/// # Example
///
/// ```
/// use rustils::parse::double::to_i16p;
///
/// assert_eq!(to_i16p(-32768_f64), -32768_i16);
/// assert_eq!(to_i16p(32767_f64), 32767_i16);
/// assert_eq!(to_i16p(1.321_f64), 1_i16);
/// assert_eq!(to_i16p(1.567_f64), 1_i16);
/// ```
/// # Panics
///
/// ```rust,should_panic
/// rustils::parse::double::to_i16p(-32769_f64);
/// ```
/// ```rust,should_panic
/// rustils::parse::double::to_i16p(32768_f64);
/// ```
pub fn to_i16p(d:f64) -> i16 {
    match to_i16(d) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

/// Converts `f64` to `i16`.
/// Returns `panic!` if `d` is less then `i16` min value,
/// `d` is greater then `i16` max value or `d` is nan.
/// otherwise returns the value of `d` as `i16`.
/// # Example
///
/// ```
/// use rustils::utils::RoundingMode::{Trunc,Ceil,Floor};
/// use rustils::parse::double::to_i16pr;
///
/// assert_eq!(to_i16pr(-32768_f64,Trunc), -32768_i16);
/// assert_eq!(to_i16pr(32767_f64,Trunc), 32767_i16);
/// assert_eq!(to_i16pr(1.321_f64,Ceil), 2_i16);
/// assert_eq!(to_i16pr(1.567_f64,Floor), 1_i16);
/// ```
/// # Panics
///
/// ```rust,should_panic
/// use rustils::utils::RoundingMode::{Round};
/// rustils::parse::double::to_i16pr(-32768.5_f64,Round);
/// ```
/// ```rust,should_panic
/// use rustils::utils::RoundingMode::{Trunc};
/// rustils::parse::double::to_i16pr(32768_f64,Trunc);
/// ```
pub fn to_i16pr(d:f64, rmode:RoundingMode) -> i16 {
    match to_i16r(d,rmode) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

/// Converts `f64` to `i32`.
/// Returns `Err` if `d` is less then `i32` min value,
/// `d` is greater then `i32` max value or `d` is nan.
/// Otherwise returns the `Ok` value of `d` as `i32`.
/// The `RoundingMode` is `Trunc`.
/// # Example
///
/// ```
/// use rustils::parse::double::to_i32;
///
/// assert_eq!(to_i32(-2147483648_f64).unwrap(), -2147483648_i32);
/// assert_eq!(to_i32(2147483647_f64).unwrap(), 2147483647_i32);
/// assert_eq!(to_i32(1.321_f64).unwrap(), 1_i32);
/// assert_eq!(to_i32(1.567_f64).unwrap(), 1_i32);
/// assert!(to_i32(-2147483649_f64).is_err());
/// assert!(to_i32(2147483648_f64).is_err());
/// ```
pub fn to_i32(d:f64) -> ParseResultI32 {
    to_i32r(d,Trunc)
}

/// Converts `f64` to `i32`.
/// Returns `Err` if `d` is less then `i32` min value,
/// `d` is greater then `i32` max value or `d` is nan.
/// Otherwise returns the `Ok` value of `d` as `i32`.
/// # Example
///
/// ```
/// use rustils::utils::RoundingMode::{Trunc,Round,Ceil,Floor};
/// use rustils::parse::double::to_i32r;
///
/// assert_eq!(to_i32r(-2147483648_f64,Trunc).unwrap(), -2147483648_i32);
/// assert_eq!(to_i32r(2147483647_f64,Trunc).unwrap(), 2147483647_i32);
/// assert_eq!(to_i32r(1.321_f64,Ceil).unwrap(), 2_i32);
/// assert_eq!(to_i32r(1.567_f64,Floor).unwrap(), 1_i32);
/// assert!(to_i32r(-2147483648.5_f64,Round).is_err());
/// assert!(to_i32r(2147483648_f64,Trunc).is_err());
/// ```
pub fn to_i32r(d:f64, rmode:RoundingMode) -> ParseResultI32 {
    let min = i32::min_value() as f64;
    let max = i32::max_value() as f64;

    let x = match rmode{
        Round => d.round(),
        Ceil => d.ceil(),
        Floor => d.floor(),
        Trunc => d.trunc()
    };

    if x.is_nan() || x < min || x > max {
        Err(ParseError::InvalidNumber(d.to_string()))
    } else { Ok(x as i32) }
}

/// Converts `f64` to `i32`.
/// Returns `panic!` if `d` is less then `i16` min value,
/// `d` is greater then `i32` max value or `d` is nan.
/// otherwise returns the value of `d` as `i32`.
/// The `RoundingMode` is `Trunc`.
/// # Example
///
/// ```
/// use rustils::parse::double::to_i32p;
///
/// assert_eq!(to_i32p(-2147483648_f64), -2147483648_i32);
/// assert_eq!(to_i32p(2147483647_f64), 2147483647_i32);
/// assert_eq!(to_i32p(1.321_f64), 1_i32);
/// assert_eq!(to_i32p(1.567_f64), 1_i32);
/// ```
/// # Panics
///
/// ```rust,should_panic
/// rustils::parse::double::to_i32p(-2147483649_f64);
/// ```
/// ```rust,should_panic
/// rustils::parse::double::to_i32p(2147483648_f64);
/// ```
pub fn to_i32p(d:f64) -> i32 {
    match to_i32(d) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

/// Converts `f64` to `i32`.
/// Returns `panic!` if `d` is less then `i32` min value,
/// `d` is greater then `i32` max value or `d` is nan.
/// otherwise returns the value of `d` as `i32`.
/// # Example
///
/// ```
/// use rustils::utils::RoundingMode::{Trunc,Ceil,Floor};
/// use rustils::parse::double::to_i32pr;
///
/// assert_eq!(to_i32pr(-2147483648_f64,Trunc), -2147483648_i32);
/// assert_eq!(to_i32pr(2147483647_f64,Trunc), 2147483647_i32);
/// assert_eq!(to_i32pr(1.321_f64,Ceil), 2_i32);
/// assert_eq!(to_i32pr(1.567_f64,Floor), 1_i32);
/// ```
/// # Panics
///
/// ```rust,should_panic
/// use rustils::utils::RoundingMode::{Round};
/// rustils::parse::double::to_i32pr(-2147483648.5_f64,Round);
/// ```
/// ```rust,should_panic
/// use rustils::utils::RoundingMode::{Trunc};
/// rustils::parse::double::to_i32pr(2147483648_f64,Trunc);
/// ```
pub fn to_i32pr(d:f64, rmode:RoundingMode) -> i32 {
    match to_i32r(d,rmode) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

/// Converts `f64` to `i64`.
/// Returns `Err` if `d` is less then -9007199254740991,
/// `d` is greater then 9007199254740991 (53bit precision of f64 minus 1)
/// or `d` is nan.
/// Otherwise returns the `Ok` value of `d` as `i64`.
/// The `RoundingMode` is `Trunc`.
/// # Example
///
/// ```
/// use rustils::parse::double::to_i64;
///
/// assert_eq!(to_i64(-9007199254740991_f64).unwrap(), -9007199254740991_i64);
/// assert_eq!(to_i64(9007199254740991_f64).unwrap(), 9007199254740991_i64);
/// assert_eq!(to_i64(1.321_f64).unwrap(), 1_i64);
/// assert_eq!(to_i64(1.567_f64).unwrap(), 1_i64);
/// assert!(to_i64(-9007199254740992_f64).is_err());
/// assert!(to_i64(9007199254740992_f64).is_err());
/// ```
pub fn to_i64(d:f64) -> ParseResultI64 {
    to_i64r(d,Trunc)
}

/// Converts `f64` to `i64`.
/// Returns `Err` if `d` is less then -9007199254740991,
/// `d` is greater then 9007199254740991 (53bit precision of f64 minus 1)
/// or `d` is nan.
/// Otherwise returns the `Ok` value of `d` as `i64`.
/// # Example
///
/// ```
/// use rustils::utils::RoundingMode::{Trunc,Round,Ceil,Floor};
/// use rustils::parse::double::to_i64r;
///
/// assert_eq!(to_i64r(-9007199254740991_f64,Trunc).unwrap(), -9007199254740991_i64);
/// assert_eq!(to_i64r(9007199254740991_f64,Trunc).unwrap(), 9007199254740991_i64);
/// assert_eq!(to_i64r(1.321_f64,Ceil).unwrap(), 2_i64);
/// assert_eq!(to_i64r(1.567_f64,Floor).unwrap(), 1_i64);
/// assert!(to_i64r(-9007199254740991.5_f64,Round).is_err());
/// assert!(to_i64r(9007199254740992_f64,Trunc).is_err());
/// ```
pub fn to_i64r(d:f64, rmode:RoundingMode) -> ParseResultI64 {
    let min = -9007199254740991_f64;
    let max = 9007199254740991_f64;

    let x = match rmode{
        Round => d.round(),
        Ceil => d.ceil(),
        Floor => d.floor(),
        Trunc => d.trunc()
    };

    if x.is_nan() || x < min || x > max {
        Err(ParseError::InvalidNumber(d.to_string()))
    } else { Ok(x as i64) }
}

/// Converts `f64` to `i64`.
/// Returns `Err` if `d` is less then -9007199254740991,
/// `d` is greater then 9007199254740991 (53bit precision of f64 minus 1)
/// or `d` is nan.
/// otherwise returns the value of `d` as `i64`.
/// The `RoundingMode` is `Trunc`.
/// # Example
///
/// ```
/// use rustils::parse::double::to_i64p;
///
/// assert_eq!(to_i64p(-9007199254740991_f64), -9007199254740991_i64);
/// assert_eq!(to_i64p(9007199254740991_f64), 9007199254740991_i64);
/// assert_eq!(to_i64p(1.321_f64), 1_i64);
/// assert_eq!(to_i64p(1.567_f64), 1_i64);
/// ```
/// # Panics
///
/// ```rust,should_panic
/// rustils::parse::double::to_i64p(-9007199254740992_f64);
/// ```
/// ```rust,should_panic
/// rustils::parse::double::to_i64p(9007199254740992_f64);
/// ```
pub fn to_i64p(d:f64) -> i64 {
    match to_i64(d) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

/// Converts `f64` to `i64`.
/// Returns `Err` if `d` is less then -9007199254740991,
/// `d` is greater then 9007199254740991 (53bit precision of f64 minus 1)
/// or `d` is nan.
/// otherwise returns the value of `d` as `i64`.
/// # Example
///
/// ```
/// use rustils::utils::RoundingMode::{Trunc,Ceil,Floor};
/// use rustils::parse::double::to_i64pr;
///
/// assert_eq!(to_i64pr(-9007199254740991_f64,Trunc), -9007199254740991_i64);
/// assert_eq!(to_i64pr(9007199254740991_f64,Trunc), 9007199254740991_i64);
/// assert_eq!(to_i64pr(1.321_f64,Ceil), 2_i64);
/// assert_eq!(to_i64pr(1.567_f64,Floor), 1_i64);
/// ```
/// # Panics
///
/// ```rust,should_panic
/// use rustils::utils::RoundingMode::{Round};
/// rustils::parse::double::to_i64pr(9007199254740991.5_f64,Round);
/// ```
/// ```rust,should_panic
/// use rustils::utils::RoundingMode::{Trunc};
/// rustils::parse::double::to_i64pr(9007199254740992_f64,Trunc);
/// ```
pub fn to_i64pr(d:f64, rmode:RoundingMode) -> i64 {
    match to_i64r(d,rmode) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

/// Converts `f64` to `isize`.
/// Returns `Err` if `d` is less then -9007199254740991,
/// `d` is greater then 9007199254740991 (53bit precision of f64 minus 1)
/// or `d` is nan.
/// Otherwise returns the `Ok` value of `d` as `isize`.
/// The `RoundingMode` is `Trunc`.
/// # Example
///
/// ```
/// use rustils::parse::double::to_isize;
///
/// assert_eq!(to_isize(-9007199254740991_f64).unwrap(), -9007199254740991_isize);
/// assert_eq!(to_isize(9007199254740991_f64).unwrap(), 9007199254740991_isize);
/// assert_eq!(to_isize(1.321_f64).unwrap(), 1_isize);
/// assert_eq!(to_isize(1.567_f64).unwrap(), 1_isize);
/// assert!(to_isize(-9007199254740992_f64).is_err());
/// assert!(to_isize(9007199254740992_f64).is_err());
/// ```
pub fn to_isize(d:f64) -> ParseResultIsize {
    to_isizer(d,Trunc)
}

/// Converts `f64` to `isize`.
/// Returns `Err` if `d` is less then -9007199254740991,
/// `d` is greater then 9007199254740991 (53bit precision of f64 minus 1)
/// or `d` is nan.
/// Otherwise returns the `Ok` value of `d` as `isize`.
/// # Example
///
/// ```
/// use rustils::utils::RoundingMode::{Trunc,Round,Ceil,Floor};
/// use rustils::parse::double::to_isizer;
///
/// assert_eq!(to_isizer(-9007199254740991_f64,Trunc).unwrap(), -9007199254740991_isize);
/// assert_eq!(to_isizer(9007199254740991_f64,Trunc).unwrap(), 9007199254740991_isize);
/// assert_eq!(to_isizer(1.321_f64,Ceil).unwrap(), 2_isize);
/// assert_eq!(to_isizer(1.567_f64,Floor).unwrap(), 1_isize);
/// assert!(to_isizer(-9007199254740991.5_f64,Round).is_err());
/// assert!(to_isizer(9007199254740992_f64,Trunc).is_err());
/// ```
pub fn to_isizer(d:f64, rmode:RoundingMode) -> ParseResultIsize {
    let min = -9007199254740991_f64;
    let max = 9007199254740991_f64;

    let x = match rmode{
        Round => d.round(),
        Ceil => d.ceil(),
        Floor => d.floor(),
        Trunc => d.trunc()
    };

    if x.is_nan() || x < min || x > max {
        Err(ParseError::InvalidNumber(d.to_string()))
    } else { Ok(x as isize) }
}

/// Converts `f64` to `isize`.
/// Returns `Err` if `d` is less then -9007199254740991,
/// `d` is greater then 9007199254740991 (53bit precision of f64 minus 1)
/// or `d` is nan.
/// otherwise returns the value of `d` as `isize`.
/// The `RoundingMode` is `Trunc`.
/// # Example
///
/// ```
/// use rustils::parse::double::to_isizep;
///
/// assert_eq!(to_isizep(-9007199254740991_f64), -9007199254740991_isize);
/// assert_eq!(to_isizep(9007199254740991_f64), 9007199254740991_isize);
/// assert_eq!(to_isizep(1.321_f64), 1_isize);
/// assert_eq!(to_isizep(1.567_f64), 1_isize);
/// ```
/// # Panics
///
/// ```rust,should_panic
/// rustils::parse::double::to_isizep(-9007199254740992_f64);
/// ```
/// ```rust,should_panic
/// rustils::parse::double::to_isizep(9007199254740992_f64);
/// ```
pub fn to_isizep(d:f64) -> isize {
    match to_isize(d) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

/// Converts `f64` to `isize`.
/// Returns `Err` if `d` is less then -9007199254740991,
/// `d` is greater then 9007199254740991 (53bit precision of f64 minus 1)
/// or `d` is nan.
/// otherwise returns the value of `d` as `isize`.
/// # Example
///
/// ```
/// use rustils::utils::RoundingMode::{Trunc,Ceil,Floor};
/// use rustils::parse::double::to_isizepr;
///
/// assert_eq!(to_isizepr(-9007199254740991_f64,Trunc), -9007199254740991_isize);
/// assert_eq!(to_isizepr(9007199254740991_f64,Trunc), 9007199254740991_isize);
/// assert_eq!(to_isizepr(1.321_f64,Ceil), 2_isize);
/// assert_eq!(to_isizepr(1.567_f64,Floor), 1_isize);
/// ```
/// # Panics
///
/// ```rust,should_panic
/// use rustils::utils::RoundingMode::{Round};
/// rustils::parse::double::to_isizepr(9007199254740991.5_f64,Round);
/// ```
/// ```rust,should_panic
/// use rustils::utils::RoundingMode::{Trunc};
/// rustils::parse::double::to_isizepr(9007199254740992_f64,Trunc);
/// ```
pub fn to_isizepr(d:f64, rmode:RoundingMode) -> isize {
    match to_isizer(d,rmode) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

/// Converts `f64` to `u8`.
/// Returns `Err` if `d` is less then 0,
/// `d` is greater then `u8` max value or `d` is nan.
/// Otherwise returns the `Ok` value of `d` as `u8`.
/// The `RoundingMode` is `Trunc`.
/// # Example
///
/// ```
/// use rustils::parse::double::to_u8;
///
/// assert_eq!(to_u8(0_f64).unwrap(), 0_u8);
/// assert_eq!(to_u8(255_f64).unwrap(), 255_u8);
/// assert_eq!(to_u8(1.321_f64).unwrap(), 1_u8);
/// assert_eq!(to_u8(1.567_f64).unwrap(), 1_u8);
/// assert!(to_u8(-1_f64).is_err());
/// assert!(to_u8(256_f64).is_err());
/// ```
pub fn to_u8(d:f64) -> ParseResultU8 {
    to_u8r(d,Trunc)
}

/// Converts `f64` to `u8`.
/// Returns `Err` if `d` is less then 0,
/// `d` is greater then `u8` max value or `d` is nan.
/// Otherwise returns the `Ok` value of `d` as `u8`.
/// # Example
///
/// ```
/// use rustils::utils::RoundingMode::{Trunc,Round,Ceil,Floor};
/// use rustils::parse::double::to_u8r;
///
/// assert_eq!(to_u8r(0_f64,Trunc).unwrap(), 0_u8);
/// assert_eq!(to_u8r(255_f64,Trunc).unwrap(), 255_u8);
/// assert_eq!(to_u8r(1.321_f64,Ceil).unwrap(), 2_u8);
/// assert_eq!(to_u8r(1.567_f64,Floor).unwrap(), 1_u8);
/// assert!(to_u8r(-0.5_f64,Round).is_err());
/// assert!(to_u8r(256_f64,Trunc).is_err());
/// ```
pub fn to_u8r(d:f64, rmode:RoundingMode) -> ParseResultU8 {
    let max = u8::max_value() as f64;

    let x = match rmode{
        Round => d.round(),
        Ceil => d.ceil(),
        Floor => d.floor(),
        Trunc => d.trunc()
    };

    if x.is_nan() || x < 0.0 || x > max {
        Err(ParseError::InvalidNumber(d.to_string()))
    } else { Ok(x as u8) }
}

/// Converts `f64` to `u8`.
/// Returns `panic!` if `d` is less then 0,
/// `d` is greater then `u8` max value or `d` is nan.
/// otherwise returns the value of `d` as `u8`.
/// The `RoundingMode` is `Trunc`.
/// # Example
///
/// ```
/// use rustils::parse::double::to_u8p;
///
/// assert_eq!(to_u8p(0_f64), 0_u8);
/// assert_eq!(to_u8p(255_f64), 255_u8);
/// assert_eq!(to_u8p(1.321_f64), 1_u8);
/// assert_eq!(to_u8p(1.567_f64), 1_u8);
/// ```
/// # Panics
///
/// ```rust,should_panic
/// rustils::parse::double::to_u8p(-1_f64);
/// ```
/// ```rust,should_panic
/// rustils::parse::double::to_u8p(256_f64);
/// ```
pub fn to_u8p(d:f64) -> u8 {
    match to_u8(d) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

/// Converts `f64` to `u8`.
/// Returns `panic!` if `d` is less then 0,
/// `d` is greater then `u8` max value or `d` is nan.
/// otherwise returns the value of `d` as `u8`.
/// # Example
///
/// ```
/// use rustils::utils::RoundingMode::{Trunc,Ceil,Floor};
/// use rustils::parse::double::to_u8pr;
///
/// assert_eq!(to_u8pr(0_f64,Trunc), 0_u8);
/// assert_eq!(to_u8pr(255_f64,Trunc), 255_u8);
/// assert_eq!(to_u8pr(1.321_f64,Ceil), 2_u8);
/// assert_eq!(to_u8pr(1.567_f64,Floor), 1_u8);
/// ```
/// # Panics
///
/// ```rust,should_panic
/// use rustils::utils::RoundingMode::{Round};
/// rustils::parse::double::to_u8pr(-0.5_f64,Round);
/// ```
/// ```rust,should_panic
/// use rustils::utils::RoundingMode::{Trunc};
/// rustils::parse::double::to_u8pr(256_f64,Trunc);
/// ```
pub fn to_u8pr(d:f64, rmode:RoundingMode) -> u8 {
    match to_u8r(d,rmode) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

/// Converts `f64` to `u16`.
/// Returns `Err` if `d` is less then 0,
/// `d` is greater then `u16` max value or `d` is nan.
/// Otherwise returns the `Ok` value of `d` as `u16`.
/// The `RoundingMode` is `Trunc`.
/// # Example
///
/// ```
/// use rustils::parse::double::to_u16;
///
/// assert_eq!(to_u16(0_f64).unwrap(), 0_u16);
/// assert_eq!(to_u16(65535_f64).unwrap(), 65535_u16);
/// assert_eq!(to_u16(1.321_f64).unwrap(), 1_u16);
/// assert_eq!(to_u16(1.567_f64).unwrap(), 1_u16);
/// assert!(to_u16(-1_f64).is_err());
/// assert!(to_u16(65536_f64).is_err());
/// ```
pub fn to_u16(d:f64) -> ParseResultU16 {
    to_u16r(d,Trunc)
}

/// Converts `f64` to `u16`.
/// Returns `Err` if `d` is less then 0,
/// `d` is greater then `u16` max value or `d` is nan.
/// Otherwise returns the `Ok` value of `d` as `u16`.
/// # Example
///
/// ```
/// use rustils::utils::RoundingMode::{Trunc,Round,Ceil,Floor};
/// use rustils::parse::double::to_u16r;
///
/// assert_eq!(to_u16r(0_f64,Trunc).unwrap(), 0_u16);
/// assert_eq!(to_u16r(65535_f64,Trunc).unwrap(), 65535_u16);
/// assert_eq!(to_u16r(1.321_f64,Ceil).unwrap(), 2_u16);
/// assert_eq!(to_u16r(1.567_f64,Floor).unwrap(), 1_u16);
/// assert!(to_u16r(-0.5_f64,Round).is_err());
/// assert!(to_u16r(65536_f64,Trunc).is_err());
/// ```
pub fn to_u16r(d:f64, rmode:RoundingMode) -> ParseResultU16 {
    let max = u16::max_value() as f64;

    let x = match rmode{
        Round => d.round(),
        Ceil => d.ceil(),
        Floor => d.floor(),
        Trunc => d.trunc()
    };

    if x.is_nan() || x < 0.0 || x > max {
        Err(ParseError::InvalidNumber(d.to_string()))
    } else { Ok(x as u16) }
}

/// Converts `f64` to `u16`.
/// Returns `panic!` if `d` is less then 0,
/// `d` is greater then `u16` max value or `d` is nan.
/// otherwise returns the value of `d` as `u16`.
/// The `RoundingMode` is `Trunc`.
/// # Example
///
/// ```
/// use rustils::parse::double::to_u16p;
///
/// assert_eq!(to_u16p(0_f64), 0_u16);
/// assert_eq!(to_u16p(65535_f64), 65535_u16);
/// assert_eq!(to_u16p(1.321_f64), 1_u16);
/// assert_eq!(to_u16p(1.567_f64), 1_u16);
/// ```
/// # Panics
///
/// ```rust,should_panic
/// rustils::parse::double::to_u16p(-1_f64);
/// ```
/// ```rust,should_panic
/// rustils::parse::double::to_u16p(65536_f64);
/// ```
pub fn to_u16p(d:f64) -> u16 {
    match to_u16(d) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

/// Converts `f64` to `u16`.
/// Returns `panic!` if `d` is less then 0,
/// `d` is greater then `u16` max value or `d` is nan.
/// otherwise returns the value of `d` as `u16`.
/// # Example
///
/// ```
/// use rustils::utils::RoundingMode::{Trunc,Ceil,Floor};
/// use rustils::parse::double::to_u16pr;
///
/// assert_eq!(to_u16pr(0_f64,Trunc), 0_u16);
/// assert_eq!(to_u16pr(65535_f64,Trunc), 65535_u16);
/// assert_eq!(to_u16pr(1.321_f64,Ceil), 2_u16);
/// assert_eq!(to_u16pr(1.567_f64,Floor), 1_u16);
/// ```
/// # Panics
///
/// ```rust,should_panic
/// use rustils::utils::RoundingMode::{Round};
/// rustils::parse::double::to_u16pr(-0.5_f64,Round);
/// ```
/// ```rust,should_panic
/// use rustils::utils::RoundingMode::{Trunc};
/// rustils::parse::double::to_u16pr(65536_f64,Trunc);
/// ```
pub fn to_u16pr(d:f64, rmode:RoundingMode) -> u16 {
    match to_u16r(d,rmode) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

/// Converts `f64` to `u32`.
/// Returns `Err` if `d` is less then 0,
/// `d` is greater then `u32` max value or `d` is nan.
/// Otherwise returns the `Ok` value of `d` as `u32`.
/// The `RoundingMode` is `Trunc`.
/// # Example
///
/// ```
/// use rustils::parse::double::to_u32;
///
/// assert_eq!(to_u32(0_f64).unwrap(), 0_u32);
/// assert_eq!(to_u32(4294967295_f64).unwrap(), 4294967295_u32);
/// assert_eq!(to_u32(1.321_f64).unwrap(), 1_u32);
/// assert_eq!(to_u32(1.567_f64).unwrap(), 1_u32);
/// assert!(to_u32(-1_f64).is_err());
/// assert!(to_u32(4294967296_f64).is_err());
/// ```
pub fn to_u32(d:f64) -> ParseResultU32 {
    to_u32r(d,Trunc)
}

/// Converts `f64` to `u32`.
/// Returns `Err` if `d` is less then 0,
/// `d` is greater then `u32` max value or `d` is nan.
/// Otherwise returns the `Ok` value of `d` as `u32`.
/// # Example
///
/// ```
/// use rustils::utils::RoundingMode::{Trunc,Round,Ceil,Floor};
/// use rustils::parse::double::to_u32r;
///
/// assert_eq!(to_u32r(0_f64,Trunc).unwrap(), 0_u32);
/// assert_eq!(to_u32r(4294967295_f64,Trunc).unwrap(), 4294967295_u32);
/// assert_eq!(to_u32r(1.321_f64,Ceil).unwrap(), 2_u32);
/// assert_eq!(to_u32r(1.567_f64,Floor).unwrap(), 1_u32);
/// assert!(to_u32r(-0.5_f64,Round).is_err());
/// assert!(to_u32r(4294967296_f64,Trunc).is_err());
/// ```
pub fn to_u32r(d:f64, rmode:RoundingMode) -> ParseResultU32 {
    let max = u32::max_value() as f64;

    let x = match rmode{
        Round => d.round(),
        Ceil => d.ceil(),
        Floor => d.floor(),
        Trunc => d.trunc()
    };

    if x.is_nan() || x < 0.0 || x > max {
        Err(ParseError::InvalidNumber(d.to_string()))
    } else { Ok(x as u32) }
}

/// Converts `f64` to `u32`.
/// Returns `panic!` if `d` is less then 0,
/// `d` is greater then `u32` max value or `d` is nan.
/// otherwise returns the value of `d` as `u32`.
/// The `RoundingMode` is `Trunc`.
/// # Example
///
/// ```
/// use rustils::parse::double::to_u32p;
///
/// assert_eq!(to_u32p(0_f64), 0_u32);
/// assert_eq!(to_u32p(4294967295_f64), 4294967295_u32);
/// assert_eq!(to_u32p(1.321_f64), 1_u32);
/// assert_eq!(to_u32p(1.567_f64), 1_u32);
/// ```
/// # Panics
///
/// ```rust,should_panic
/// rustils::parse::double::to_u32p(-1_f64);
/// ```
/// ```rust,should_panic
/// rustils::parse::double::to_u32p(4294967296_f64);
/// ```
pub fn to_u32p(d:f64) -> u32 {
    match to_u32(d) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

/// Converts `f64` to `u32`.
/// Returns `panic!` if `d` is less then 0,
/// `d` is greater then `u32` max value or `d` is nan.
/// otherwise returns the value of `d` as `u32`.
/// # Example
///
/// ```
/// use rustils::utils::RoundingMode::{Trunc,Ceil,Floor};
/// use rustils::parse::double::to_u32pr;
///
/// assert_eq!(to_u32pr(0_f64,Trunc), 0_u32);
/// assert_eq!(to_u32pr(4294967295_f64,Trunc), 4294967295_u32);
/// assert_eq!(to_u32pr(1.321_f64,Ceil), 2_u32);
/// assert_eq!(to_u32pr(1.567_f64,Floor), 1_u32);
/// ```
/// # Panics
///
/// ```rust,should_panic
/// use rustils::utils::RoundingMode::{Round};
/// rustils::parse::double::to_u32pr(-0.5_f64,Round);
/// ```
/// ```rust,should_panic
/// use rustils::utils::RoundingMode::{Trunc};
/// rustils::parse::double::to_u32pr(4294967296_f64,Trunc);
/// ```
pub fn to_u32pr(d:f64, rmode:RoundingMode) -> u32 {
    match to_u32r(d,rmode) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

/// Converts `f64` to `u64`.
/// Returns `Err` if `d` is less 0,
/// `d` is greater then 9007199254740991 (53bit precision of f64 minus 1)
/// or `d` is nan.
/// Otherwise returns the `Ok` value of `d` as `u64`.
/// The `RoundingMode` is `Trunc`.
/// # Example
///
/// ```
/// use rustils::parse::double::to_u64;
///
/// assert_eq!(to_u64(0_f64).unwrap(), 0_u64);
/// assert_eq!(to_u64(9007199254740991_f64).unwrap(), 9007199254740991_u64);
/// assert_eq!(to_u64(1.321_f64).unwrap(), 1_u64);
/// assert_eq!(to_u64(1.567_f64).unwrap(), 1_u64);
/// assert!(to_u64(-1_f64).is_err());
/// assert!(to_u64(9007199254740992_f64).is_err());
/// ```
pub fn to_u64(d:f64) -> ParseResultU64 {
    to_u64r(d,Trunc)
}

/// Converts `f64` to `u64`.
/// Returns `Err` if `d` is less 0,
/// `d` is greater then 9007199254740991 (53bit precision of f64 minus 1)
/// or `d` is nan.
/// Otherwise returns the `Ok` value of `d` as `u64`.
/// # Example
///
/// ```
/// use rustils::utils::RoundingMode::{Trunc,Round,Ceil,Floor};
/// use rustils::parse::double::to_u64r;
///
/// assert_eq!(to_u64r(0_f64,Trunc).unwrap(), 0_u64);
/// assert_eq!(to_u64r(9007199254740991_f64,Trunc).unwrap(), 9007199254740991_u64);
/// assert_eq!(to_u64r(1.321_f64,Ceil).unwrap(), 2_u64);
/// assert_eq!(to_u64r(1.567_f64,Floor).unwrap(), 1_u64);
/// assert!(to_u64r(-0.5_f64,Round).is_err());
/// assert!(to_u64r(9007199254740992_f64,Trunc).is_err());
/// ```
pub fn to_u64r(d:f64, rmode:RoundingMode) -> ParseResultU64 {
    let max = 9007199254740991_f64;

    let x = match rmode{
        Round => d.round(),
        Ceil => d.ceil(),
        Floor => d.floor(),
        Trunc => d.trunc()
    };

    if x.is_nan() || x < 0.0 || x > max {
        Err(ParseError::InvalidNumber(d.to_string()))
    } else { Ok(x as u64) }
}

/// Converts `f64` to `u64`.
/// Returns `Err` if `d` is less 0,
/// `d` is greater then 9007199254740991 (53bit precision of f64 minus 1)
/// or `d` is nan.
/// otherwise returns the value of `d` as `u64`.
/// The `RoundingMode` is `Trunc`.
/// # Example
///
/// ```
/// use rustils::parse::double::to_u64p;
///
/// assert_eq!(to_u64p(0_f64), 0_u64);
/// assert_eq!(to_u64p(9007199254740991_f64), 9007199254740991_u64);
/// assert_eq!(to_u64p(1.321_f64), 1_u64);
/// assert_eq!(to_u64p(1.567_f64), 1_u64);
/// ```
/// # Panics
///
/// ```rust,should_panic
/// rustils::parse::double::to_u64p(-1_f64);
/// ```
/// ```rust,should_panic
/// rustils::parse::double::to_u64p(9007199254740992_f64);
/// ```
pub fn to_u64p(d:f64) -> u64 {
    match to_u64(d) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

/// Converts `f64` to `u64`.
/// Returns `Err` if `d` is less 0,
/// `d` is greater then 9007199254740991 (53bit precision of f64 minus 1)
/// or `d` is nan.
/// otherwise returns the value of `d` as `u64`.
/// # Example
///
/// ```
/// use rustils::utils::RoundingMode::{Trunc,Ceil,Floor};
/// use rustils::parse::double::to_u64pr;
///
/// assert_eq!(to_u64pr(0_f64,Trunc), 0_u64);
/// assert_eq!(to_u64pr(9007199254740991_f64,Trunc), 9007199254740991_u64);
/// assert_eq!(to_u64pr(1.321_f64,Ceil), 2_u64);
/// assert_eq!(to_u64pr(1.567_f64,Floor), 1_u64);
/// ```
/// # Panics
///
/// ```rust,should_panic
/// use rustils::utils::RoundingMode::{Round};
/// rustils::parse::double::to_u64pr(-0.5_f64,Round);
/// ```
/// ```rust,should_panic
/// use rustils::utils::RoundingMode::{Trunc};
/// rustils::parse::double::to_u64pr(9007199254740992_f64,Trunc);
/// ```
pub fn to_u64pr(d:f64, rmode:RoundingMode) -> u64 {
    match to_u64r(d,rmode) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

/// Converts `f64` to `usize`.
/// Returns `Err` if `d` is less 0,
/// `d` is greater then 9007199254740991 (53bit precision of f64 minus 1)
/// or `d` is nan.
/// Otherwise returns the `Ok` value of `d` as `usize`.
/// The `RoundingMode` is `Trunc`.
/// # Example
///
/// ```
/// use rustils::parse::double::to_usize;
///
/// assert_eq!(to_usize(0_f64).unwrap(), 0_usize);
/// assert_eq!(to_usize(9007199254740991_f64).unwrap(), 9007199254740991_usize);
/// assert_eq!(to_usize(1.321_f64).unwrap(), 1_usize);
/// assert_eq!(to_usize(1.567_f64).unwrap(), 1_usize);
/// assert!(to_usize(-1_f64).is_err());
/// assert!(to_usize(9007199254740992_f64).is_err());
/// ```
pub fn to_usize(d:f64) -> ParseResultUsize {
    to_usizer(d,Trunc)
}

/// Converts `f64` to `usize`.
/// Returns `Err` if `d` is less 0,
/// `d` is greater then 9007199254740991 (53bit precision of f64 minus 1)
/// or `d` is nan.
/// Otherwise returns the `Ok` value of `d` as `usize`.
/// # Example
///
/// ```
/// use rustils::utils::RoundingMode::{Trunc,Round,Ceil,Floor};
/// use rustils::parse::double::to_usizer;
///
/// assert_eq!(to_usizer(0_f64,Trunc).unwrap(), 0_usize);
/// assert_eq!(to_usizer(9007199254740991_f64,Trunc).unwrap(), 9007199254740991_usize);
/// assert_eq!(to_usizer(1.321_f64,Ceil).unwrap(), 2_usize);
/// assert_eq!(to_usizer(1.567_f64,Floor).unwrap(), 1_usize);
/// assert!(to_usizer(-0.5_f64,Round).is_err());
/// assert!(to_usizer(9007199254740992_f64,Trunc).is_err());
/// ```
pub fn to_usizer(d:f64, rmode:RoundingMode) -> ParseResultUsize {
    let max = 9007199254740991_f64;

    let x = match rmode{
        Round => d.round(),
        Ceil => d.ceil(),
        Floor => d.floor(),
        Trunc => d.trunc()
    };

    if x.is_nan() || x < 0.0 || x > max {
        Err(ParseError::InvalidNumber(d.to_string()))
    } else { Ok(x as usize) }
}

/// Converts `f64` to `usize`.
/// Returns `Err` if `d` is less 0,
/// `d` is greater then 9007199254740991 (53bit precision of f64 minus 1)
/// or `d` is nan.
/// otherwise returns the value of `d` as `usize`.
/// The `RoundingMode` is `Trunc`.
/// # Example
///
/// ```
/// use rustils::parse::double::to_usizep;
///
/// assert_eq!(to_usizep(0_f64), 0_usize);
/// assert_eq!(to_usizep(9007199254740991_f64), 9007199254740991_usize);
/// assert_eq!(to_usizep(1.321_f64), 1_usize);
/// assert_eq!(to_usizep(1.567_f64), 1_usize);
/// ```
/// # Panics
///
/// ```rust,should_panic
/// rustils::parse::double::to_usizep(-1_f64);
/// ```
/// ```rust,should_panic
/// rustils::parse::double::to_usizep(9007199254740992_f64);
/// ```
pub fn to_usizep(d:f64) -> usize {
    match to_usize(d) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

/// Converts `f64` to `usize`.
/// Returns `Err` if `d` is less 0,
/// `d` is greater then 9007199254740991 (53bit precision of f64 minus 1)
/// or `d` is nan.
/// otherwise returns the value of `d` as `usize`.
/// # Example
///
/// ```
/// use rustils::utils::RoundingMode::{Trunc,Ceil,Floor};
/// use rustils::parse::double::to_usizepr;
///
/// assert_eq!(to_usizepr(0_f64,Trunc), 0_usize);
/// assert_eq!(to_usizepr(9007199254740991_f64,Trunc), 9007199254740991_usize);
/// assert_eq!(to_usizepr(1.321_f64,Ceil), 2_usize);
/// assert_eq!(to_usizepr(1.567_f64,Floor), 1_usize);
/// ```
/// # Panics
///
/// ```rust,should_panic
/// use rustils::utils::RoundingMode::{Round};
/// rustils::parse::double::to_usizepr(-0.5_f64,Round);
/// ```
/// ```rust,should_panic
/// use rustils::utils::RoundingMode::{Trunc};
/// rustils::parse::double::to_usizepr(9007199254740992_f64,TRUNC);
/// ```
pub fn to_usizepr(d:f64, rmode:RoundingMode) -> usize {
    match to_usizer(d,rmode) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}
