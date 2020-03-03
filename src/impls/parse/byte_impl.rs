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

    /// Parse [`u8`](https://doc.rust-lang.org/std/primitive.u8.html) to
    /// [`i8`](https://doc.rust-lang.org/std/primitive.i8.html)
    /// (see more: [`u8_to_i8_res`](../../parse/byte/fn.u8_to_i8_res.html))
    ///
    /// # Examples
    ///
    /// ```
    /// use rustils::parse::byte::ToI8;
    ///
    /// assert_eq!(0_u8.to_i8_res(), Ok(0_i8));
    /// assert_eq!(127_u8.to_i8_res(), Ok(127_i8));
    /// ```
    fn to_i8_res(self)
        -> ParseResultI8 {

        u8_to_i8_res(self)
    }

    /// Parse [`u8`](https://doc.rust-lang.org/std/primitive.u8.html) to
    /// [`i8`](https://doc.rust-lang.org/std/primitive.i8.html)
    /// (see more: [`u8_to_i8`](../../parse/byte/fn.u8_to_i8.html))
    ///
    /// # Examples
    ///
    /// ```
    /// use rustils::parse::byte::ToI8;
    ///
    /// assert_eq!(0_u8.to_i8(), 0_i8);
    /// assert_eq!(127_u8.to_i8(), 127_i8);
    /// ```
    fn to_i8(self)
        -> i8 {

        u8_to_i8(self)
    }
}

impl ToI8 for i16 {

    /// Parse [`i16`](https://doc.rust-lang.org/std/primitive.i16.html) to
    /// [`i8`](https://doc.rust-lang.org/std/primitive.i8.html)
    /// (see more: [`i16_to_i8_res`](../../parse/byte/fn.i16_to_i8_res.html))
    ///
    /// # Examples
    ///
    /// ```
    /// use rustils::parse::byte::ToI8;
    ///
    /// assert_eq!((-128_i16).to_i8_res(), Ok(-128_i8));
    /// assert_eq!(127_i16.to_i8_res(), Ok(127_i8));
    /// ```
    fn to_i8_res(self)
        -> ParseResultI8 {

        i16_to_i8_res(self)
    }

    /// Parse [`i16`](https://doc.rust-lang.org/std/primitive.i16.html) to
    /// [`i8`](https://doc.rust-lang.org/std/primitive.i8.html)
    /// (see more: [`i16_to_i8`](../../parse/byte/fn.i16_to_i8.html))
    ///
    /// # Examples
    ///
    /// ```
    /// use rustils::parse::byte::ToI8;
    ///
    /// assert_eq!((-128_i16).to_i8(), -128_i8);
    /// assert_eq!(127_i16.to_i8(), 127_i8);
    /// ```
    fn to_i8(self)
        -> i8 {

        i16_to_i8(self)
    }
}

impl ToI8 for u16 {

    /// Parse [`u16`](https://doc.rust-lang.org/std/primitive.u16.html) to
    /// [`i8`](https://doc.rust-lang.org/std/primitive.i8.html)
    /// (see more: [`u16_to_i8_res`](../../parse/byte/fn.u16_to_i8_res.html))
    ///
    /// # Examples
    ///
    /// ```
    /// use rustils::parse::byte::ToI8;
    ///
    /// assert_eq!(0_u16.to_i8_res(), Ok(0_i8));
    /// assert_eq!(127_u16.to_i8_res(), Ok(127_i8));
    /// ```
    fn to_i8_res(self)
        -> ParseResultI8 {

        u16_to_i8_res(self)
    }

    /// Parse [`u16`](https://doc.rust-lang.org/std/primitive.u16.html) to
    /// [`i8`](https://doc.rust-lang.org/std/primitive.i8.html)
    /// (see more: [`u16_to_i8`](../../parse/byte/fn.u16_to_i8.html))
    ///
    /// # Examples
    ///
    /// ```
    /// use rustils::parse::byte::ToI8;
    ///
    /// assert_eq!(0_u16.to_i8(), 0_i8);
    /// assert_eq!(127_u16.to_i8(), 127_i8);
    /// ```
    fn to_i8(self)
        -> i8 {

        u16_to_i8(self)
    }
}

impl ToI8 for i32 {

    /// Parse [`i32`](https://doc.rust-lang.org/std/primitive.i32.html) to
    /// [`i8`](https://doc.rust-lang.org/std/primitive.i8.html)
    /// (see more: [`i32_to_i8_res`](../../parse/byte/fn.i32_to_i8_res.html))
    ///
    /// # Examples
    ///
    /// ```
    /// use rustils::parse::byte::ToI8;
    ///
    /// assert_eq!((-128_i32).to_i8_res(), Ok(-128_i8));
    /// assert_eq!(127_i32.to_i8_res(), Ok(127_i8));
    /// ```
    fn to_i8_res(self)
        -> ParseResultI8 {

        i32_to_i8_res(self)
    }

    /// Parse [`i32`](https://doc.rust-lang.org/std/primitive.i32.html) to
    /// [`i8`](https://doc.rust-lang.org/std/primitive.i8.html)
    /// (see more: [`i32_to_i8`](../../parse/byte/fn.i32_to_i8.html))
    ///
    /// # Examples
    ///
    /// ```
    /// use rustils::parse::byte::ToI8;
    ///
    /// assert_eq!((-128_i32).to_i8(), -128_i8);
    /// assert_eq!(127_i32.to_i8(), 127_i8);
    /// ```
    fn to_i8(self)
        -> i8 {

        i32_to_i8(self)
    }
}

impl ToI8 for f32 {

    /// Parse [`f32`](https://doc.rust-lang.org/std/primitive.f32.html) to
    /// [`i8`](https://doc.rust-lang.org/std/primitive.i8.html)
    /// (see more: [`f32_to_i8_res`](../../parse/byte/fn.f32_to_i8_res.html))
    ///
    /// # Examples
    ///
    /// ```
    /// use rustils::parse::byte::ToI8;
    ///
    /// assert_eq!((-128.0_f32).to_i8_res(), Ok(-128_i8));
    /// assert_eq!(127.5_f32.to_i8_res(), Ok(127_i8));
    /// ```
    fn to_i8_res(self)
        -> ParseResultI8{

        f32_to_i8_res(self)
    }

    /// Parse [`f32`](https://doc.rust-lang.org/std/primitive.f32.html) to
    /// [`i8`](https://doc.rust-lang.org/std/primitive.i8.html)
    /// (see more: [`f32_to_i8`](../../parse/byte/fn.f32_to_i8.html))
    ///
    /// # Examples
    ///
    /// ```
    /// use rustils::parse::byte::ToI8;
    ///
    /// assert_eq!((-128.0_f32).to_i8(), -128_i8);
    /// assert_eq!(127.5_f32.to_i8(), 127_i8);
    /// ```
    fn to_i8(self)
        -> i8 {

        f32_to_i8(self)
    }
}

impl ToI8 for u32 {

    /// Parse [`u32`](https://doc.rust-lang.org/std/primitive.u32.html) to
    /// [`i8`](https://doc.rust-lang.org/std/primitive.i8.html)
    /// (see more: [`u32_to_i8_res`](../../parse/byte/fn.u32_to_i8_res.html))
    ///
    /// # Examples
    ///
    /// ```
    /// use rustils::parse::byte::ToI8;
    ///
    /// assert_eq!(0_u32.to_i8_res(), Ok(0_i8));
    /// assert_eq!(127_u32.to_i8_res(), Ok(127_i8));
    /// ```
    fn to_i8_res(self)
        -> ParseResultI8 {

        u32_to_i8_res(self)
    }

    /// Parse [`u32`](https://doc.rust-lang.org/std/primitive.u32.html) to
    /// [`i8`](https://doc.rust-lang.org/std/primitive.i8.html)
    /// (see more: [`u32_to_i8`](../../parse/byte/fn.u32_to_i8.html))
    ///
    /// # Examples
    ///
    /// ```
    /// use rustils::parse::byte::ToI8;
    ///
    /// assert_eq!(0_u32.to_i8(), 0_i8);
    /// assert_eq!(127_u32.to_i8(), 127_i8);
    /// ```
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

    /// Parse [`i64`](https://doc.rust-lang.org/std/primitive.i64.html) to
    /// [`i8`](https://doc.rust-lang.org/std/primitive.i8.html)
    /// (see more: [`i64_to_i8_res`](../../parse/byte/fn.i64_to_i8_res.html))
    ///
    /// # Examples
    ///
    /// ```
    /// use rustils::parse::byte::ToI8;
    ///
    /// assert_eq!((-128_i64).to_i8_res(), Ok(-128_i8));
    /// assert_eq!(127_i64.to_i8_res(), Ok(127_i8));
    /// ```
    fn to_i8_res(self)
        -> ParseResultI8 {

        i64_to_i8_res(self)
    }

    /// Parse [`i64`](https://doc.rust-lang.org/std/primitive.i64.html) to
    /// [`i8`](https://doc.rust-lang.org/std/primitive.i8.html)
    /// (see more: [`i64_to_i8`](../../parse/byte/fn.i64_to_i8.html))
    ///
    /// # Examples
    ///
    /// ```
    /// use rustils::parse::byte::ToI8;
    ///
    /// assert_eq!((-128_i64).to_i8(), -128_i8);
    /// assert_eq!(127_i64.to_i8(), 127_i8);
    /// ```
    fn to_i8(self)
        -> i8 {

        i64_to_i8(self)
    }
}

impl ToI8 for u64 {

    /// Parse [`u64`](https://doc.rust-lang.org/std/primitive.u64.html) to
    /// [`i8`](https://doc.rust-lang.org/std/primitive.i8.html)
    /// (see more: [`u64_to_i8_res`](../../parse/byte/fn.u64_to_i8_res.html))
    ///
    /// # Examples
    ///
    /// ```
    /// use rustils::parse::byte::ToI8;
    ///
    /// assert_eq!(0_u64.to_i8_res(), Ok(0_i8));
    /// assert_eq!(127_u64.to_i8_res(), Ok(127_i8));
    /// ```
    fn to_i8_res(self)
        -> ParseResultI8 {

        u64_to_i8_res(self)
    }

    /// Parse [`u64`](https://doc.rust-lang.org/std/primitive.u64.html) to
    /// [`i8`](https://doc.rust-lang.org/std/primitive.i8.html)
    /// (see more: [`u64_to_i8`](../../parse/byte/fn.u64_to_i8.html))
    ///
    /// # Examples
    ///
    /// ```
    /// use rustils::parse::byte::ToI8;
    ///
    /// assert_eq!(0_u64.to_i8(), 0_i8);
    /// assert_eq!(127_u64.to_i8(), 127_i8);
    /// ```
    fn to_i8(self)
        -> i8 {

        u64_to_i8(self)
    }
}

impl ToI8 for f64 {

    /// Parse [`f64`](https://doc.rust-lang.org/std/primitive.f64.html) to
    /// [`i8`](https://doc.rust-lang.org/std/primitive.i8.html)
    /// (see more: [`f64_to_i8_res`](../../parse/byte/fn.f64_to_i8_res.html))
    ///
    /// # Examples
    ///
    /// ```
    /// use rustils::parse::byte::ToI8;
    ///
    /// assert_eq!((-128.0_f64).to_i8_res(), Ok(-128_i8));
    /// assert_eq!(127.5_f64.to_i8_res(), Ok(127_i8));
    /// ```
    fn to_i8_res(self)
        -> ParseResultI8 {

        f64_to_i8_res(self)
    }

    /// Parse [`f64`](https://doc.rust-lang.org/std/primitive.f64.html) to
    /// [`i8`](https://doc.rust-lang.org/std/primitive.i8.html)
    /// (see more: [`f64_to_i8`](../../parse/byte/fn.f64_to_i8.html))
    ///
    /// # Examples
    ///
    /// ```
    /// use rustils::parse::byte::ToI8;
    ///
    /// assert_eq!((-128.0_f64).to_i8(), -128_i8);
    /// assert_eq!(127.5_f64.to_i8(), 127_i8);
    /// ```
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
    /// Parse [`isize`](https://doc.rust-lang.org/std/primitive.isize.html) to
    /// [`i8`](https://doc.rust-lang.org/std/primitive.i8.html)
    /// (see more: [`isize_to_i8_res`](../../parse/byte/fn.isize_to_i8_res.html))
    ///
    /// # Examples
    ///
    /// ```
    /// use rustils::parse::byte::ToI8;
    ///
    /// assert_eq!((-128_isize).to_i8_res(), Ok(-128_i8));
    /// assert_eq!(127_isize.to_i8_res(), Ok(127_i8));
    /// ```
    fn to_i8_res(self)
        -> ParseResultI8 {

        isize_to_i8_res(self)
    }

    /// Parse [`isize`](https://doc.rust-lang.org/std/primitive.isize.html) to
    /// [`i8`](https://doc.rust-lang.org/std/primitive.i8.html)
    /// (see more: [`isize_to_i8`](../../parse/byte/fn.isize_to_i8.html))
    ///
    /// # Examples
    ///
    /// ```
    /// use rustils::parse::byte::ToI8;
    ///
    /// assert_eq!((-128_isize).to_i8(), -128_i8);
    /// assert_eq!(127_isize.to_i8(), 127_i8);
    /// ```
    fn to_i8(self)
        -> i8 {

        isize_to_i8(self)
    }
}

impl ToI8 for usize {

    /// Parse [`usize`](https://doc.rust-lang.org/std/primitive.usize.html) to
    /// [`i8`](https://doc.rust-lang.org/std/primitive.i8.html)
    /// (see more: [`usize_to_i8_res`](../../parse/byte/fn.usize_to_i8_res.html))
    ///
    /// # Examples
    ///
    /// ```
    /// use rustils::parse::byte::ToI8;
    ///
    /// assert_eq!(0_usize.to_i8_res(), Ok(0_i8));
    /// assert_eq!(127_usize.to_i8_res(), Ok(127_i8));
    ///
    /// ```
    fn to_i8_res(self)
        -> ParseResultI8 {

        usize_to_i8_res(self)
    }

    /// Parse [`usize`](https://doc.rust-lang.org/std/primitive.usize.html) to
    /// [`i8`](https://doc.rust-lang.org/std/primitive.i8.html)
    /// (see more: [`usize_to_i8`](../../parse/byte/fn.usize_to_i8.html))
    ///
    /// # Examples
    ///
    /// ```
    /// use rustils::parse::byte::ToI8;
    ///
    /// assert_eq!(0_usize.to_i8(), 0_i8);
    /// assert_eq!(127_usize.to_i8(), 127_i8);
    /// ```
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
