///Converts `true` to `1_i8` and `false` to `0_i8`.
/// # Example
///
/// ```
/// use rustils::parse::boolean;
///
/// let x = true;
/// let y = false;
///
/// assert_eq!(boolean::toi8(x), 1_i8);
/// assert_eq!(boolean::toi8(y), 0_i8);
/// ```
pub fn toi8(b:bool) -> i8 {
    if b { 1 } else { 0 }
}

///Converts `true` to `1_i16` and `false` to `0_i16`.
/// # Example
///
/// ```
/// use rustils::parse::boolean;
///
/// let x = true;
/// let y = false;
///
/// assert_eq!(boolean::toi16(x), 1_i16);
/// assert_eq!(boolean::toi16(y), 0_i16);
/// ```
pub fn toi16(b:bool) -> i16 {
    if b { 1 } else { 0 }
}

///Converts `true` to `1_i32` and `false` to `0_i32`.
/// # Example
///
/// ```
/// use rustils::parse::boolean;
///
/// let x = true;
/// let y = false;
///
/// assert_eq!(boolean::toi32(x), 1_i32);
/// assert_eq!(boolean::toi32(y), 0_i32);
/// ```
pub fn toi32(b:bool) -> i32 {
    if b { 1 } else { 0 }
}

///Converts `true` to `1_i64` and `false` to `0_i64`.
/// # Example
///
/// ```
/// use rustils::parse::boolean;
///
/// let x = true;
/// let y = false;
///
/// assert_eq!(boolean::toi64(x), 1_i64);
/// assert_eq!(boolean::toi64(y), 0_i64);
/// ```
pub fn toi64(b:bool) -> i64 {
    if b { 1 } else { 0 }
}

///Converts `true` to `1_u8` and `false` to `0_u8`.
/// # Example
///
/// ```
/// use rustils::parse::boolean;
///
/// let x = true;
/// let y = false;
///
/// assert_eq!(boolean::tou8(x), 1_u8);
/// assert_eq!(boolean::tou8(y), 0_u8);
/// ```
pub fn tou8(b:bool) -> u8 {
    if b { 1 } else { 0 }
}

///Converts `true` to `1_u16` and `false` to `0_u16`.
/// # Example
///
/// ```
/// use rustils::parse::boolean;
///
/// let x = true;
/// let y = false;
///
/// assert_eq!(boolean::tou16(x), 1_u16);
/// assert_eq!(boolean::tou16(y), 0_u16);
/// ```
pub fn tou16(b:bool) -> u16 {
    if b { 1 } else { 0 }
}

///Converts `true` to `1_u32` and `false` to `0_u32`.
/// # Example
///
/// ```
/// use rustils::parse::boolean;
///
/// let x = true;
/// let y = false;
///
/// assert_eq!(boolean::tou32(x), 1_u32);
/// assert_eq!(boolean::tou32(y), 0_u32);
/// ```
pub fn tou32(b:bool) -> u32 {
    if b { 1 } else { 0 }
}

///Converts `true` to `1_u64` and `false` to `0_u64`.
/// # Example
///
/// ```
/// use rustils::parse::boolean;
///
/// let x = true;
/// let y = false;
///
/// assert_eq!(boolean::tou64(x), 1_u64);
/// assert_eq!(boolean::tou64(y), 0_u64);
/// ```
pub fn tou64(b:bool) -> u64 {
    if b { 1 } else { 0 }
}

///Converts `true` to `1_f32` and `false` to `0_f32`.
/// # Example
///
/// ```
/// use rustils::parse::boolean;
///
/// let x = true;
/// let y = false;
///
/// assert_eq!(boolean::tof32(x), 1_f32);
/// assert_eq!(boolean::tof32(y), 0_f32);
/// ```
pub fn tof32(b:bool) -> f32 {
    if b { 1.0 } else { 0.0 }
}

///Converts `true` to `1_f64` and `false` to `0_f64`.
/// # Example
///
/// ```
/// use rustils::parse::boolean;
///
/// let x = true;
/// let y = false;
///
/// assert_eq!(boolean::tof64(x), 1_f64);
/// assert_eq!(boolean::tof64(y), 0_f64);
/// ```
pub fn tof64(b:bool) -> f64 {
    if b { 1.0 } else { 0.0 }
}

///Converts `true` to `1_usize` and `false` to `0_usize`.
/// # Example
///
/// ```
/// use rustils::parse::boolean;
///
/// let x = true;
/// let y = false;
///
/// assert_eq!(boolean::tousize(x), 1_usize);
/// assert_eq!(boolean::tousize(y), 0_usize);
/// ```
pub fn tousize(b:bool) -> usize {
    if b { 1 } else { 0 }
}

///Converts `true` to `1_isize` and `false` to `0_isize`.
/// # Example
///
/// ```
/// use rustils::parse::boolean;
///
/// let x = true;
/// let y = false;
///
/// assert_eq!(boolean::toisize(x), 1_isize);
/// assert_eq!(boolean::toisize(y), 0_isize);
/// ```
pub fn toisize(b:bool) -> isize {
    if b { 1 } else { 0 }
}
