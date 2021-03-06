use parse::ulong::*;
use RoundingMode;

impl ToU64 for bool {

    /// Parse [`bool`](https://doc.rust-lang.org/std/primitive.bool.html) to
    /// [`u64`](https://doc.rust-lang.org/std/primitive.u64.html)
    /// (see more: [`bool_to_u64_res`](../../parse/ulong/fn.bool_to_u64_res.html))
    ///
    /// # Examples
    ///
    /// ```
    /// use rustils::parse::ulong::ToU64;
    ///
    /// assert_eq!(true.to_u64_res(), Ok(1_u64));
    /// assert_eq!(false.to_u64_res(), Ok(0_u64));
    /// ```
    fn to_u64_res(self)
        -> ParseResultU64 {

        bool_to_u64_res(self)
    }

    /// Parse [`bool`](https://doc.rust-lang.org/std/primitive.bool.html) to
    /// [`u64`](https://doc.rust-lang.org/std/primitive.u64.html)
    /// (see more: [`bool_to_u64`](../../parse/ulong/fn.bool_to_u64.html))
    ///
    /// # Examples
    ///
    /// ```
    /// use rustils::parse::ulong::ToU64;
    ///
    /// assert_eq!(true.to_u64(), 1_u64);
    /// assert_eq!(false.to_u64(), 0_u64);
    /// ```
    fn to_u64(self)
        -> u64 {

        bool_to_u64(self)
    }
}

impl ToU64 for i8 {

    fn to_u64_res(self)
        -> ParseResultU64 {

        i8_to_u64_res(self)
    }

    fn to_u64(self)
        -> u64 {

        i8_to_u64(self)
    }
}

impl ToU64 for i16 {

    fn to_u64_res(self)
        -> ParseResultU64 {

        i16_to_u64_res(self)
    }

    fn to_u64(self)
        -> u64 {

        i16_to_u64(self)
    }
}


impl ToU64 for i32 {

    fn to_u64_res(self)
        -> ParseResultU64 {

        i32_to_u64_res(self)
    }

    fn to_u64(self)
        -> u64 {

        i32_to_u64(self)
    }
}

impl ToU64 for f32 {

    fn to_u64_res(self)
        -> ParseResultU64 {

        f32_to_u64_res(self)
    }

    fn to_u64(self)
        -> u64 {

        f32_to_u64(self)
    }
}

impl ToU64RM for f32 {

    fn to_u64_rm_res(self, rm: RoundingMode)
        -> ParseResultU64 {

        f32_to_u64_rm_res(self, rm)
    }

    fn to_u64_rm(self, rm: RoundingMode)
        -> u64 {

        f32_to_u64_rm(self, rm)
    }
}

impl ToU64 for i64 {

    fn to_u64_res(self)
        -> ParseResultU64 {

        i64_to_u64_res(self)
    }

    fn to_u64(self)
        -> u64 {

        i64_to_u64(self)
    }
}

impl ToU64 for f64 {

    fn to_u64_res(self)
        -> ParseResultU64 {

        f64_to_u64_res(self)
    }

    fn to_u64(self)
        -> u64 {

        f64_to_u64(self)
    }
}

impl ToU64RM for f64 {

    fn to_u64_rm_res(self, rm: RoundingMode)
        -> ParseResultU64 {

        f64_to_u64_rm_res(self, rm)
    }

    fn to_u64_rm(self, rm: RoundingMode)
        -> u64 {

        f64_to_u64_rm(self, rm)
    }
}

impl ToU64 for isize {

    fn to_u64_res(self)
        -> ParseResultU64 {

        isize_to_u64_res(self)
    }

    fn to_u64(self)
        -> u64 {

        isize_to_u64(self)
    }
}

impl ToU64 for String {

    fn to_u64_res(self)
        -> ParseResultU64 {

        string_to_u64_res(self)
    }

    fn to_u64(self)
        -> u64 {

        string_to_u64(self)
    }
}

impl ToU64 for &'static str {

    fn to_u64_res(self)
        -> ParseResultU64 {

        str_to_u64_res(self)
    }

    fn to_u64(self)
        -> u64 {

        str_to_u64(self)
    }
}
