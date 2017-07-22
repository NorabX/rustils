// <editor-fold> # Uses

use parse::byte::*;
use RoundingMode;
// </editor-fold>

// <editor-fold> # Impls

// <editor-fold> ## ToI8

impl ToI8 for bool {

    fn to_i8_res(self)
        -> ParseResultI8 {

        bool_to_i8_res(self)
    }

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
// </editor-fold>

// </editor-fold>
