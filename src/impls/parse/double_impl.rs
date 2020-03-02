use parse::double::*;

impl ToF64 for bool {

    /// Parse [`bool`](https://doc.rust-lang.org/std/primitive.bool.html) to
    /// [`f64`](https://doc.rust-lang.org/std/primitive.f64.html)
    /// (see more: [`bool_to_f64_res`](../../parse/double/fn.bool_to_f64_res.html))
    ///
    /// # Examples
    ///
    /// ```
    /// use rustils::parse::double::ToF64;
    ///
    /// assert_eq!(true.to_f64_res(), Ok(1.0_f64));
    /// assert_eq!(false.to_f64_res(), Ok(0.0_f64));
    /// ```
    fn to_f64_res(self)
        -> ParseResultF64 {

        bool_to_f64_res(self)
    }

    /// Parse [`bool`](https://doc.rust-lang.org/std/primitive.bool.html) to
    /// [`f64`](https://doc.rust-lang.org/std/primitive.f64.html)
    /// (see more: [`bool_to_f64`](../../parse/double/fn.bool_to_f64.html))
    ///
    /// # Examples
    ///
    /// ```
    /// use rustils::parse::double::ToF64;
    ///
    /// assert_eq!(true.to_f64(), 1.0_f64);
    /// assert_eq!(false.to_f64(), 0.0_f64);
    /// ```
    fn to_f64(self)
        -> f64 {

        bool_to_f64(self)
    }
}

impl ToF64 for String {

    fn to_f64_res(self)
        -> ParseResultF64 {

        string_to_f64_res(self)
    }

    fn to_f64(self)
        -> f64 {

        string_to_f64(self)
    }
}

impl ToF64 for &'static str {

    fn to_f64_res(self)
        -> ParseResultF64 {

        str_to_f64_res(self)
    }

    fn to_f64(self)
        -> f64 {

        str_to_f64(self)
    }
}
