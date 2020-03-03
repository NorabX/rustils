use parse::int::*;
use RoundingMode;

impl ToI32 for bool {

    /// Parse [`bool`](https://doc.rust-lang.org/std/primitive.bool.html) to
    /// [`i32`](https://doc.rust-lang.org/std/primitive.i32.html)
    /// (see more: [`bool_to_i32_res`](../../parse/int/fn.bool_to_i32_res.html))
    ///
    /// # Examples
    ///
    /// ```
    /// use rustils::parse::int::ToI32;
    ///
    /// assert_eq!(true.to_i32_res(), Ok(1_i32));
    /// assert_eq!(false.to_i32_res(), Ok(0_i32));
    /// ```
    fn to_i32_res(self)
        -> ParseResultI32 {

        bool_to_i32_res(self)
    }

    /// Parse [`bool`](https://doc.rust-lang.org/std/primitive.bool.html) to
    /// [`i32`](https://doc.rust-lang.org/std/primitive.i32.html)
    /// (see more: [`bool_to_i32`](../../parse/int/fn.bool_to_i32.html))
    ///
    /// # Examples
    ///
    /// ```
    /// use rustils::parse::int::ToI32;
    ///
    /// assert_eq!(true.to_i32(), 1_i32);
    /// assert_eq!(false.to_i32(), 0_i32);
    /// ```
    fn to_i32(self)
        -> i32 {

        bool_to_i32(self)
    }
}

impl ToI32 for u32 {

    /// Parse [`u32`](https://doc.rust-lang.org/std/primitive.u32.html) to
    /// [`i32`](https://doc.rust-lang.org/std/primitive.i32.html)
    /// (see more: [`u32_to_i32_res`](../../parse/int/fn.u32_to_i32_res.html))
    ///
    /// # Examples
    ///
    /// ```
    /// use rustils::parse::int::ToI32;
    ///
    /// assert_eq!(0_u32.to_i32_res(), Ok(0_i32));
    /// assert_eq!(2147483647_u32.to_i32_res(), Ok(2147483647_i32));
    /// ```
    fn to_i32_res(self)
        -> ParseResultI32 {

        u32_to_i32_res(self)
    }

    /// Parse [`u32`](https://doc.rust-lang.org/std/primitive.u32.html) to
    /// [`i32`](https://doc.rust-lang.org/std/primitive.i32.html)
    /// (see more: [`u32_to_i32`](../../parse/int/fn.u32_to_i32.html))
    ///
    /// # Examples
    ///
    /// ```
    /// use rustils::parse::int::ToI32;
    ///
    /// assert_eq!(0_u32.to_i32(), 0_i32);
    /// assert_eq!(2147483647_u32.to_i32(), 2147483647_i32);
    /// ```
    fn to_i32(self)
        -> i32 {

        u32_to_i32(self)
    }
}

impl ToI32 for f32 {

    fn to_i32_res(self)
        -> ParseResultI32 {

        f32_to_i32_res(self)
    }

    fn to_i32(self)
        -> i32 {

        f32_to_i32(self)
    }
}

impl ToI32RM for f32 {

    fn to_i32_rm_res(self, rm: RoundingMode)
        -> ParseResultI32 {

        f32_to_i32_rm_res(self, rm)
    }

    fn to_i32_rm(self, rm: RoundingMode)
        -> i32 {

        f32_to_i32_rm(self, rm)
    }
}

impl ToI32 for i64 {

    fn to_i32_res(self)
        -> ParseResultI32 {

        i64_to_i32_res(self)
    }

    fn to_i32(self)
        -> i32 {

        i64_to_i32(self)
    }
}

impl ToI32 for u64 {

    /// Parse [`u64`](https://doc.rust-lang.org/std/primitive.u64.html) to
    /// [`i32`](https://doc.rust-lang.org/std/primitive.i32.html)
    /// (see more: [`u64_to_i32_res`](../../parse/int/fn.u64_to_i32_res.html))
    ///
    /// # Examples
    ///
    /// ```
    /// use rustils::parse::int::ToI32;
    ///
    /// assert_eq!(0_u64.to_i32_res(), Ok(0_i32));
    /// assert_eq!(2147483647_u64.to_i32_res(), Ok(2147483647_i32));
    /// ```
    fn to_i32_res(self)
        -> ParseResultI32 {

        u64_to_i32_res(self)
    }

    /// Parse [`u64`](https://doc.rust-lang.org/std/primitive.u64.html) to
    /// [`i32`](https://doc.rust-lang.org/std/primitive.i32.html)
    /// (see more: [`u64_to_i32`](../../parse/int/fn.u64_to_i32.html))
    ///
    /// # Examples
    ///
    /// ```
    /// use rustils::parse::int::ToI32;
    ///
    /// assert_eq!(0_u64.to_i32(), 0_i32);
    /// assert_eq!(2147483647_u64.to_i32(), 2147483647_i32);
    /// ```
    fn to_i32(self)
        -> i32 {

        u64_to_i32(self)
    }
}

impl ToI32 for f64 {

    fn to_i32_res(self)
        -> ParseResultI32 {

        f64_to_i32_res(self)
    }

    fn to_i32(self)
        -> i32 {

        f64_to_i32(self)
    }
}

impl ToI32RM for f64 {

    fn to_i32_rm_res(self, rm: RoundingMode)
        -> ParseResultI32 {

        f64_to_i32_rm_res(self, rm)
    }

    fn to_i32_rm(self, rm: RoundingMode)
        -> i32 {

        f64_to_i32_rm(self, rm)
    }
}

impl ToI32 for isize {

    fn to_i32_res(self)
        -> ParseResultI32 {

        isize_to_i32_res(self)
    }

    fn to_i32(self)
        -> i32 {

        isize_to_i32(self)
    }
}

impl ToI32 for usize {

    /// Parse [`usize`](https://doc.rust-lang.org/std/primitive.usize.html) to
    /// [`i32`](https://doc.rust-lang.org/std/primitive.i32.html)
    /// (see more: [`usize_to_i32_res`](../../parse/int/fn.usize_to_i32_res.html))
    ///
    /// # Examples
    ///
    /// ```
    /// use rustils::parse::int::ToI32;
    ///
    /// assert_eq!(0_usize.to_i32_res(), Ok(0_i32));
    /// assert_eq!(2147483647_usize.to_i32_res(), Ok(2147483647_i32));
    /// ```
    fn to_i32_res(self)
        -> ParseResultI32 {

        usize_to_i32_res(self)
    }

    /// Parse [`usize`](https://doc.rust-lang.org/std/primitive.usize.html) to
    /// [`i32`](https://doc.rust-lang.org/std/primitive.i32.html)
    /// (see more: [`usize_to_i32`](../../parse/int/fn.usize_to_i32.html))
    ///
    /// # Examples
    ///
    /// ```
    /// use rustils::parse::int::ToI32;
    ///
    /// assert_eq!(0_usize.to_i32(), 0_i32);
    /// assert_eq!(2147483647_usize.to_i32(), 2147483647_i32);
    /// ```
    fn to_i32(self)
        -> i32 {

        usize_to_i32(self)
    }
}

impl ToI32 for String {

    fn to_i32_res(self)
        -> ParseResultI32 {

        string_to_i32_res(self)
    }

    fn to_i32(self)
        -> i32 {

        string_to_i32(self)
    }
}

impl ToI32 for &'static str {

    fn to_i32_res(self)
        -> ParseResultI32 {

        str_to_i32_res(self)
    }

    fn to_i32(self)
        -> i32 {

        str_to_i32(self)
    }
}
