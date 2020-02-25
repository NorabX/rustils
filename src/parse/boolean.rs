use error::ParseError;

pub trait ToBool {

    fn to_bool(self)
        -> bool;
}

/// Parse `i8` to `bool`
///
/// If `a` is 0 then returns `false`,
/// otherwise `true`.
///
/// # Arguments
///
/// * `a` - Any `i8` number
///
/// # Examples
///
/// ```
/// use rustils::parse::boolean::i8_to_bool;
///
/// let a = 42_i8;
/// let b = -1_i8;
/// let c = 0_i8;
///
/// assert!(i8_to_bool(a));
/// assert!(i8_to_bool(b));
/// assert!(!i8_to_bool(c));
/// ```
pub fn i8_to_bool(a: i8)
    -> bool {

    if a == 0 { false } else { true }
}

/// Parse `u8` to `bool`
///
/// If `a` is 0 then returns `false`,
/// otherwise `true`.
///
/// # Arguments
///
/// * `a` - Any `u8` number
///
/// # Examples
///
/// ```
/// use rustils::parse::boolean::u8_to_bool;
///
/// let a = 42_u8;
/// let b = 1_u8;
/// let c = 0_u8;
///
/// assert!(u8_to_bool(a));
/// assert!(u8_to_bool(b));
/// assert!(!u8_to_bool(c));
/// ```
pub fn u8_to_bool(a: u8)
    -> bool {

    if a == 0 { false } else { true }
}

/// Parse `i16` to `bool`
///
/// If `a` is 0 then returns `false`,
/// otherwise `true`.
///
/// # Arguments
///
/// * `a` - Any `i16` number
///
/// # Examples
///
/// ```
/// use rustils::parse::boolean::i16_to_bool;
///
/// let a = 42_i16;
/// let b = -1_i16;
/// let c = 0_i16;
///
/// assert!(i16_to_bool(a));
/// assert!(i16_to_bool(b));
/// assert!(!i16_to_bool(c));
/// ```
pub fn i16_to_bool(a: i16)
    -> bool {

    if a == 0 { false } else { true }
}

/// Parse `u16` to `bool`
///
/// If `a` is 0 then returns `false`,
/// otherwise `true`.
///
/// # Arguments
///
/// * `a` - Any `u16` number
///
/// # Examples
///
/// ```
/// use rustils::parse::boolean::u16_to_bool;
///
/// let a = 42_u16;
/// let b = 1_u16;
/// let c = 0_u16;
///
/// assert!(u16_to_bool(a));
/// assert!(u16_to_bool(b));
/// assert!(!u16_to_bool(c));
/// ```
pub fn u16_to_bool(a: u16)
    -> bool {

    if a == 0 { false } else { true }
}

/// Parse `i32` to `bool`
///
/// If `a` is 0 then returns `false`,
/// otherwise `true`.
///
/// # Arguments
///
/// * `a` - Any `i32` number
///
/// # Examples
///
/// ```
/// use rustils::parse::boolean::i32_to_bool;
///
/// let a = 42_i32;
/// let b = -1_i32;
/// let c = 0_i32;
///
/// assert!(i32_to_bool(a));
/// assert!(i32_to_bool(b));
/// assert!(!i32_to_bool(c));
/// ```
pub fn i32_to_bool(a: i32)
    -> bool {

    if a == 0 { false } else { true }
}

/// Parse `u32` to `bool`
///
/// If `a` is 0 then returns `false`,
/// otherwise `true`.
///
/// # Arguments
///
/// * `a` - Any `u32` number
///
/// # Examples
///
/// ```
/// use rustils::parse::boolean::u32_to_bool;
///
/// let a = 42_u32;
/// let b = 1_u32;
/// let c = 0_u32;
///
/// assert!(u32_to_bool(a));
/// assert!(u32_to_bool(b));
/// assert!(!u32_to_bool(c));
/// ```
pub fn u32_to_bool(a: u32)
    -> bool {

    if a == 0 { false } else { true }
}

/// Parse `f32` to `bool`
///
/// If `a` is 0 then returns `false`,
/// otherwise `true`.
///
/// # Arguments
///
/// * `a` - Any `f32` number
///
/// # Examples
///
/// ```
/// use rustils::parse::boolean::f32_to_bool;
///
/// let a = 42.0_f32;
/// let b = -1.0_f32;
/// let c = 0.0_f32;
///
/// assert!(f32_to_bool(a));
/// assert!(f32_to_bool(b));
/// assert!(!f32_to_bool(c));
/// ```
pub fn f32_to_bool(a: f32)
    -> bool {

    if a.is_nan() || a == 0.0 { false } else { true }
}

/// Parse `i64` to `bool`
///
/// If `a` is 0 then returns `false`,
/// otherwise `true`.
///
/// # Arguments
///
/// * `a` - Any `i64` number
///
/// # Examples
///
/// ```
/// use rustils::parse::boolean::i64_to_bool;
///
/// let a = 42_i64;
/// let b = -1_i64;
/// let c = 0_i64;
///
/// assert!(i64_to_bool(a));
/// assert!(i64_to_bool(b));
/// assert!(!i64_to_bool(c));
/// ```
pub fn i64_to_bool(a: i64)
    -> bool {

    if a == 0 { false } else { true }
}

/// Parse `u64` to `bool`
///
/// If `a` is 0 then returns `false`,
/// otherwise `true`.
///
/// # Arguments
///
/// * `a` - Any `u64` number
///
/// # Examples
///
/// ```
/// use rustils::parse::boolean::u64_to_bool;
///
/// let a = 42_u64;
/// let b = 1_u64;
/// let c = 0_u64;
///
/// assert!(u64_to_bool(a));
/// assert!(u64_to_bool(b));
/// assert!(!u64_to_bool(c));
/// ```
pub fn u64_to_bool(a: u64)
    -> bool {

    if a == 0 { false } else { true }
}

/// Parse `f64` to `bool`
///
/// If `a` is 0 then returns `false`,
/// otherwise `true`.
///
/// # Arguments
///
/// * `a` - Any `f64` number
///
/// # Examples
///
/// ```
/// use rustils::parse::boolean::f64_to_bool;
///
/// let a = 42.0_f64;
/// let b = -1.0_f64;
/// let c = 0.0_f64;
///
/// assert!(f64_to_bool(a));
/// assert!(f64_to_bool(b));
/// assert!(!f64_to_bool(c));
/// ```
pub fn f64_to_bool(a: f64)
    -> bool {

    if a.is_nan() || a == 0.0 { false } else { true }
}

/// Parse `isize` to `bool`
///
/// If `a` is 0 then returns `false`,
/// otherwise `true`.
///
/// # Arguments
///
/// * `a` - Any `isize` number
///
/// # Examples
///
/// ```
/// use rustils::parse::boolean::isize_to_bool;
///
/// let a = 42_isize;
/// let b = -1_isize;
/// let c = 0_isize;
///
/// assert!(isize_to_bool(a));
/// assert!(isize_to_bool(b));
/// assert!(!isize_to_bool(c));
/// ```
pub fn isize_to_bool(a: isize)
    -> bool {

    if a == 0 { false } else { true }
}

/// Parse `usize` to `bool`
///
/// If `a` is 0 then returns `false`,
/// otherwise `true`.
///
/// # Arguments
///
/// * `a` - Any `usize` number
///
/// # Examples
///
/// ```
/// use rustils::parse::boolean::usize_to_bool;
///
/// let a = 42_usize;
/// let b = 1_usize;
/// let c = 0_usize;
///
/// assert!(usize_to_bool(a));
/// assert!(usize_to_bool(b));
/// assert!(!usize_to_bool(c));
/// ```
pub fn usize_to_bool(a: usize)
    -> bool {

    if a == 0 { false } else { true }
}

/// Parse `String` to `bool`
///
/// If `a` is "0", "f", "false", "n", "no" then returns `false`.<br>
/// If `a` is "1", "t", "true", "y", "yes" then returns `true`.<br>
/// Otherwise panic!
///
/// # Panics
///
/// Panics <a href="../../error/enum.ParseError.html">`ParseError::InvalidString`</a> if `a` is no
/// valid `String`.
///
/// # Arguments
///
/// * `a` - Any `String`
///
/// # Examples
///
/// ```
/// use rustils::parse::boolean::string_to_bool;
///
/// let a = String::from("true");
/// let b = String::from("y");
/// let c = String::from("0");
///
/// assert!(string_to_bool(a));
/// assert!(string_to_bool(b));
/// assert!(!string_to_bool(c));
/// ```
pub fn string_to_bool(a: String)
    -> bool {

    let x = &*a.to_lowercase();

    match x {
        "true" => true,
        "t" => true,
        "false" => false,
        "f" => false,
        "yes" => true,
        "y" => true,
        "no" => false,
        "n" => false,
        "1" => true,
        "0" => false,
        _ => panic!("{}", ParseError::InvalidString(a))
    }
}

/// Parse `&str` to `bool`
///
/// If `a` is "0", "f", "false", "n", "no" then returns `false`.<br>
/// If `a` is "1", "t", "true", "y", "yes" then returns `true`.<br>
/// Otherwise panic!
///
/// # Panics
///
/// Panics <a href="../../error/enum.ParseError.html">`ParseError::InvalidString`</a> if `a` is no
/// valid `&str`.
///
/// # Arguments
///
/// * `a` - Any `&str`
///
/// # Examples
///
/// ```
/// use rustils::parse::boolean::str_to_bool;
///
/// let a = "true";
/// let b = "y";
/// let c = "0";
///
/// assert!(str_to_bool(a));
/// assert!(str_to_bool(b));
/// assert!(!str_to_bool(c));
/// ```
pub fn str_to_bool(a: &str)
    -> bool {

    let x = &*a.to_lowercase();

    match x {
        "true" => true,
        "t" => true,
        "false" => false,
        "f" => false,
        "yes" => true,
        "y" => true,
        "no" => false,
        "n" => false,
        "1" => true,
        "0" => false,
        _ => panic!("{}", ParseError::InvalidString(a.to_string()))
    }
}
