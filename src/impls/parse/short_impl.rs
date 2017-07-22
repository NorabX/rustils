// <editor-fold> # Uses

use parse::short::*;
use RoundingMode;
// </editor-fold>

// <editor-fold> # Impl

// <editor-fold> ## ToI16

impl ToI16 for bool {

    fn to_i16_res(self)
        -> ParseResultI16 {

        bool_to_i16_res(self)
    }

    fn to_i16(self)
        -> i16 {

        bool_to_i16(self)
    }
}

impl ToI16 for u16 {

    fn to_i16_res(self)
        -> ParseResultI16 {

        u16_to_i16_res(self)
    }

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

    fn to_i16_res(self)
        -> ParseResultI16 {

        u32_to_i16_res(self)
    }

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

    fn to_i16_res(self)
        -> ParseResultI16 {

        u64_to_i16_res(self)
    }

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

    fn to_i16_res(self)
        -> ParseResultI16 {

        usize_to_i16_res(self)
    }

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
// </editor-fold>

// </editor-fold>
