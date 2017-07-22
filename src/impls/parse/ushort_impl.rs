// <editor-fold> # Uses

use parse::ushort::*;
use RoundingMode;
// </editor-fold>

// <editor-fold> # Impls

// <editor-fold> ## ToU16

impl ToU16 for bool {

    fn to_u16_res(self)
        -> ParseResultU16 {

        bool_to_u16_res(self)
    }

    fn to_u16(self)
        -> u16 {

        bool_to_u16(self)
    }
}

impl ToU16 for i8 {

    fn to_u16_res(self)
        -> ParseResultU16 {

        i8_to_u16_res(self)
    }

    fn to_u16(self)
        -> u16 {

        i8_to_u16(self)
    }
}

impl ToU16 for i16 {

    fn to_u16_res(self)
        -> ParseResultU16 {

        i16_to_u16_res(self)
    }

    fn to_u16(self)
        -> u16 {

        i16_to_u16(self)
    }
}

impl ToU16 for i32 {

    fn to_u16_res(self)
        -> ParseResultU16 {

        i32_to_u16_res(self)
    }

    fn to_u16(self)
        -> u16 {

        i32_to_u16(self)
    }
}

impl ToU16 for u32 {

    fn to_u16_res(self)
        -> ParseResultU16 {

        u32_to_u16_res(self)
    }

    fn to_u16(self)
        -> u16 {

        u32_to_u16(self)
    }
}

impl ToU16 for f32 {

    fn to_u16_res(self)
        -> ParseResultU16 {

        f32_to_u16_res(self)
    }

    fn to_u16(self)
        -> u16 {

        f32_to_u16(self)
    }
}

impl ToU16RM for f32 {

    fn to_u16_rm_res(self, rm: RoundingMode)
        -> ParseResultU16 {

        f32_to_u16_rm_res(self, rm)
    }

    fn to_u16_rm(self, rm: RoundingMode)
        -> u16 {

        f32_to_u16_rm(self, rm)
    }
}

impl ToU16 for i64 {

    fn to_u16_res(self)
        -> ParseResultU16 {

        i64_to_u16_res(self)
    }

    fn to_u16(self)
        -> u16 {

        i64_to_u16(self)
    }
}

impl ToU16 for u64 {

    fn to_u16_res(self)
        -> ParseResultU16 {

        u64_to_u16_res(self)
    }

    fn to_u16(self)
        -> u16 {

        u64_to_u16(self)
    }
}

impl ToU16 for f64 {

    fn to_u16_res(self)
        -> ParseResultU16 {

        f64_to_u16_res(self)
    }

    fn to_u16(self)
        -> u16 {

        f64_to_u16(self)
    }
}

impl ToU16RM for f64 {

    fn to_u16_rm_res(self, rm: RoundingMode)
        -> ParseResultU16 {

        f64_to_u16_rm_res(self, rm)
    }

    fn to_u16_rm(self, rm: RoundingMode)
        -> u16 {

        f64_to_u16_rm(self, rm)
    }
}

impl ToU16 for isize {

    fn to_u16_res(self)
        -> ParseResultU16 {

        isize_to_u16_res(self)
    }

    fn to_u16(self)
        -> u16 {

        isize_to_u16(self)
    }
}

impl ToU16 for usize {

    fn to_u16_res(self)
        -> ParseResultU16 {

        usize_to_u16_res(self)
    }

    fn to_u16(self)
        -> u16 {

        usize_to_u16(self)
    }
}

impl ToU16 for String {

    fn to_u16_res(self)
        -> ParseResultU16 {

        string_to_u16_res(self)
    }

    fn to_u16(self)
        -> u16 {

        string_to_u16(self)
    }
}

impl ToU16 for &'static str {

    fn to_u16_res(self)
        -> ParseResultU16 {

        str_to_u16_res(self)
    }

    fn to_u16(self)
        -> u16 {

        str_to_u16(self)
    }
}
// </editor-fold>

// </editor-fold>
