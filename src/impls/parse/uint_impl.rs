// <editor-fold> # Uses

use parse::uint::*;
use RoundingMode;
// </editor-fold>

// <editor-fold> # Impls

// <editor-fold> ## ToU32

impl ToU32 for bool {

    fn to_u32_res(self)
        -> ParseResultU32 {

        bool_to_u32_res(self)
    }

    fn to_u32(self)
        -> u32 {

        bool_to_u32(self)
    }
}

impl ToU32 for i8 {

    fn to_u32_res(self)
        -> ParseResultU32 {

        i8_to_u32_res(self)
    }

    fn to_u32(self)
        -> u32 {

        i8_to_u32(self)
    }
}

impl ToU32 for i16 {

    fn to_u32_res(self)
        -> ParseResultU32 {

        i16_to_u32_res(self)
    }

    fn to_u32(self)
        -> u32 {

        i16_to_u32(self)
    }
}

impl ToU32 for i32 {

    fn to_u32_res(self)
        -> ParseResultU32 {

        i32_to_u32_res(self)
    }

    fn to_u32(self)
        -> u32 {

        i32_to_u32(self)
    }
}

impl ToU32 for f32 {

    fn to_u32_res(self)
        -> ParseResultU32 {

        f32_to_u32_res(self)
    }

    fn to_u32(self)
        -> u32 {

        f32_to_u32(self)
    }
}

impl ToU32RM for f32 {

    fn to_u32_rm_res(self, rm: RoundingMode)
        -> ParseResultU32 {

        f32_to_u32_rm_res(self, rm)
    }

    fn to_u32_rm(self, rm: RoundingMode)
        -> u32 {

        f32_to_u32_rm(self, rm)
    }
}

impl ToU32 for i64 {

    fn to_u32_res(self)
        -> ParseResultU32 {

        i64_to_u32_res(self)
    }

    fn to_u32(self)
        -> u32 {

        i64_to_u32(self)
    }
}

impl ToU32 for u64 {

    fn to_u32_res(self)
        -> ParseResultU32 {

        u64_to_u32_res(self)
    }

    fn to_u32(self)
        -> u32 {

        u64_to_u32(self)
    }
}


impl ToU32 for f64 {

    fn to_u32_res(self)
        -> ParseResultU32 {

        f64_to_u32_res(self)
    }

    fn to_u32(self)
        -> u32 {

        f64_to_u32(self)
    }
}

impl ToU32RM for f64 {

    fn to_u32_rm_res(self, rm: RoundingMode)
        -> ParseResultU32 {

        f64_to_u32_rm_res(self, rm)
    }

    fn to_u32_rm(self, rm: RoundingMode)
        -> u32 {

        f64_to_u32_rm(self, rm)
    }
}

impl ToU32 for isize {

    fn to_u32_res(self)
        -> ParseResultU32 {

        isize_to_u32_res(self)
    }

    fn to_u32(self)
        -> u32 {

        isize_to_u32(self)
    }
}

impl ToU32 for usize {

    fn to_u32_res(self)
        -> ParseResultU32 {

        usize_to_u32_res(self)
    }

    fn to_u32(self)
        -> u32 {

        usize_to_u32(self)
    }
}

impl ToU32 for String {

    fn to_u32_res(self)
        -> ParseResultU32 {

        string_to_u32_res(self)
    }

    fn to_u32(self)
        -> u32 {

        string_to_u32(self)
    }
}

impl ToU32 for &'static str {

    fn to_u32_res(self)
        -> ParseResultU32 {

        str_to_u32_res(self)
    }

    fn to_u32(self)
        -> u32 {

        str_to_u32(self)
    }
}
// </editor-fold>

// </editor-fold>
