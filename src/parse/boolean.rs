use error::ParseError;

pub trait ToBool {

    fn to_bool(self)
        -> bool;
}

/// Parse i8 to bool
///
/// If `a` is 0 then returns false,
/// otherwise true.
///
/// # Arguments
///
/// * `a` - Any i8 number
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

pub fn u8_to_bool(a: u8)
    -> bool {

    if a == 0 { false } else { true }
}

pub fn i16_to_bool(a: i16)
    -> bool {

    if a == 0 { false } else { true }
}

pub fn u16_to_bool(a: u16)
    -> bool {

    if a == 0 { false } else { true }
}

pub fn i32_to_bool(a: i32)
    -> bool {

    if a == 0 { false } else { true }
}

pub fn u32_to_bool(a: u32)
    -> bool {

    if a == 0 { false } else { true }
}

pub fn f32_to_bool(a: f32)
    -> bool {

    if a.is_nan() || a == 0.0 { false } else { true }
}

pub fn i64_to_bool(a: i64)
    -> bool {

    if a == 0 { false } else { true }
}

pub fn u64_to_bool(a: u64)
    -> bool {

    if a == 0 { false } else { true }
}

pub fn f64_to_bool(a: f64)
    -> bool {

    if a.is_nan() || a == 0.0 { false } else { true }
}

pub fn isize_to_bool(a: isize)
    -> bool {

    if a == 0 { false } else { true }
}

pub fn usize_to_bool(a: usize)
    -> bool {

    if a == 0 { false } else { true }
}

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
