use parse::float::*;

impl ToF32 for bool {

    /// Parse [`bool`](https://doc.rust-lang.org/std/primitive.bool.html) to
    /// [`f32`](https://doc.rust-lang.org/std/primitive.f32.html)
    /// (see more: [`bool_to_f32_res`](../../parse/float/fn.bool_to_f32_res.html))
    ///
    /// # Examples
    ///
    /// ```
    /// use rustils::parse::float::ToF32;
    ///
    /// assert_eq!(true.to_f32_res(), Ok(1.0_f32));
    /// assert_eq!(false.to_f32_res(), Ok(0.0_f32));
    /// ```
    fn to_f32_res(self)
        -> ParseResultF32 {

        bool_to_f32_res(self)
    }

    /// Parse [`bool`](https://doc.rust-lang.org/std/primitive.bool.html) to
    /// [`f32`](https://doc.rust-lang.org/std/primitive.f32.html)
    /// (see more: [`bool_to_f32`](../../parse/float/fn.bool_to_f32.html))
    ///
    /// # Examples
    ///
    /// ```
    /// use rustils::parse::float::ToF32;
    ///
    /// assert_eq!(true.to_f32(), 1.0_f32);
    /// assert_eq!(false.to_f32(), 0.0_f32);
    /// ```
    fn to_f32(self)
        -> f32 {

        bool_to_f32(self)
    }
}

impl ToF32 for String {

    fn to_f32_res(self)
        -> ParseResultF32 {

        string_to_f32_res(self)
    }

    fn to_f32(self)
        -> f32 {

        string_to_f32(self)
    }
}

impl ToF32 for &'static str {

    fn to_f32_res(self)
        -> ParseResultF32 {

        str_to_f32_res(self)
    }

    fn to_f32(self)
        -> f32 {

        str_to_f32(self)
    }
}
