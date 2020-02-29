use parse::boolean::*;

impl ToBool for i8 {
    /// Parse [`i8`](https://doc.rust-lang.org/std/primitive.i8.html) to
    /// [`bool`](https://doc.rust-lang.org/std/primitive.bool.html)
    /// (see more: [`i8_to_bool`](../../parse/boolean/fn.i8_to_bool.html))
    ///
    /// # Examples
    ///
    /// ```
    /// use rustils::parse::boolean::ToBool;
    ///
    /// assert!(!0_i8.to_bool());
    /// assert!(1_i8.to_bool());
    /// ```
    fn to_bool(self)
        -> bool {

        i8_to_bool(self)
    }
}

impl ToBool for u8 {
    /// Parse [`u8`](https://doc.rust-lang.org/std/primitive.u8.html) to
    /// [`bool`](https://doc.rust-lang.org/std/primitive.bool.html)
    /// (see more: [`u8_to_bool`](../../parse/boolean/fn.u8_to_bool.html))
    ///
    /// # Examples
    ///
    /// ```
    /// use rustils::parse::boolean::ToBool;
    ///
    /// assert!(!0_u8.to_bool());
    /// assert!(1_u8.to_bool());
    /// ```
    fn to_bool(self)
        -> bool {

        u8_to_bool(self)
    }
}

impl ToBool for i16 {
    /// Parse [`i16`](https://doc.rust-lang.org/std/primitive.i16.html) to
    /// [`bool`](https://doc.rust-lang.org/std/primitive.bool.html)
    /// (see more: [`i16_to_bool`](../../parse/boolean/fn.i16_to_bool.html))
    ///
    /// # Examples
    ///
    /// ```
    /// use rustils::parse::boolean::ToBool;
    ///
    /// assert!(!0_i16.to_bool());
    /// assert!(1_i16.to_bool());
    /// ```
    fn to_bool(self)
        -> bool {

        i16_to_bool(self)
    }
}

impl ToBool for u16 {
    /// Parse [`u16`](https://doc.rust-lang.org/std/primitive.u16.html) to
    /// [`bool`](https://doc.rust-lang.org/std/primitive.bool.html)
    /// (see more: [`u16_to_bool`](../../parse/boolean/fn.u16_to_bool.html))
    ///
    /// # Examples
    ///
    /// ```
    /// use rustils::parse::boolean::ToBool;
    ///
    /// assert!(!0_u16.to_bool());
    /// assert!(1_u16.to_bool());
    /// ```
    fn to_bool(self)
        -> bool {

        u16_to_bool(self)
    }
}

impl ToBool for i32 {
    /// Parse [`i32`](https://doc.rust-lang.org/std/primitive.i32.html) to
    /// [`bool`](https://doc.rust-lang.org/std/primitive.bool.html)
    /// (see more: [`i32_to_bool`](../../parse/boolean/fn.i32_to_bool.html))
    ///
    /// # Examples
    ///
    /// ```
    /// use rustils::parse::boolean::ToBool;
    ///
    /// assert!(!0_i32.to_bool());
    /// assert!(1_i32.to_bool());
    /// ```
    fn to_bool(self)
        -> bool {

        i32_to_bool(self)
    }
}

impl ToBool for u32 {
    /// Parse [`u32`](https://doc.rust-lang.org/std/primitive.u32.html) to
    /// [`bool`](https://doc.rust-lang.org/std/primitive.bool.html)
    /// (see more: [`u32_to_bool`](../../parse/boolean/fn.u32_to_bool.html))
    ///
    /// # Examples
    ///
    /// ```
    /// use rustils::parse::boolean::ToBool;
    ///
    /// assert!(!0_u32.to_bool());
    /// assert!(1_u32.to_bool());
    /// ```
    fn to_bool(self)
        -> bool {

        u32_to_bool(self)
    }
}

impl ToBool for f32 {
    /// Parse [`f32`](https://doc.rust-lang.org/std/primitive.f32.html) to
    /// [`bool`](https://doc.rust-lang.org/std/primitive.bool.html)
    /// (see more: [`f32_to_bool`](../../parse/boolean/fn.f32_to_bool.html))
    ///
    /// # Examples
    ///
    /// ```
    /// use rustils::parse::boolean::ToBool;
    ///
    /// assert!(!0.0_f32.to_bool());
    /// assert!(1.0_f32.to_bool());
    /// ```
    fn to_bool(self)
        -> bool {

        f32_to_bool(self)
    }
}

impl ToBool for i64 {
    /// Parse [`i64`](https://doc.rust-lang.org/std/primitive.i64.html) to
    /// [`bool`](https://doc.rust-lang.org/std/primitive.bool.html)
    /// (see more: [`i64_to_bool`](../../parse/boolean/fn.i64_to_bool.html))
    ///
    /// # Examples
    ///
    /// ```
    /// use rustils::parse::boolean::ToBool;
    ///
    /// assert!(!0_i64.to_bool());
    /// assert!(1_i64.to_bool());
    /// ```
    fn to_bool(self)
        -> bool {

        i64_to_bool(self)
    }
}

impl ToBool for u64 {
    /// Parse [`u64`](https://doc.rust-lang.org/std/primitive.u64.html) to
    /// [`bool`](https://doc.rust-lang.org/std/primitive.bool.html)
    /// (see more: [`u64_to_bool`](../../parse/boolean/fn.u64_to_bool.html))
    ///
    /// # Examples
    ///
    /// ```
    /// use rustils::parse::boolean::ToBool;
    ///
    /// assert!(!0_u64.to_bool());
    /// assert!(1_u64.to_bool());
    /// ```
    fn to_bool(self)
        -> bool {

        u64_to_bool(self)
    }
}

impl ToBool for f64 {
    /// Parse [`f64`](https://doc.rust-lang.org/std/primitive.f64.html) to
    /// [`bool`](https://doc.rust-lang.org/std/primitive.bool.html)
    /// (see more: [`f64_to_bool`](../../parse/boolean/fn.f64_to_bool.html))
    ///
    /// # Examples
    ///
    /// ```
    /// use rustils::parse::boolean::ToBool;
    ///
    /// assert!(!0.0_f64.to_bool());
    /// assert!(1.0_f64.to_bool());
    /// ```
    fn to_bool(self)
        -> bool {

        f64_to_bool(self)
    }
}

impl ToBool for isize {
    /// Parse [`isize`](https://doc.rust-lang.org/std/primitive.isize.html) to
    /// [`bool`](https://doc.rust-lang.org/std/primitive.bool.html)
    /// (see more: [`isize_to_bool`](../../parse/boolean/fn.isize_to_bool.html))
    ///
    /// # Examples
    ///
    /// ```
    /// use rustils::parse::boolean::ToBool;
    ///
    /// assert!(!0_isize.to_bool());
    /// assert!(1_isize.to_bool());
    /// ```
    fn to_bool(self)
        -> bool {

        isize_to_bool(self)
    }
}

impl ToBool for usize {
    /// Parse [`usize`](https://doc.rust-lang.org/std/primitive.usize.html) to
    /// [`bool`](https://doc.rust-lang.org/std/primitive.bool.html)
    /// (see more: [`usize_to_bool`](../../parse/boolean/fn.usize_to_bool.html))
    ///
    /// # Examples
    ///
    /// ```
    /// use rustils::parse::boolean::ToBool;
    ///
    /// assert!(!0_usize.to_bool());
    /// assert!(1_usize.to_bool());
    /// ```
    fn to_bool(self)
        -> bool {

        usize_to_bool(self)
    }
}

impl ToBool for String {
    /// Parse [`String`](https://doc.rust-lang.org/std/string/struct.String.html) to
    /// [`bool`](https://doc.rust-lang.org/std/primitive.bool.html)
    /// (see more: [`string_to_bool`](../../parse/boolean/fn.string_to_bool.html))
    ///
    /// # Examples
    ///
    /// ```
    /// use rustils::parse::boolean::ToBool;
    ///
    /// assert!(!String::from("0").to_bool());
    /// assert!(String::from("1").to_bool());
    /// ```
    fn to_bool(self)
        -> bool {

        string_to_bool(self)
    }
}

impl ToBool for &'static str {
    /// Parse [`&str`](https://doc.rust-lang.org/std/primitive.str.html) to
    /// [`bool`](https://doc.rust-lang.org/std/primitive.bool.html)
    /// (see more: [`str_to_bool`](../../parse/boolean/fn.str_to_bool.html))
    ///
    /// # Examples
    ///
    /// ```
    /// use rustils::parse::boolean::ToBool;
    ///
    /// assert!(!"0".to_bool());
    /// assert!("1".to_bool());
    /// ```
    fn to_bool(self)
        -> bool {

        str_to_bool(self)
    }
}
