use error::ParseError;
use RoundingMode;
use RoundingMode::*;

pub trait ToIsize {

    fn to_isize_res(self)
        -> ParseResultIsize;

    fn to_isize(self)
        -> isize;
}

pub trait ToIsizeRM {

    fn to_isize_rm_res(self, rm: RoundingMode)
        -> ParseResultIsize;

    fn to_isize_rm(self, rm: RoundingMode)
        -> isize;
}

/// Parse [`bool`](https://doc.rust-lang.org/std/primitive.bool.html) to
/// [`isize`](https://doc.rust-lang.org/std/primitive.isize.html)
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
/// use rustils::parse::isize::bool_to_isize_res;
///
/// assert_eq!(bool_to_isize_res(true), Ok(1_isize));
/// assert_eq!(bool_to_isize_res(false), Ok(0_isize));
/// ```
pub fn bool_to_isize_res(a: bool)
    -> ParseResultIsize {

    if a { Ok(1) } else { Ok(0) }
}

/// Parse [`bool`](https://doc.rust-lang.org/std/primitive.bool.html) to
/// [`isize`](https://doc.rust-lang.org/std/primitive.isize.html)
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
/// use rustils::parse::isize::bool_to_isize;
///
/// assert_eq!(bool_to_isize(true), 1_isize);
/// assert_eq!(bool_to_isize(false), 0_isize);
/// ```
pub fn bool_to_isize(a: bool)
    -> isize {

    if a { 1 } else { 0 }
}

pub fn f32_to_isize_res(a: f32)
    -> ParseResultIsize {

    f32_to_isize_rm_res(a, Trunc)
}

pub fn f32_to_isize(a: f32)
    -> isize {

    f32_to_isize_rm(a, Trunc)
}

pub fn f32_to_isize_rm_res(a: f32, rm: RoundingMode)
    -> ParseResultIsize {

    let min = -16777215_f32;
    let max = 16777215_f32;

    let x = match rm {
        Round => a.round(),
        Ceil => a.ceil(),
        Floor => a.floor(),
        Trunc => a.trunc()
    };

    if x.is_nan() || x < min || x > max {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(x as isize) }
}

pub fn f32_to_isize_rm(a: f32, rm: RoundingMode)
    -> isize {

    match a.to_isize_rm_res(rm) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub fn u64_to_isize_res(a: u64)
    -> ParseResultIsize {

    let max = isize::max_value() as u64;

    if a > max {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(a as isize) }
}

pub fn u64_to_isize(a: u64)
    -> isize {

    match u64_to_isize_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub fn f64_to_isize_res(a: f64)
    -> ParseResultIsize {

    f64_to_isize_rm_res(a, Trunc)
}

pub fn f64_to_isize(a: f64)
    -> isize {

    f64_to_isize_rm(a, Trunc)
}

pub fn f64_to_isize_rm_res(a: f64, rm: RoundingMode)
    -> ParseResultIsize {

    let min = -9007199254740991_f64;
    let max = 9007199254740991_f64;

    let x = match rm {
        Round => a.round(),
        Ceil => a.ceil(),
        Floor => a.floor(),
        Trunc => a.trunc()
    };

    if x.is_nan() || x < min || x > max {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(x as isize) }
}

pub fn f64_to_isize_rm(a: f64, rm: RoundingMode)
    -> isize {

    match f64_to_isize_rm_res(a, rm) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub fn usize_to_isize_res(a: usize)
    -> ParseResultIsize {

    let max = isize::max_value() as usize;

    if a > max {
        Err(ParseError::InvalidNumber(a.to_string()))
    } else { Ok(a as isize) }
}

pub fn usize_to_isize(a: usize)
    -> isize {

    match usize_to_isize_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub fn string_to_isize_res(a: String)
    -> ParseResultIsize {

    match a.parse::<isize>() {
        Ok(n) => Ok(n),
        Err(_) => Err(ParseError::InvalidNumber(a))
    }
}

pub fn string_to_isize(a: String)
    -> isize {

    match string_to_isize_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub fn str_to_isize_res(a: &str)
    -> ParseResultIsize {

    match a.parse::<isize>() {
        Ok(n) => Ok(n),
        Err(_) => Err(ParseError::InvalidNumber(a.to_string()))
    }
}

pub fn str_to_isize(a: &str)
    -> isize {

    match str_to_isize_res(a) {
        Ok(i) => i,
        Err(err) => panic!("{}",err)
    }
}

pub type ParseResultIsize = Result<isize, ParseError>;
