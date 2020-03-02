use parse::ubyte::*;
use RoundingMode;

impl ToU8 for bool {

    /// Parse [`bool`](https://doc.rust-lang.org/std/primitive.bool.html) to
    /// [`u8`](https://doc.rust-lang.org/std/primitive.u8.html)
    /// (see more: [`bool_to_u8_res`](../../parse/ubyte/fn.bool_to_u8_res.html))
    ///
    /// # Examples
    ///
    /// ```
    /// use rustils::parse::ubyte::ToU8;
    ///
    /// assert_eq!(true.to_u8_res(), Ok(1_u8));
    /// assert_eq!(false.to_u8_res(), Ok(0_u8));
    /// ```
    fn to_u8_res(self)
        -> ParseResultU8 {

        bool_to_u8_res(self)
    }

    /// Parse [`bool`](https://doc.rust-lang.org/std/primitive.bool.html) to
    /// [`u8`](https://doc.rust-lang.org/std/primitive.u8.html)
    /// (see more: [`bool_to_u8`](../../parse/ubyte/fn.bool_to_u8.html))
    ///
    /// # Examples
    ///
    /// ```
    /// use rustils::parse::ubyte::ToU8;
    ///
    /// assert_eq!(true.to_u8(), 1_u8);
    /// assert_eq!(false.to_u8(), 0_u8);
    /// ```
    fn to_u8(self)
        -> u8 {

        bool_to_u8(self)
    }
}

impl ToU8 for i8 {

    fn to_u8_res(self)
        -> ParseResultU8 {

        i8_to_u8_res(self)
    }

    fn to_u8(self)
        -> u8 {

        i8_to_u8(self)
    }
}

impl ToU8 for i16 {

    fn to_u8_res(self)
        -> ParseResultU8 {

        i16_to_u8_res(self)
    }

    fn to_u8(self)
        -> u8 {

        i16_to_u8(self)
    }
}

impl ToU8 for u16 {

    fn to_u8_res(self)
        -> ParseResultU8 {

        u16_to_u8_res(self)
    }

    fn to_u8(self)
        -> u8 {

        u16_to_u8(self)
    }
}

impl ToU8 for i32 {

    fn to_u8_res(self)
        -> ParseResultU8 {

        i32_to_u8_res(self)
    }

    fn to_u8(self)
        -> u8 {

        i32_to_u8(self)
    }
}

impl ToU8 for u32 {

    fn to_u8_res(self)
        -> ParseResultU8 {

        u32_to_u8_res(self)
    }

    fn to_u8(self)
        -> u8 {

        u32_to_u8(self)
    }
}

impl ToU8 for f32 {

    fn to_u8_res(self)
        -> ParseResultU8{

        f32_to_u8_res(self)
    }

    fn to_u8(self)
        -> u8 {

        f32_to_u8(self)
    }
}

impl ToU8RM for f32 {

    fn to_u8_rm_res(self, rm: RoundingMode)
        -> ParseResultU8 {

        f32_to_u8_rm_res(self, rm)
    }

    fn to_u8_rm(self, rm: RoundingMode)
        -> u8 {

        f32_to_u8_rm(self, rm)
    }
}

impl ToU8 for i64 {

    fn to_u8_res(self)
        -> ParseResultU8 {

        i64_to_u8_res(self)
    }

    fn to_u8(self)
        -> u8 {

        i64_to_u8(self)
    }
}

impl ToU8 for u64 {

    fn to_u8_res(self)
        -> ParseResultU8 {

        u64_to_u8_res(self)
    }

    fn to_u8(self)
        -> u8 {

        u64_to_u8(self)
    }
}

impl ToU8 for f64 {

    fn to_u8_res(self)
        -> ParseResultU8{

        f64_to_u8_res(self)
    }

    fn to_u8(self)
        -> u8 {

        f64_to_u8(self)
    }
}

impl ToU8RM for f64 {

    fn to_u8_rm_res(self, rm: RoundingMode)
        -> ParseResultU8 {

        f64_to_u8_rm_res(self, rm)
    }

    fn to_u8_rm(self, rm: RoundingMode)
        -> u8 {

        f64_to_u8_rm(self, rm)
    }
}

impl ToU8 for isize {

    fn to_u8_res(self)
        -> ParseResultU8 {

        isize_to_u8_res(self)
    }

    fn to_u8(self)
        -> u8 {

        isize_to_u8(self)
    }
}

impl ToU8 for usize {

    fn to_u8_res(self)
        -> ParseResultU8 {

        usize_to_u8_res(self)
    }

    fn to_u8(self)
        -> u8 {

        usize_to_u8(self)
    }
}

impl ToU8 for String {

    fn to_u8_res(self)
        -> ParseResultU8 {

        string_to_u8_res(self)
    }

    fn to_u8(self)
        -> u8 {

        string_to_u8(self)
    }
}

impl ToU8 for &'static str {

    fn to_u8_res(self)
        -> ParseResultU8 {

        str_to_u8_res(self)
    }

    fn to_u8(self)
        -> u8 {

        str_to_u8(self)
    }
}
