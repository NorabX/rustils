use parse::short::*;
use RoundingMode;

impl ToI16 for bool {

    /// Parse [`bool`](https://doc.rust-lang.org/std/primitive.bool.html) to
    /// [`i16`](https://doc.rust-lang.org/std/primitive.i16.html)
    /// (see more: [`bool_to_i16_res`](../../parse/short/fn.bool_to_i16_res.html))
    ///
    /// # Examples
    ///
    /// ```
    /// use rustils::parse::short::ToI16;
    ///
    /// assert_eq!(true.to_i16_res(), Ok(1_i16));
    /// assert_eq!(false.to_i16_res(), Ok(0_i16));
    /// ```
    fn to_i16_res(self)
        -> ParseResultI16 {

        bool_to_i16_res(self)
    }

    /// Parse [`bool`](https://doc.rust-lang.org/std/primitive.bool.html) to
    /// [`i16`](https://doc.rust-lang.org/std/primitive.i16.html)
    /// (see more: [`bool_to_i16`](../../parse/short/fn.bool_to_i16.html))
    ///
    /// # Examples
    ///
    /// ```
    /// use rustils::parse::short::ToI16;
    ///
    /// assert_eq!(true.to_i16(), 1_i16);
    /// assert_eq!(false.to_i16(), 0_i16);
    /// ```
    fn to_i16(self)
        -> i16 {

        bool_to_i16(self)
    }
}

impl ToI16 for u16 {

    /// Parse [`u16`](https://doc.rust-lang.org/std/primitive.u16.html) to
    /// [`i16`](https://doc.rust-lang.org/std/primitive.i16.html)
    /// (see more: [`u16_to_i16_res`](../../parse/short/fn.u16_to_i16_res.html))
    ///
    /// # Examples
    ///
    /// ```
    /// use rustils::parse::short::ToI16;
    ///
    /// assert_eq!(0_u16.to_i16_res(), Ok(0_i16));
    /// assert_eq!(32767_u16.to_i16_res(), Ok(32767_i16));
    /// ```
    fn to_i16_res(self)
        -> ParseResultI16 {

        u16_to_i16_res(self)
    }

    /// Parse [`u16`](https://doc.rust-lang.org/std/primitive.u16.html) to
    /// [`i16`](https://doc.rust-lang.org/std/primitive.i16.html)
    /// (see more: [`u16_to_i16`](../../parse/short/fn.u16_to_i16.html))
    ///
    /// # Examples
    ///
    /// ```
    /// use rustils::parse::short::ToI16;
    ///
    /// assert_eq!(0_u16.to_i16(), 0_i16);
    /// assert_eq!(32767_u16.to_i16(), 32767_i16);
    /// ```
    fn to_i16(self)
        -> i16 {

        u16_to_i16(self)
    }
}

impl ToI16 for i32 {

    fn to_i16_res(self)
        -> ParseResultI16 {

        i32_to_i16_res(self)
    }

    fn to_i16(self)
        -> i16 {

        i32_to_i16(self)
    }
}

impl ToI16 for u32 {

    /// Parse [`u32`](https://doc.rust-lang.org/std/primitive.u32.html) to
    /// [`i16`](https://doc.rust-lang.org/std/primitive.i16.html)
    /// (see more: [`u32_to_i16_res`](../../parse/short/fn.u32_to_i16_res.html))
    ///
    /// # Examples
    ///
    /// ```
    /// use rustils::parse::short::ToI16;
    ///
    /// assert_eq!(0_u32.to_i16_res(), Ok(0_i16));
    /// assert_eq!(32767_u32.to_i16_res(), Ok(32767_i16));
    /// ```
    fn to_i16_res(self)
        -> ParseResultI16 {

        u32_to_i16_res(self)
    }

    /// Parse [`u32`](https://doc.rust-lang.org/std/primitive.u32.html) to
    /// [`i16`](https://doc.rust-lang.org/std/primitive.i16.html)
    /// (see more: [`u32_to_i16`](../../parse/short/fn.u32_to_i16.html))
    ///
    /// # Examples
    ///
    /// ```
    /// use rustils::parse::short::ToI16;
    ///
    /// assert_eq!(0_u32.to_i16(), 0_i16);
    /// assert_eq!(32767_u32.to_i16(), 32767_i16);
    /// ```
    fn to_i16(self)
        -> i16 {

        u32_to_i16(self)
    }
}

impl ToI16 for f32 {

    fn to_i16_res(self)
        -> ParseResultI16 {

        f32_to_i16_res(self)
    }

    fn to_i16(self)
        -> i16 {

        f32_to_i16(self)
    }
}

impl ToI16RM for f32 {

    fn to_i16_rm_res(self, rm: RoundingMode)
        -> ParseResultI16 {

        f32_to_i16_rm_res(self, rm)
    }

    fn to_i16_rm(self, rm: RoundingMode)
        -> i16 {

        match f32_to_i16_rm_res(self, rm) {
            Ok(i) => i,
            Err(err) => panic!("{}",err)
        }
    }
}

impl ToI16 for i64 {

    fn to_i16_res(self)
        -> ParseResultI16 {

        i64_to_i16_res(self)
    }

    fn to_i16(self)
        -> i16 {

        i64_to_i16(self)
    }
}

impl ToI16 for u64 {

    /// Parse [`u64`](https://doc.rust-lang.org/std/primitive.u64.html) to
    /// [`i16`](https://doc.rust-lang.org/std/primitive.i16.html)
    /// (see more: [`u64_to_i16_res`](../../parse/short/fn.u64_to_i16_res.html))
    ///
    /// # Examples
    ///
    /// ```
    /// use rustils::parse::short::ToI16;
    ///
    /// assert_eq!(0_u64.to_i16_res(), Ok(0_i16));
    /// assert_eq!(32767_u64.to_i16_res(), Ok(32767_i16));
    /// ```
    fn to_i16_res(self)
        -> ParseResultI16 {

        u64_to_i16_res(self)
    }

    /// Parse [`u64`](https://doc.rust-lang.org/std/primitive.u64.html) to
    /// [`i16`](https://doc.rust-lang.org/std/primitive.i16.html)
    /// (see more: [`u64_to_i16`](../../parse/short/fn.u64_to_i16.html))
    ///
    /// # Examples
    ///
    /// ```
    /// use rustils::parse::short::ToI16;
    ///
    /// assert_eq!(0_u64.to_i16(), 0_i16);
    /// assert_eq!(32767_u64.to_i16(), 32767_i16);
    /// ```
    fn to_i16(self)
        -> i16 {

        u64_to_i16(self)
    }
}

impl ToI16 for f64 {

    fn to_i16_res(self)
        -> ParseResultI16 {

        f64_to_i16_res(self)
    }

    fn to_i16(self)
        -> i16 {

        f64_to_i16(self)
    }
}

impl ToI16RM for f64 {

    fn to_i16_rm_res(self, rm: RoundingMode)
        -> ParseResultI16 {

        f64_to_i16_rm_res(self, rm)
    }

    fn to_i16_rm(self, rm: RoundingMode)
        -> i16 {

        f64_to_i16_rm(self, rm)
    }
}

impl ToI16 for isize {

    fn to_i16_res(self)
        -> ParseResultI16 {

        isize_to_i16_res(self)
    }

    fn to_i16(self)
        -> i16 {

        isize_to_i16(self)
    }
}

impl ToI16 for usize {

    /// Parse [`usize`](https://doc.rust-lang.org/std/primitive.usize.html) to
    /// [`i16`](https://doc.rust-lang.org/std/primitive.i16.html)
    /// (see more: [`usize_to_i16_res`](../../parse/short/fn.usize_to_i16_res.html))
    ///
    /// # Examples
    ///
    /// ```
    /// use rustils::parse::short::ToI16;
    ///
    /// assert_eq!(0_usize.to_i16_res(), Ok(0_i16));
    /// assert_eq!(32767_usize.to_i16_res(), Ok(32767_i16));
    /// ```
    fn to_i16_res(self)
        -> ParseResultI16 {

        usize_to_i16_res(self)
    }

    /// Parse [`usize`](https://doc.rust-lang.org/std/primitive.usize.html) to
    /// [`i16`](https://doc.rust-lang.org/std/primitive.i16.html)
    /// (see more: [`usize_to_i16`](../../parse/short/fn.usize_to_i16.html))
    ///
    /// # Examples
    ///
    /// ```
    /// use rustils::parse::short::ToI16;
    ///
    /// assert_eq!(0_usize.to_i16(), 0_i16);
    /// assert_eq!(32767_usize.to_i16(), 32767_i16);
    /// ```
    fn to_i16(self)
        -> i16 {

        usize_to_i16(self)
    }
}

impl ToI16 for String {

    fn to_i16_res(self)
        -> ParseResultI16 {

        string_to_i16_res(self)
    }

    fn to_i16(self)
        -> i16 {

        string_to_i16(self)
    }
}

impl ToI16 for &'static str {

    fn to_i16_res(self)
        -> ParseResultI16 {

        str_to_i16_res(self)
    }

    fn to_i16(self)
        -> i16 {

        str_to_i16(self)
    }
}
