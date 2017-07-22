use error::ParseError;
use parse::RoundingMode;
use parse::RoundingMode::*;

// <editor-fold> # Traits

pub trait ToI8 {

    fn to_i8_res(self)
        -> ParseResultI8;

    fn to_i8(self)
        -> i8;
}

pub trait ToI8RM {

    fn to_i8_rm_res(self, rm: RoundingMode)
        -> ParseResultI8;

    fn to_i8_rm(self, rm: RoundingMode)
        -> i8;
}
// </editor-fold>

// <editor-fold> # Functions

// <editor-fold> ## bool

pub fn bool_to_i8_res(a: bool)
    -> ParseResultI8 {

    if a { Ok(1) } else { Ok(0) }
}

pub fn bool_to_i8(a: bool)
    -> i8 {

    if a { 1 } else { 0 }
}
// </editor-fold>

// <editor-fold> ## 8

/// Parse [`u8`](https://doc.rust-lang.org/std/primitive.u8.html) to [`i8`](https://doc.rust-lang.org/std/primitive.i8.html)
///
///
///
///
/// # Arguments
///
/// * `a` - Any [`u8`](https://doc.rust-lang.org/std/primitive.u8.html) number
///
/// # Examples
///
/// ```
/// extern crate rustils;
///
/// use rustils::parse::byte::u8_to_i8_res;
/// use rustils::error::ParseError::InvalidNumber;
///
/// let a = 42_u8;
/// let b = 128_u8;
///
/// assert_eq!(
///     u8_to_i8_res(a),
///     Ok(42_i8));
///
/// assert_eq!(
///     u8_to_i8_res(b),
///     Err(InvalidNumber("128")));
/// ```
pub fn u8_to_i8_res(a: u8)
    -> ParseResultI8 {

    let max = i8::max_value() as u8;

    if a > max {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(a as i8) }
}

/// Parse [`u8`](https://doc.rust-lang.org/std/primitive.u8.html) to [`i8`](https://doc.rust-lang.org/std/primitive.i8.html)
///
///
///
///
/// # Arguments
///
/// * `a` - Any [`u8`](https://doc.rust-lang.org/std/primitive.u8.html) number
///
/// # Examples
///
/// ```
/// extern crate rustils;
///
/// use rustils::parse::byte::u8_to_i8;
/// use rustils::error::ParseError::InvalidNumber;
///
/// let a = 42_u8;
///
/// assert_eq!(
///     u8_to_i8_res(a),
///     42_i8);
/// ```
pub fn u8_to_i8(a: u8)
    -> i8 {

    match u8_to_i8_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}
// </editor-fold>

// <editor-fold> ## 16

/// Parse [`i16`](https://doc.rust-lang.org/std/primitive.i16.html) to [`i8`](https://doc.rust-lang.org/std/primitive.i8.html)
///
///
///
///
/// # Arguments
///
/// * `a` - Any [`i16`](https://doc.rust-lang.org/std/primitive.i16.html) number
///
/// # Examples
///
/// ```
/// rustils::parse::byte::i16_to_i8_res;
///
///
///
/// ```
pub fn i16_to_i8_res(a: i16)
    -> ParseResultI8 {

    let min = i8::min_value() as i16;
    let max = i8::max_value() as i16;

    if a < min || a > max {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(a as i8) }
}

/// Parse [`i16`](https://doc.rust-lang.org/std/primitive.i16.html) to [`i8`](https://doc.rust-lang.org/std/primitive.i8.html)
///
///
///
///
/// # Arguments
///
/// * `a` - Any [`i16`](https://doc.rust-lang.org/std/primitive.i16.html) number
///
/// # Examples
///
/// ```
/// rustils::parse::byte::i16_to_i8;
///
///
///
/// ```
pub fn i16_to_i8(a: i16)
    -> i8 {

    match i16_to_i8_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

/// Parse [`u16`](https://doc.rust-lang.org/std/primitive.u16.html) to [`i8`](https://doc.rust-lang.org/std/primitive.i8.html)
///
///
///
///
/// # Arguments
///
/// * `a` - Any [`u16`](https://doc.rust-lang.org/std/primitive.u16.html) number
///
/// # Examples
///
/// ```
/// rustils::parse::byte::u16_to_i8_res;
///
///
///
/// ```
pub fn u16_to_i8_res(a: u16)
    -> ParseResultI8 {

    let max = i8::max_value() as u16;

    if a > max {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(a as i8) }
}

/// Parse [`u16`](https://doc.rust-lang.org/std/primitive.u16.html) to [`i8`](https://doc.rust-lang.org/std/primitive.i8.html)
///
///
///
///
/// # Arguments
///
/// * `a` - Any [`u16`](https://doc.rust-lang.org/std/primitive.u16.html) number
///
/// # Examples
///
/// ```
/// rustils::parse::byte::u16_to_i8;
///
///
///
/// ```
pub fn u16_to_i8(a: u16)
    -> i8 {

    match u16_to_i8_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}
// </editor-fold>

// <editor-fold> ## 32

/// Parse [`i32`](https://doc.rust-lang.org/std/primitive.i32.html) to [`i8`](https://doc.rust-lang.org/std/primitive.i8.html)
///
///
///
///
/// # Arguments
///
/// * `a` - Any [`i32`](https://doc.rust-lang.org/std/primitive.i32.html) number
///
/// # Examples
///
/// ```
/// rustils::parse::byte::i32_to_i8_res;
///
///
///
/// ```
pub fn i32_to_i8_res(a: i32)
    -> ParseResultI8 {

    let min = i8::min_value() as i32;
    let max = i8::max_value() as i32;

    if a < min || a > max {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(a as i8) }
}

/// Parse [`i32`](https://doc.rust-lang.org/std/primitive.i32.html) to [`i8`](https://doc.rust-lang.org/std/primitive.i8.html)
///
///
///
///
/// # Arguments
///
/// * `a` - Any [`i32`](https://doc.rust-lang.org/std/primitive.i32.html) number
///
/// # Examples
///
/// ```
/// rustils::parse::byte::i32_to_i8;
///
///
///
/// ```
pub fn i32_to_i8(a: i32)
    -> i8 {

    match i32_to_i8_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

/// Parse [`u32`](https://doc.rust-lang.org/std/primitive.u32.html) to [`i8`](https://doc.rust-lang.org/std/primitive.i8.html)
///
///
///
///
/// # Arguments
///
/// * `a` - Any [`u32`](https://doc.rust-lang.org/std/primitive.u32.html) number
///
/// # Examples
///
/// ```
/// rustils::parse::byte::u32_to_i8_res;
///
///
///
/// ```
pub fn u32_to_i8_res(a: u32)
    -> ParseResultI8 {

    let max = i8::max_value() as u32;

    if a > max {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(a as i8) }
}

/// Parse [`u32`](https://doc.rust-lang.org/std/primitive.u32.html) to [`i8`](https://doc.rust-lang.org/std/primitive.i8.html)
///
///
///
///
/// # Arguments
///
/// * `a` - Any [`u32`](https://doc.rust-lang.org/std/primitive.u32.html) number
///
/// # Examples
///
/// ```
/// rustils::parse::byte::u32_to_i8;
///
///
///
/// ```
pub fn u32_to_i8(a: u32)
    -> i8 {

    match u32_to_i8_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

/// Parse [`f32`](https://doc.rust-lang.org/std/primitive.f32.html) to [`i8`](https://doc.rust-lang.org/std/primitive.i8.html)
///
///
///
///
/// # Arguments
///
/// * `a` - Any [`f32`](https://doc.rust-lang.org/std/primitive.f32.html) number
///
/// # Examples
///
/// ```
/// rustils::parse::byte::f32_to_i8_res;
///
///
///
/// ```
pub fn f32_to_i8_res(a: f32)
    -> ParseResultI8{

    f32_to_i8_rm_res(a, Trunc)
}

/// Parse [`f32`](https://doc.rust-lang.org/std/primitive.f32.html) to [`i8`](https://doc.rust-lang.org/std/primitive.i8.html)
///
///
///
///
/// # Arguments
///
/// * `a` - Any [`f32`](https://doc.rust-lang.org/std/primitive.f32.html) number
///
/// # Examples
///
/// ```
/// rustils::parse::byte::f32_to_i8;
///
///
///
/// ```
pub fn f32_to_i8(a: f32)
    -> i8 {

    f32_to_i8_rm(a, Trunc)
}

/// Parse [`f32`](https://doc.rust-lang.org/std/primitive.f32.html) to [`i8`](https://doc.rust-lang.org/std/primitive.i8.html)
///
///
///
///
/// # Arguments
///
/// * `a` - Any [`f32`](https://doc.rust-lang.org/std/primitive.f32.html) number
///
/// # Examples
///
/// ```
/// rustils::parse::byte::f32_to_i8_rm_res;
///
///
///
/// ```
pub fn f32_to_i8_rm_res(a: f32, rm: RoundingMode)
    -> ParseResultI8 {

    let min = i8::min_value() as f32;
    let max = i8::max_value() as f32;

    let x = match rm {
        Round => a.round(),
        Ceil => a.ceil(),
        Floor => a.floor(),
        Trunc => a.trunc()
    };

    if x.is_nan() || x < min || x > max {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(x as i8) }
}

/// Parse [`f32`](https://doc.rust-lang.org/std/primitive.f32.html) to [`i8`](https://doc.rust-lang.org/std/primitive.i8.html)
///
///
///
///
/// # Arguments
///
/// * `a` - Any [`f32`](https://doc.rust-lang.org/std/primitive.f32.html) number
///
/// # Examples
///
/// ```
/// rustils::parse::byte::f32_to_i8_rm;
///
///
///
/// ```
pub fn f32_to_i8_rm(a: f32, rm: RoundingMode)
    -> i8 {

    match f32_to_i8_rm_res(a, rm) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}
// </editor-fold>

// <editor-fold> ## 64

/// Parse [`i64`](https://doc.rust-lang.org/std/primitive.i64.html) to [`i8`](https://doc.rust-lang.org/std/primitive.i8.html)
///
///
///
///
/// # Arguments
///
/// * `a` - Any [`i64`](https://doc.rust-lang.org/std/primitive.i64.html) number
///
/// # Examples
///
/// ```
/// rustils::parse::byte::i64_to_i8_res;
///
///
///
/// ```
pub fn i64_to_i8_res(a: i64)
    -> ParseResultI8 {

    let min = i8::min_value() as i64;
    let max = i8::max_value() as i64;

    if a < min || a > max {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(a as i8) }
}

/// Parse [`i64`](https://doc.rust-lang.org/std/primitive.i64.html) to [`i8`](https://doc.rust-lang.org/std/primitive.i8.html)
///
///
///
///
/// # Arguments
///
/// * `a` - Any [`i64`](https://doc.rust-lang.org/std/primitive.i64.html) number
///
/// # Examples
///
/// ```
/// rustils::parse::byte::i64_to_i8;
///
///
///
/// ```
pub fn i64_to_i8(a: i64)
    -> i8 {

    match i64_to_i8_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

/// Parse [`u64`](https://doc.rust-lang.org/std/primitive.u64.html) to [`i8`](https://doc.rust-lang.org/std/primitive.i8.html)
///
///
///
///
/// # Arguments
///
/// * `a` - Any [`u64`](https://doc.rust-lang.org/std/primitive.u64.html) number
///
/// # Examples
///
/// ```
/// rustils::parse::byte::u64_to_i8_res;
///
///
///
/// ```
pub fn u64_to_i8_res(a: u64)
    -> ParseResultI8 {

    let max = i8::max_value() as u64;

    if a > max {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(a as i8) }
}

/// Parse [`u64`](https://doc.rust-lang.org/std/primitive.u64.html) to [`i8`](https://doc.rust-lang.org/std/primitive.i8.html)
///
///
///
///
/// # Arguments
///
/// * `a` - Any [`u64`](https://doc.rust-lang.org/std/primitive.u64.html) number
///
/// # Examples
///
/// ```
/// rustils::parse::byte::u64_to_i8;
///
///
///
/// ```
pub fn u64_to_i8(a: u64)
    -> i8 {

    match u64_to_i8_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

/// Parse [`f64`](https://doc.rust-lang.org/std/primitive.f64.html) to [`i8`](https://doc.rust-lang.org/std/primitive.i8.html)
///
///
///
///
/// # Arguments
///
/// * `a` - Any [`f64`](https://doc.rust-lang.org/std/primitive.f64.html) number
///
/// # Examples
///
/// ```
/// rustils::parse::byte::f64_to_i8_res;
///
///
///
/// ```
pub fn f64_to_i8_res(a: f64)
    -> ParseResultI8 {

    f64_to_i8_rm_res(a, Trunc)
}

/// Parse [`f64`](https://doc.rust-lang.org/std/primitive.f64.html) to [`i8`](https://doc.rust-lang.org/std/primitive.i8.html)
///
///
///
///
/// # Arguments
///
/// * `a` - Any [`f64`](https://doc.rust-lang.org/std/primitive.f64.html) number
///
/// # Examples
///
/// ```
/// rustils::parse::byte::f64_to_i8;
///
///
///
/// ```
pub fn f64_to_i8(a: f64)
    -> i8 {

    f64_to_i8_rm(a, Trunc)
}

/// Parse [`f64`](https://doc.rust-lang.org/std/primitive.f64.html) to [`i8`](https://doc.rust-lang.org/std/primitive.i8.html)
///
///
///
///
/// # Arguments
///
/// * `a` - Any [`f64`](https://doc.rust-lang.org/std/primitive.f64.html) number
///
/// # Examples
///
/// ```
/// rustils::parse::byte::f64_to_i8_rm_res;
///
///
///
/// ```
pub fn f64_to_i8_rm_res(a: f64, rm: RoundingMode)
    -> ParseResultI8 {

    let min = i8::min_value() as f64;
    let max = i8::max_value() as f64;

    let x = match rm {
        Round => a.round(),
        Ceil => a.ceil(),
        Floor => a.floor(),
        Trunc => a.trunc()
    };

    if x.is_nan() || x < min || x > max {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(x as i8) }
}

/// Parse [`f64`](https://doc.rust-lang.org/std/primitive.f64.html) to [`i8`](https://doc.rust-lang.org/std/primitive.i8.html)
///
///
///
///
/// # Arguments
///
/// * `a` - Any [`f64`](https://doc.rust-lang.org/std/primitive.f64.html) number
///
/// # Examples
///
/// ```
/// rustils::parse::byte::f64_to_i8_rm;
///
///
///
/// ```
pub fn f64_to_i8_rm(a: f64, rm: RoundingMode)
    -> i8 {

    match f64_to_i8_rm_res(a, rm) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}
// </editor-fold>

// <editor-fold> ## size

/// Parse [`isize`](https://doc.rust-lang.org/std/primitive.isize.html) to [`i8`](https://doc.rust-lang.org/std/primitive.i8.html)
///
///
///
///
/// # Arguments
///
/// * `a` - Any [`isize`](https://doc.rust-lang.org/std/primitive.isize.html) number
///
/// # Examples
///
/// ```
/// rustils::parse::byte::isize_to_i8_res;
///
///
///
/// ```
pub fn isize_to_i8_res(a: isize)
    -> ParseResultI8 {

    let min = i8::min_value() as isize;
    let max = i8::max_value() as isize;

    if a < min || a > max {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(a as i8) }
}

/// Parse [`isize`](https://doc.rust-lang.org/std/primitive.isize.html) to [`i8`](https://doc.rust-lang.org/std/primitive.i8.html)
///
///
///
///
/// # Arguments
///
/// * `a` - Any [`isize`](https://doc.rust-lang.org/std/primitive.isize.html) number
///
/// # Examples
///
/// ```
/// rustils::parse::byte::isize_to_i8;
///
///
///
/// ```
pub fn isize_to_i8(a: isize)
    -> i8 {

    match isize_to_i8_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

/// Parse [`usize`](https://doc.rust-lang.org/std/primitive.usize.html) to [`i8`](https://doc.rust-lang.org/std/primitive.i8.html)
///
///
///
///
/// # Arguments
///
/// * `a` - Any [`usize`](https://doc.rust-lang.org/std/primitive.usize.html) number
///
/// # Examples
///
/// ```
/// rustils::parse::byte::usize_to_i8_res;
///
///
///
/// ```
pub fn usize_to_i8_res(a: usize)
    -> ParseResultI8 {

    let max = i8::max_value() as usize;

    if a > max {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(a as i8) }
}

/// Parse [`usize`](https://doc.rust-lang.org/std/primitive.usize.html) to [`i8`](https://doc.rust-lang.org/std/primitive.i8.html)
///
///
///
///
/// # Arguments
///
/// * `a` - Any [`usize`](https://doc.rust-lang.org/std/primitive.usize.html) number
///
/// # Examples
///
/// ```
/// rustils::parse::byte::usize_to_i8;
///
///
///
/// ```
pub fn usize_to_i8(a: usize)
    -> i8 {

    match usize_to_i8_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}
// </editor-fold>

// <editor-fold> ## string

/// Parse [`String`](https://doc.rust-lang.org/std/string/struct.String.html) to [`i8`](https://doc.rust-lang.org/std/primitive.i8.html)
///
///
///
///
/// # Arguments
///
/// * `a` - Any [`String`](https://doc.rust-lang.org/std/string/struct.String.html)
///
/// # Examples
///
/// ```
/// rustils::parse::byte::string_to_i8_res;
///
///
///
/// ```
pub fn string_to_i8_res(a: String)
    -> ParseResultI8 {

    match a.parse::<i8>() {
        Ok(n) => Ok(n),
        Err(_) => Err(ParseError::InvalidNumber(a))
    }
}

/// Parse [`String`](https://doc.rust-lang.org/std/string/struct.String.html) to [`i8`](https://doc.rust-lang.org/std/primitive.i8.html)
///
///
///
///
/// # Arguments
///
/// * `a` - Any [`String`](https://doc.rust-lang.org/std/string/struct.String.html)
///
/// # Examples
///
/// ```
/// rustils::parse::byte::string_to_i8;
///
///
///
/// ```
pub fn string_to_i8(a: String)
    -> i8 {

    match string_to_i8_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

/// Parse [`&str`](https://doc.rust-lang.org/std/primitive.str.html) to [`i8`](https://doc.rust-lang.org/std/primitive.i8.html)
///
///
///
///
/// # Arguments
///
/// * `a` - Any [`&str`](https://doc.rust-lang.org/std/primitive.str.html)
///
/// # Examples
///
/// ```
/// rustils::parse::byte::str_to_i8_res;
///
///
///
/// ```
pub fn str_to_i8_res(a: &str)
    -> ParseResultI8 {

    match a.parse::<i8>() {
        Ok(n) => Ok(n),
        Err(_) => Err(ParseError::InvalidNumber(a.to_string()))
    }
}

/// Parse [`&str`](https://doc.rust-lang.org/std/primitive.str.html) to [`i8`](https://doc.rust-lang.org/std/primitive.i8.html)
///
///
///
///
/// # Arguments
///
/// * `a` - Any [`&str`](https://doc.rust-lang.org/std/primitive.str.html)
///
/// # Examples
///
/// ```
/// rustils::parse::byte::str_to_i8;
///
///
///
/// ```
pub fn str_to_i8(a: &str)
    -> i8 {

    match str_to_i8_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}
// </editor-fold>

// </editor-fold>

// <editor-fold> # Types
pub type ParseResultI8 = Result<i8,ParseError>;
// </editor-fold>

// use parse::{
//     ToU8, ToU16, ToU32, ToU64,
//     ToUsize,
//     ToBool
// };
//
// use parse::{
//     ParseResultU8, ParseResultU16, ParseResultU32, ParseResultU64,
//     ParseResultUsize
// };
//
// use error::ParseError;
//

//

//
// impl ToU16 for i8 {
//     fn to_u16_res(self) -> ParseResultU16 {
//         if self < 0 {
//             Err(ParseError::InvalidNumber(self.to_string()))
//         } else { Ok(self as u16) }
//     }
//
//     fn to_u16(self) -> u16 {
//         match self.to_u16_res() {
//             Ok(i) => i,
//             Err(err) => panic!("{}",err)
//         }
//     }
// }
//
// impl ToU32 for i8 {
//     fn to_u32_res(self) -> ParseResultU32 {
//         if self < 0 {
//             Err(ParseError::InvalidNumber(self.to_string()))
//         } else { Ok(self as u32) }
//     }
//
//     fn to_u32(self) -> u32 {
//         match self.to_u32_res() {
//             Ok(i) => i,
//             Err(err) => panic!("{}",err)
//         }
//     }
// }
//
// impl ToU64 for i8 {
//     fn to_u64_res(self) -> ParseResultU64 {
//         if self < 0 {
//             Err(ParseError::InvalidNumber(self.to_string()))
//         } else { Ok(self as u64) }
//     }
//
//     fn to_u64(self) -> u64 {
//         match self.to_u64_res() {
//             Ok(i) => i,
//             Err(err) => panic!("{}",err)
//         }
//     }
// }
//
// impl ToUsize for i8 {
//     fn to_usize_res(self) -> ParseResultUsize {
//         if self < 0 {
//             Err(ParseError::InvalidNumber(self.to_string()))
//         } else { Ok(self as usize) }
//     }
//
//     fn to_usize(self) -> usize {
//         match self.to_usize_res() {
//             Ok(i) => i,
//             Err(err) => panic!("{}",err)
//         }
//     }
// }
