use parse::long::*;
use RoundingMode;

impl ToI64 for bool {

    /// Parse [`bool`](https://doc.rust-lang.org/std/primitive.bool.html) to
    /// [`i64`](https://doc.rust-lang.org/std/primitive.i64.html)
    /// (see more: [`bool_to_i64_res`](../../parse/long/fn.bool_to_i64_res.html))
    ///
    /// # Examples
    ///
    /// ```
    /// use rustils::parse::long::ToI64;
    ///
    /// assert_eq!(true.to_i64_res(), Ok(1_i64));
    /// assert_eq!(false.to_i64_res(), Ok(0_i64));
    /// ```
    fn to_i64_res(self)
    -> ParseResultI64 {

        bool_to_i64_res(self)
    }

    /// Parse [`bool`](https://doc.rust-lang.org/std/primitive.bool.html) to
    /// [`i64`](https://doc.rust-lang.org/std/primitive.i64.html)
    /// (see more: [`bool_to_i64`](../../parse/long/fn.bool_to_i64.html))
    ///
    /// # Examples
    ///
    /// ```
    /// use rustils::parse::long::ToI64;
    ///
    /// assert_eq!(true.to_i64(), 1_i64);
    /// assert_eq!(false.to_i64(), 0_i64);
    /// ```
    fn to_i64(self)
        -> i64 {

        bool_to_i64(self)
    }
}

impl ToI64 for f32 {

    fn to_i64_res(self)
        -> ParseResultI64 {

        f32_to_i64_res(self)
    }

    fn to_i64(self)
        -> i64 {

        f32_to_i64(self)
    }
}

impl ToI64RM for f32 {

    fn to_i64_rm_res(self, rm: RoundingMode)
        -> ParseResultI64 {

        f32_to_i64_rm_res(self, rm)
    }

    fn to_i64_rm(self, rm: RoundingMode)
        -> i64 {

        f32_to_i64_rm(self, rm)
    }
}

impl ToI64 for u64 {

    fn to_i64_res(self)
        -> ParseResultI64 {

        u64_to_i64_res(self)
    }

    fn to_i64(self)
        -> i64 {

        u64_to_i64(self)
    }
}

impl ToI64 for f64 {

    fn to_i64_res(self)
    -> ParseResultI64 {

        f64_to_i64_res(self)
    }

    fn to_i64(self)
        -> i64 {

        f64_to_i64(self)
    }
}

impl ToI64RM for f64 {

    fn to_i64_rm_res(self, rm: RoundingMode)
        -> ParseResultI64 {

        f64_to_i64_rm_res(self, rm)
    }

    fn to_i64_rm(self, rm: RoundingMode)
        -> i64 {

        f64_to_i64_rm(self, rm)
    }
}

impl ToI64 for usize {

    fn to_i64_res(self)
        -> ParseResultI64 {

        usize_to_i64_res(self)
    }

    fn to_i64(self)
        -> i64 {

        usize_to_i64(self)
    }
}

impl ToI64 for String {

    fn to_i64_res(self)
        -> ParseResultI64 {

        string_to_i64_res(self)
    }

    fn to_i64(self)
        -> i64 {

        string_to_i64(self)
    }
}

impl ToI64 for &'static str {

    fn to_i64_res(self)
        -> ParseResultI64 {

        str_to_i64_res(self)
    }

    fn to_i64(self)
        -> i64 {

        str_to_i64(self)
    }
}
