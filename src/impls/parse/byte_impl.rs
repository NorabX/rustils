use parse::byte::*;
use RoundingMode;

impl ToI8 for bool {

    /// Parse [`bool`](https://doc.rust-lang.org/std/primitive.bool.html) to
    /// [`i8`](https://doc.rust-lang.org/std/primitive.i8.html)
    /// (see more: [`bool_to_i8_res`](../../parse/byte/fn.bool_to_i8_res.html))
    ///
    /// # Examples
    ///
    /// ```
    /// use rustils::parse::byte::ToI8;
    ///
    /// assert_eq!(true.to_i8_res(), Ok(1_i8));
    /// assert_eq!(false.to_i8_res(), Ok(0_i8));
    /// ```
    fn to_i8_res(self)
        -> ParseResultI8 {

        bool_to_i8_res(self)
    }

    /// Parse [`bool`](https://doc.rust-lang.org/std/primitive.bool.html) to
    /// [`i8`](https://doc.rust-lang.org/std/primitive.i8.html)
    /// (see more: [`bool_to_i8`](../../parse/byte/fn.bool_to_i8.html))
    ///
    /// # Examples
    ///
    /// ```
    /// use rustils::parse::byte::ToI8;
    ///
    /// assert_eq!(true.to_i8(), 1_i8);
    /// assert_eq!(false.to_i8(), 0_i8);
    /// ```
    fn to_i8(self)
        -> i8 {

        bool_to_i8(self)
    }
}

impl ToI8 for u8 {

    fn to_i8_res(self)
        -> ParseResultI8 {

        u8_to_i8_res(self)
    }

    fn to_i8(self)
        -> i8 {

        u8_to_i8(self)
    }
}

impl ToI8 for i16 {

    fn to_i8_res(self)
        -> ParseResultI8 {

        i16_to_i8_res(self)
    }

    fn to_i8(self)
        -> i8 {

        i16_to_i8(self)
    }
}

impl ToI8 for u16 {

    fn to_i8_res(self)
        -> ParseResultI8 {

        u16_to_i8_res(self)
    }

    fn to_i8(self)
        -> i8 {

        u16_to_i8(self)
    }
}

impl ToI8 for i32 {

    fn to_i8_res(self)
        -> ParseResultI8 {

        i32_to_i8_res(self)
    }

    fn to_i8(self)
        -> i8 {

        i32_to_i8(self)
    }
}

impl ToI8 for f32 {

    fn to_i8_res(self)
        -> ParseResultI8{

        f32_to_i8_res(self)
    }

    fn to_i8(self)
        -> i8 {

        f32_to_i8(self)
    }
}

impl ToI8 for u32 {

    fn to_i8_res(self)
        -> ParseResultI8 {

        u32_to_i8_res(self)
    }

    fn to_i8(self)
        -> i8 {

        u32_to_i8(self)
    }
}

impl ToI8RM for f32 {

    fn to_i8_rm_res(self, rm: RoundingMode)
        -> ParseResultI8 {

        f32_to_i8_rm_res(self, rm)
    }

    fn to_i8_rm(self, rm: RoundingMode)
        -> i8 {

        f32_to_i8_rm(self, rm)
    }
}

impl ToI8 for i64 {

    fn to_i8_res(self)
        -> ParseResultI8 {

        i64_to_i8_res(self)
    }

    fn to_i8(self)
        -> i8 {

        i64_to_i8(self)
    }
}

impl ToI8 for u64 {

    fn to_i8_res(self)
        -> ParseResultI8 {

        u64_to_i8_res(self)
    }

    fn to_i8(self)
        -> i8 {

        u64_to_i8(self)
    }
}

impl ToI8 for f64 {

    fn to_i8_res(self)
        -> ParseResultI8 {

        f64_to_i8_res(self)
    }

    fn to_i8(self)
        -> i8 {

        f64_to_i8(self)
    }
}

impl ToI8RM for f64 {

    fn to_i8_rm_res(self, rm: RoundingMode)
        -> ParseResultI8 {

        f64_to_i8_rm_res(self, rm)
    }

    fn to_i8_rm(self, rm: RoundingMode)
        -> i8 {

        f64_to_i8_rm(self, rm)
    }
}

impl ToI8 for isize {

    fn to_i8_res(self)
        -> ParseResultI8 {

        isize_to_i8_res(self)
    }

    fn to_i8(self)
        -> i8 {

        isize_to_i8(self)
    }
}

impl ToI8 for usize {

    fn to_i8_res(self)
        -> ParseResultI8 {

        usize_to_i8_res(self)
    }

    fn to_i8(self)
        -> i8 {

        usize_to_i8(self)
    }
}

impl ToI8 for String {

    fn to_i8_res(self)
        -> ParseResultI8 {

        string_to_i8_res(self)
    }

    fn to_i8(self)
        -> i8 {

        string_to_i8(self)
    }
}

impl ToI8 for &'static str {

    fn to_i8_res(self)
        -> ParseResultI8 {

        str_to_i8_res(self)
    }

    fn to_i8(self)
        -> i8 {

        str_to_i8(self)
    }
}
