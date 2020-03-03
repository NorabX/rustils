use error::ParseError;
use RoundingMode;
use RoundingMode::*;

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

/// Parse [`bool`](https://doc.rust-lang.org/std/primitive.bool.html) to
/// [`i8`](https://doc.rust-lang.org/std/primitive.i8.html)
///
/// If `a` is `false` then returns `Ok(0)`.<br>
/// If `a` is `true` then returns `Ok(1)`.
///
/// # Arguments
///
/// * `a` - [`bool`](https://doc.rust-lang.org/std/primitive.bool.html)
///
/// # Examples
///
/// ```
/// use rustils::parse::byte::bool_to_i8_res;
///
/// assert_eq!(bool_to_i8_res(true), Ok(1_i8));
/// assert_eq!(bool_to_i8_res(false), Ok(0_i8));
/// ```
pub fn bool_to_i8_res(a: bool)
    -> ParseResultI8 {

    if a { Ok(1) } else { Ok(0) }
}

/// Parse [`bool`](https://doc.rust-lang.org/std/primitive.bool.html) to
/// [`i8`](https://doc.rust-lang.org/std/primitive.i8.html)
///
/// If `a` is `false` then returns 0.<br>
/// If `a` is `true` then returns 1.
///
/// # Arguments
///
/// * `a` - [`bool`](https://doc.rust-lang.org/std/primitive.bool.html)
///
/// # Examples
///
/// ```
/// use rustils::parse::byte::bool_to_i8;
///
/// assert_eq!(bool_to_i8(true), 1_i8);
/// assert_eq!(bool_to_i8(false), 0_i8);
/// ```
pub fn bool_to_i8(a: bool)
    -> i8 {

    if a { 1 } else { 0 }
}

/// Parse [`u8`](https://doc.rust-lang.org/std/primitive.u8.html) to
/// [`i8`](https://doc.rust-lang.org/std/primitive.i8.html)
///
/// # Arguments
///
/// * `a` - Any [`u8`](https://doc.rust-lang.org/std/primitive.u8.html) number
///
/// # Examples
///
/// ```
/// use rustils::parse::byte::u8_to_i8_res;
/// use rustils::error::ParseError::InvalidNumber;
///
/// assert_eq!(u8_to_i8_res(0_u8), Ok(0_i8));
/// assert_eq!(u8_to_i8_res(127_u8), Ok(127_i8));
/// assert_eq!(u8_to_i8_res(128_u8), Err(InvalidNumber(String::from("128"))));
/// ```
pub fn u8_to_i8_res(a: u8)
    -> ParseResultI8 {

    let max = i8::max_value() as u8;

    if a > max {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(a as i8) }
}

/// Parse [`u8`](https://doc.rust-lang.org/std/primitive.u8.html) to
/// [`i8`](https://doc.rust-lang.org/std/primitive.i8.html)
///
/// # Panics
///
/// ```rust,should_panic
/// rustils::parse::byte::u8_to_i8(128_u8);
/// ```
///
/// # Arguments
///
/// * `a` - Any [`u8`](https://doc.rust-lang.org/std/primitive.u8.html) number
///
/// # Examples
///
/// ```
/// use rustils::parse::byte::u8_to_i8;
///
/// assert_eq!(u8_to_i8(0_u8), 0_i8);
/// assert_eq!(u8_to_i8(127_u8), 127_i8);
/// ```
pub fn u8_to_i8(a: u8)
    -> i8 {

    match u8_to_i8_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}", err)
    }
}

/// Parse [`i16`](https://doc.rust-lang.org/std/primitive.i16.html) to
/// [`i8`](https://doc.rust-lang.org/std/primitive.i8.html)
///
/// # Arguments
///
/// * `a` - Any [`i16`](https://doc.rust-lang.org/std/primitive.i16.html) number
///
/// # Examples
///
/// ```
/// use rustils::parse::byte::i16_to_i8_res;
/// use rustils::error::ParseError::InvalidNumber;
///
/// assert_eq!(i16_to_i8_res(-128_i16), Ok(-128_i8));
/// assert_eq!(i16_to_i8_res(127_i16), Ok(127_i8));
/// assert_eq!(i16_to_i8_res(-129_i16), Err(InvalidNumber(String::from("-129"))));
/// assert_eq!(i16_to_i8_res(128_i16), Err(InvalidNumber(String::from("128"))));
/// ```
pub fn i16_to_i8_res(a: i16)
    -> ParseResultI8 {

    let min = i8::min_value() as i16;
    let max = i8::max_value() as i16;

    if a < min || a > max {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(a as i8) }
}

/// Parse [`i16`](https://doc.rust-lang.org/std/primitive.i16.html) to
/// [`i8`](https://doc.rust-lang.org/std/primitive.i8.html)
///
/// # Panics
///
/// ```rust,should_panic
/// rustils::parse::byte::i16_to_i8(-129_i16);
/// rustils::parse::byte::i16_to_i8(128_i16);
/// ```
///
/// # Arguments
///
/// * `a` - Any [`i16`](https://doc.rust-lang.org/std/primitive.i16.html) number
///
/// # Examples
///
/// ```
/// use rustils::parse::byte::i16_to_i8;
///
/// assert_eq!(i16_to_i8(-128_i16), -128_i8);
/// assert_eq!(i16_to_i8(127_i16), 127_i8);
/// ```
pub fn i16_to_i8(a: i16)
    -> i8 {

    match i16_to_i8_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

/// Parse [`u16`](https://doc.rust-lang.org/std/primitive.u16.html) to
/// [`i8`](https://doc.rust-lang.org/std/primitive.i8.html)
///
/// # Arguments
///
/// * `a` - Any [`u16`](https://doc.rust-lang.org/std/primitive.u16.html) number
///
/// # Examples
///
/// ```
/// use rustils::parse::byte::u16_to_i8_res;
/// use rustils::error::ParseError::InvalidNumber;
///
/// assert_eq!(u16_to_i8_res(0_u16), Ok(0_i8));
/// assert_eq!(u16_to_i8_res(127_u16), Ok(127_i8));
/// assert_eq!(u16_to_i8_res(128_u16), Err(InvalidNumber(String::from("128"))));
/// ```
pub fn u16_to_i8_res(a: u16)
    -> ParseResultI8 {

    let max = i8::max_value() as u16;

    if a > max {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(a as i8) }
}

/// Parse [`u16`](https://doc.rust-lang.org/std/primitive.u16.html) to
/// [`i8`](https://doc.rust-lang.org/std/primitive.i8.html)
///
/// # Panics
///
/// ```rust,should_panic
/// rustils::parse::byte::u16_to_i8(128_u16);
/// ```
///
/// # Arguments
///
/// * `a` - Any [`u16`](https://doc.rust-lang.org/std/primitive.u16.html) number
///
/// # Examples
///
/// ```
/// use rustils::parse::byte::u16_to_i8;
///
/// assert_eq!(u16_to_i8(0_u16), 0_i8);
/// assert_eq!(u16_to_i8(127_u16), 127_i8);
/// ```
pub fn u16_to_i8(a: u16)
    -> i8 {

    match u16_to_i8_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

/// Parse [`i32`](https://doc.rust-lang.org/std/primitive.i32.html) to
/// [`i8`](https://doc.rust-lang.org/std/primitive.i8.html)
///
/// # Arguments
///
/// * `a` - Any [`i32`](https://doc.rust-lang.org/std/primitive.i32.html) number
///
/// # Examples
///
/// ```
/// use rustils::parse::byte::i32_to_i8_res;
/// use rustils::error::ParseError::InvalidNumber;
///
/// assert_eq!(i32_to_i8_res(-128_i32), Ok(-128_i8));
/// assert_eq!(i32_to_i8_res(127_i32), Ok(127_i8));
/// assert_eq!(i32_to_i8_res(-129_i32), Err(InvalidNumber(String::from("-129"))));
/// assert_eq!(i32_to_i8_res(128_i32), Err(InvalidNumber(String::from("128"))));
/// ```
pub fn i32_to_i8_res(a: i32)
    -> ParseResultI8 {

    let min = i8::min_value() as i32;
    let max = i8::max_value() as i32;

    if a < min || a > max {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(a as i8) }
}

/// Parse [`i32`](https://doc.rust-lang.org/std/primitive.i32.html) to
/// [`i8`](https://doc.rust-lang.org/std/primitive.i8.html)
///
/// # Panics
///
/// ```rust,should_panic
/// rustils::parse::byte::i32_to_i8(-129_i32);
/// rustils::parse::byte::i32_to_i8(128_i32);
/// ```
///
/// # Arguments
///
/// * `a` - Any [`i32`](https://doc.rust-lang.org/std/primitive.i32.html) number
///
/// # Examples
///
/// ```
/// use rustils::parse::byte::i32_to_i8;
///
/// assert_eq!(i32_to_i8(-128_i32), -128_i8);
/// assert_eq!(i32_to_i8(127_i32), 127_i8);
/// ```
pub fn i32_to_i8(a: i32)
    -> i8 {

    match i32_to_i8_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

/// Parse [`u32`](https://doc.rust-lang.org/std/primitive.u32.html) to
/// [`i8`](https://doc.rust-lang.org/std/primitive.i8.html)
///
/// # Arguments
///
/// * `a` - Any [`u32`](https://doc.rust-lang.org/std/primitive.u32.html) number
///
/// # Examples
///
/// ```
/// use rustils::parse::byte::u32_to_i8_res;
/// use rustils::error::ParseError::InvalidNumber;
///
/// assert_eq!(u32_to_i8_res(0_u32), Ok(0_i8));
/// assert_eq!(u32_to_i8_res(127_u32), Ok(127_i8));
/// assert_eq!(u32_to_i8_res(128_u32), Err(InvalidNumber(String::from("128"))));
/// ```
pub fn u32_to_i8_res(a: u32)
    -> ParseResultI8 {

    let max = i8::max_value() as u32;

    if a > max {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(a as i8) }
}

/// Parse [`u32`](https://doc.rust-lang.org/std/primitive.u32.html) to
/// [`i8`](https://doc.rust-lang.org/std/primitive.i8.html)
///
/// # Panics
///
/// ```rust,should_panic
/// rustils::parse::byte::u32_to_i8(128_u32);
/// ```
///
/// # Arguments
///
/// * `a` - Any [`u32`](https://doc.rust-lang.org/std/primitive.u32.html) number
///
/// # Examples
///
/// ```
/// use rustils::parse::byte::u32_to_i8;
///
/// assert_eq!(u32_to_i8(0_u32), 0_i8);
/// assert_eq!(u32_to_i8(127_u32), 127_i8);
/// ```
pub fn u32_to_i8(a: u32)
    -> i8 {

    match u32_to_i8_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

/// Parse [`f32`](https://doc.rust-lang.org/std/primitive.f32.html) to
/// [`i8`](https://doc.rust-lang.org/std/primitive.i8.html)
///
/// # Arguments
///
/// * `a` - Any [`f32`](https://doc.rust-lang.org/std/primitive.f32.html) number
///
/// # Examples
///
/// ```
/// use rustils::parse::byte::f32_to_i8_res;
/// use rustils::error::ParseError::InvalidNumber;
///
/// assert_eq!(f32_to_i8_res(-128.0_f32), Ok(-128_i8));
/// assert_eq!(f32_to_i8_res(127.0_f32), Ok(127_i8));
/// assert_eq!(f32_to_i8_res(127.5_f32), Ok(127_i8));
///
/// assert_eq!(f32_to_i8_res(128.0_f32), Err(InvalidNumber(String::from("128"))));
/// assert_eq!(f32_to_i8_res(-129.0_f32), Err(InvalidNumber(String::from("-129"))));
/// assert_eq!(f32_to_i8_res(-129.5_f32), Err(InvalidNumber(String::from("-129.5"))));
/// ```
pub fn f32_to_i8_res(a: f32)
    -> ParseResultI8{

    f32_to_i8_rm_res(a, Trunc)
}

/// Parse [`f32`](https://doc.rust-lang.org/std/primitive.f32.html) to
/// [`i8`](https://doc.rust-lang.org/std/primitive.i8.html)
///
/// # Panics
///
/// ```rust,should_panic
/// rustils::parse::byte::f32_to_i8(128.0_f32);
/// rustils::parse::byte::f32_to_i8(-129.0_f32);
/// rustils::parse::byte::f32_to_i8(-129.5_f32);
/// ```
///
/// # Arguments
///
/// * `a` - Any [`f32`](https://doc.rust-lang.org/std/primitive.f32.html) number
///
/// # Examples
///
/// ```
/// use rustils::parse::byte::f32_to_i8;
///
/// assert_eq!(f32_to_i8(-128.0_f32), -128_i8);
/// assert_eq!(f32_to_i8(127.0_f32), 127_i8);
/// assert_eq!(f32_to_i8(127.5_f32), 127_i8);
/// ```
pub fn f32_to_i8(a: f32)
    -> i8 {

    f32_to_i8_rm(a, Trunc)
}

/// Parse [`f32`](https://doc.rust-lang.org/std/primitive.f32.html) to
/// [`i8`](https://doc.rust-lang.org/std/primitive.i8.html)
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

/// Parse [`f32`](https://doc.rust-lang.org/std/primitive.f32.html) to
/// [`i8`](https://doc.rust-lang.org/std/primitive.i8.html)
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

/// Parse [`i64`](https://doc.rust-lang.org/std/primitive.i64.html) to
/// [`i8`](https://doc.rust-lang.org/std/primitive.i8.html)
///
/// # Arguments
///
/// * `a` - Any [`i64`](https://doc.rust-lang.org/std/primitive.i64.html) number
///
/// # Examples
///
/// ```
/// use rustils::parse::byte::i64_to_i8_res;
/// use rustils::error::ParseError::InvalidNumber;
///
/// assert_eq!(i64_to_i8_res(-128_i64), Ok(-128_i8));
/// assert_eq!(i64_to_i8_res(127_i64), Ok(127_i8));
/// assert_eq!(i64_to_i8_res(-129_i64), Err(InvalidNumber(String::from("-129"))));
/// assert_eq!(i64_to_i8_res(128_i64), Err(InvalidNumber(String::from("128"))));
/// ```
pub fn i64_to_i8_res(a: i64)
    -> ParseResultI8 {

    let min = i8::min_value() as i64;
    let max = i8::max_value() as i64;

    if a < min || a > max {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(a as i8) }
}

/// Parse [`i64`](https://doc.rust-lang.org/std/primitive.i64.html) to
/// [`i8`](https://doc.rust-lang.org/std/primitive.i8.html)
///
/// # Panics
///
/// ```rust,should_panic
/// rustils::parse::byte::i64_to_i8(-129_i64);
/// rustils::parse::byte::i64_to_i8(128_i64);
/// ```
///
/// # Arguments
///
/// * `a` - Any [`i64`](https://doc.rust-lang.org/std/primitive.i64.html) number
///
/// # Examples
///
/// ```
/// use rustils::parse::byte::i64_to_i8;
///
/// assert_eq!(i64_to_i8(-128_i64), -128_i8);
/// assert_eq!(i64_to_i8(127_i64), 127_i8);
/// ```
pub fn i64_to_i8(a: i64)
    -> i8 {

    match i64_to_i8_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

/// Parse [`u64`](https://doc.rust-lang.org/std/primitive.u64.html) to
/// [`i8`](https://doc.rust-lang.org/std/primitive.i8.html)
///
/// # Arguments
///
/// * `a` - Any [`u64`](https://doc.rust-lang.org/std/primitive.u64.html) number
///
/// # Examples
///
/// ```
/// use rustils::parse::byte::u64_to_i8_res;
/// use rustils::error::ParseError::InvalidNumber;
///
/// assert_eq!(u64_to_i8_res(0_u64), Ok(0_i8));
/// assert_eq!(u64_to_i8_res(127_u64), Ok(127_i8));
/// assert_eq!(u64_to_i8_res(128_u64), Err(InvalidNumber(String::from("128"))));
/// ```
pub fn u64_to_i8_res(a: u64)
    -> ParseResultI8 {

    let max = i8::max_value() as u64;

    if a > max {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(a as i8) }
}

/// Parse [`u64`](https://doc.rust-lang.org/std/primitive.u64.html) to
/// [`i8`](https://doc.rust-lang.org/std/primitive.i8.html)
///
/// # Panics
///
/// ```rust,should_panic
/// rustils::parse::byte::u64_to_i8(128_u64);
/// ```
///
/// # Arguments
///
/// * `a` - Any [`u64`](https://doc.rust-lang.org/std/primitive.u64.html) number
///
/// # Examples
///
/// ```
/// use rustils::parse::byte::u64_to_i8;
///
/// assert_eq!(u64_to_i8(0_u64), 0_i8);
/// assert_eq!(u64_to_i8(127_u64), 127_i8);
/// ```
pub fn u64_to_i8(a: u64)
    -> i8 {

    match u64_to_i8_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

/// Parse [`f64`](https://doc.rust-lang.org/std/primitive.f64.html) to
/// [`i8`](https://doc.rust-lang.org/std/primitive.i8.html)
///
/// # Arguments
///
/// * `a` - Any [`f64`](https://doc.rust-lang.org/std/primitive.f64.html) number
///
/// # Examples
///
/// ```
/// use rustils::parse::byte::f64_to_i8_res;
/// use rustils::error::ParseError::InvalidNumber;
///
/// assert_eq!(f64_to_i8_res(-128.0_f64), Ok(-128_i8));
/// assert_eq!(f64_to_i8_res(127.0_f64), Ok(127_i8));
/// assert_eq!(f64_to_i8_res(127.5_f64), Ok(127_i8));
///
/// assert_eq!(f64_to_i8_res(128.0_f64), Err(InvalidNumber(String::from("128"))));
/// assert_eq!(f64_to_i8_res(-129.0_f64), Err(InvalidNumber(String::from("-129"))));
/// assert_eq!(f64_to_i8_res(-129.5_f64), Err(InvalidNumber(String::from("-129.5"))));
/// ```
pub fn f64_to_i8_res(a: f64)
    -> ParseResultI8 {

    f64_to_i8_rm_res(a, Trunc)
}

/// Parse [`f64`](https://doc.rust-lang.org/std/primitive.f64.html) to
/// [`i8`](https://doc.rust-lang.org/std/primitive.i8.html)
///
/// # Panics
///
/// ```rust,should_panic
/// rustils::parse::byte::f64_to_i8(128.0_f64);
/// rustils::parse::byte::f64_to_i8(-129.0_f64);
/// rustils::parse::byte::f64_to_i8(-129.5_f64);
/// ```
///
/// # Arguments
///
/// * `a` - Any [`f64`](https://doc.rust-lang.org/std/primitive.f64.html) number
///
/// # Examples
///
/// ```
/// use rustils::parse::byte::f64_to_i8;
///
/// assert_eq!(f64_to_i8(-128.0_f64), -128_i8);
/// assert_eq!(f64_to_i8(127.0_f64), 127_i8);
/// assert_eq!(f64_to_i8(127.5_f64), 127_i8);
/// ```
pub fn f64_to_i8(a: f64)
    -> i8 {

    f64_to_i8_rm(a, Trunc)
}

/// Parse [`f64`](https://doc.rust-lang.org/std/primitive.f64.html) to
/// [`i8`](https://doc.rust-lang.org/std/primitive.i8.html)
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

/// Parse [`f64`](https://doc.rust-lang.org/std/primitive.f64.html) to
/// [`i8`](https://doc.rust-lang.org/std/primitive.i8.html)
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

/// Parse [`isize`](https://doc.rust-lang.org/std/primitive.isize.html) to
/// [`i8`](https://doc.rust-lang.org/std/primitive.i8.html)
///
/// # Arguments
///
/// * `a` - Any [`isize`](https://doc.rust-lang.org/std/primitive.isize.html) number
///
/// # Examples
///
/// ```
/// use rustils::parse::byte::isize_to_i8_res;
/// use rustils::error::ParseError::InvalidNumber;
///
/// assert_eq!(isize_to_i8_res(-128_isize), Ok(-128_i8));
/// assert_eq!(isize_to_i8_res(127_isize), Ok(127_i8));
/// assert_eq!(isize_to_i8_res(-129_isize), Err(InvalidNumber(String::from("-129"))));
/// assert_eq!(isize_to_i8_res(128_isize), Err(InvalidNumber(String::from("128"))));
/// ```
pub fn isize_to_i8_res(a: isize)
    -> ParseResultI8 {

    let min = i8::min_value() as isize;
    let max = i8::max_value() as isize;

    if a < min || a > max {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(a as i8) }
}

/// Parse [`isize`](https://doc.rust-lang.org/std/primitive.isize.html) to
/// [`i8`](https://doc.rust-lang.org/std/primitive.i8.html)
///
/// # Panics
///
/// ```rust,should_panic
/// rustils::parse::byte::isize_to_i8(-129_isize);
/// rustils::parse::byte::isize_to_i8(128_isize);
/// ```
///
/// # Arguments
///
/// * `a` - Any [`isize`](https://doc.rust-lang.org/std/primitive.isize.html) number
///
/// # Examples
///
/// ```
/// use rustils::parse::byte::isize_to_i8;
///
/// assert_eq!(isize_to_i8(-128_isize), -128_i8);
/// assert_eq!(isize_to_i8(127_isize), 127_i8);
/// ```
pub fn isize_to_i8(a: isize)
    -> i8 {

    match isize_to_i8_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

/// Parse [`usize`](https://doc.rust-lang.org/std/primitive.usize.html) to
/// [`i8`](https://doc.rust-lang.org/std/primitive.i8.html)
///
/// # Arguments
///
/// * `a` - Any [`usize`](https://doc.rust-lang.org/std/primitive.usize.html) number
///
/// # Examples
///
/// ```
/// use rustils::parse::byte::usize_to_i8_res;
/// use rustils::error::ParseError::InvalidNumber;
///
/// assert_eq!(usize_to_i8_res(0_usize), Ok(0_i8));
/// assert_eq!(usize_to_i8_res(127_usize), Ok(127_i8));
/// assert_eq!(usize_to_i8_res(128_usize), Err(InvalidNumber(String::from("128"))));
///
/// ```
pub fn usize_to_i8_res(a: usize)
    -> ParseResultI8 {

    let max = i8::max_value() as usize;

    if a > max {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(a as i8) }
}

/// Parse [`usize`](https://doc.rust-lang.org/std/primitive.usize.html) to
/// [`i8`](https://doc.rust-lang.org/std/primitive.i8.html)
///
/// # Panics
///
/// ```rust,should_panic
/// rustils::parse::byte::usize_to_i8(128_usize);
/// ```
///
/// # Arguments
///
/// * `a` - Any [`usize`](https://doc.rust-lang.org/std/primitive.usize.html) number
///
/// # Examples
///
/// ```
/// use rustils::parse::byte::usize_to_i8;
///
/// assert_eq!(usize_to_i8(0_usize), 0_i8);
/// assert_eq!(usize_to_i8(127_usize), 127_i8);
/// ```
pub fn usize_to_i8(a: usize)
    -> i8 {

    match usize_to_i8_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

/// Parse [`String`](https://doc.rust-lang.org/std/string/struct.String.html) to
/// [`i8`](https://doc.rust-lang.org/std/primitive.i8.html)
///
/// # Arguments
///
/// * `a` - Any [`String`](https://doc.rust-lang.org/std/string/struct.String.html)
///
/// # Examples
///
/// ```
/// use rustils::parse::byte::string_to_i8_res;
/// use rustils::error::ParseError::InvalidNumber;
///
/// assert_eq!(string_to_i8_res("-128".to_string()), Ok(-128_i8));
/// assert_eq!(string_to_i8_res("127".to_string()), Ok(127_i8));
/// assert_eq!(string_to_i8_res("-129".to_string()), Err(InvalidNumber(String::from("-129"))));
/// assert_eq!(string_to_i8_res("128".to_string()), Err(InvalidNumber(String::from("128"))));
/// ```
pub fn string_to_i8_res(a: String)
    -> ParseResultI8 {

    match a.parse::<i8>() {
        Ok(n) => Ok(n),
        Err(_) => Err(ParseError::InvalidNumber(a))
    }
}

/// Parse [`String`](https://doc.rust-lang.org/std/string/struct.String.html) to
/// [`i8`](https://doc.rust-lang.org/std/primitive.i8.html)
///
/// # Panics
///
/// ```rust,should_panic
/// rustils::parse::byte::string_to_i8("-129".to_string());
/// rustils::parse::byte::string_to_i8("128".to_string());
/// ```
///
/// # Arguments
///
/// * `a` - Any [`String`](https://doc.rust-lang.org/std/string/struct.String.html)
///
/// # Examples
///
/// ```
/// use rustils::parse::byte::string_to_i8;
///
/// assert_eq!(string_to_i8("-128".to_string()), -128_i8);
/// assert_eq!(string_to_i8("127".to_string()), 127_i8);
/// ```
pub fn string_to_i8(a: String)
    -> i8 {

    match string_to_i8_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

/// Parse [`&str`](https://doc.rust-lang.org/std/primitive.str.html) to
/// [`i8`](https://doc.rust-lang.org/std/primitive.i8.html)
///
/// # Arguments
///
/// * `a` - Any [`&str`](https://doc.rust-lang.org/std/primitive.str.html)
///
/// # Examples
///
/// ```
/// use rustils::parse::byte::str_to_i8_res;
/// use rustils::error::ParseError::InvalidNumber;
///
/// assert_eq!(str_to_i8_res("-128"), Ok(-128_i8));
/// assert_eq!(str_to_i8_res("127"), Ok(127_i8));
/// assert_eq!(str_to_i8_res("-129"), Err(InvalidNumber(String::from("-129"))));
/// assert_eq!(str_to_i8_res("128"), Err(InvalidNumber(String::from("128"))));
/// ```
pub fn str_to_i8_res(a: &str)
    -> ParseResultI8 {

    match a.parse::<i8>() {
        Ok(n) => Ok(n),
        Err(_) => Err(ParseError::InvalidNumber(a.to_string()))
    }
}

/// Parse [`&str`](https://doc.rust-lang.org/std/primitive.str.html) to
/// [`i8`](https://doc.rust-lang.org/std/primitive.i8.html)
///
/// # Panics
///
/// ```rust,should_panic
/// rustils::parse::byte::str_to_i8("-129");
/// rustils::parse::byte::str_to_i8("128");
/// ```
///
/// # Arguments
///
/// * `a` - Any [`&str`](https://doc.rust-lang.org/std/primitive.str.html)
///
/// # Examples
///
/// ```
/// use rustils::parse::byte::str_to_i8;
///
/// assert_eq!(str_to_i8("-128"), -128_i8);
/// assert_eq!(str_to_i8("127"), 127_i8);
/// ```
pub fn str_to_i8(a: &str)
    -> i8 {

    match str_to_i8_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub type ParseResultI8 = Result<i8, ParseError>;
