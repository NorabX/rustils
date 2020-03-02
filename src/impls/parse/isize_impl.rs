use parse::isize::*;
use RoundingMode;

impl ToIsize for bool {

    /// Parse [`bool`](https://doc.rust-lang.org/std/primitive.bool.html) to
    /// [`isize`](https://doc.rust-lang.org/std/primitive.isize.html)
    /// (see more: [`bool_to_isize_res`](../../parse/isize/fn.bool_to_isize_res.html))
    ///
    /// # Examples
    ///
    /// ```
    /// use rustils::parse::isize::ToIsize;
    ///
    /// assert_eq!(true.to_isize_res(), Ok(1_isize));
    /// assert_eq!(false.to_isize_res(), Ok(0_isize));
    /// ```
    fn to_isize_res(self)
        -> ParseResultIsize {

        bool_to_isize_res(self)
    }

    /// Parse [`bool`](https://doc.rust-lang.org/std/primitive.bool.html) to
    /// [`isize`](https://doc.rust-lang.org/std/primitive.isize.html)
    /// (see more: [`bool_to_isize`](../../parse/isize/fn.bool_to_isize.html))
    ///
    /// # Examples
    ///
    /// ```
    /// use rustils::parse::isize::ToIsize;
    ///
    /// assert_eq!(true.to_isize(), 1_isize);
    /// assert_eq!(false.to_isize(), 0_isize);
    /// ```
    fn to_isize(self)
        -> isize {

        bool_to_isize(self)
    }
}

impl ToIsize for f32 {

    fn to_isize_res(self)
        -> ParseResultIsize {

        f32_to_isize_res(self)
    }

    fn to_isize(self)
        -> isize {

        f32_to_isize(self)
    }
}

impl ToIsizeRM for f32 {

    fn to_isize_rm_res(self, rm: RoundingMode)
        -> ParseResultIsize {

        f32_to_isize_rm_res(self, rm)
    }

    fn to_isize_rm(self, rm: RoundingMode)
        -> isize {

        f32_to_isize_rm(self, rm)
    }
}

impl ToIsize for u64 {

    fn to_isize_res(self)
        -> ParseResultIsize {

        u64_to_isize_res(self)
    }

    fn to_isize(self)
        -> isize {

        u64_to_isize(self)
    }
}

impl ToIsize for f64 {

    fn to_isize_res(self)
        -> ParseResultIsize {

        f64_to_isize_res(self)
    }

    fn to_isize(self)
        -> isize {

        f64_to_isize(self)
    }
}

impl ToIsizeRM for f64 {

    fn to_isize_rm_res(self, rm: RoundingMode)
        -> ParseResultIsize {

        f64_to_isize_rm_res(self, rm)
    }

    fn to_isize_rm(self, rm: RoundingMode)
        -> isize {

        f64_to_isize_rm(self, rm)
    }
}

impl ToIsize for usize {

    fn to_isize_res(self)
        -> ParseResultIsize {

        usize_to_isize_res(self)
    }

    fn to_isize(self)
        -> isize {

         usize_to_isize(self)
    }
}

impl ToIsize for String {

    fn to_isize_res(self)
        -> ParseResultIsize {

        string_to_isize_res(self)
    }

    fn to_isize(self)
        -> isize {

        string_to_isize(self)
    }
}

impl ToIsize for &'static str {

    fn to_isize_res(self)
        -> ParseResultIsize {

        str_to_isize_res(self)
    }

    fn to_isize(self)
        -> isize {

        str_to_isize(self)
    }
}
