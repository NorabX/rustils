use parse::int::*;
use RoundingMode;

impl ToI32 for bool {

    fn to_i32_res(self)
        -> ParseResultI32 {

        bool_to_i32_res(self)
    }

    fn to_i32(self)
        -> i32 {

        bool_to_i32(self)
    }
}

impl ToI32 for f32 {

    fn to_i32_res(self)
        -> ParseResultI32 {

        f32_to_i32_res(self)
    }

    fn to_i32(self)
        -> i32 {

        f32_to_i32(self)
    }
}

impl ToI32RM for f32 {

    fn to_i32_rm_res(self, rm: RoundingMode)
        -> ParseResultI32 {

        f32_to_i32_rm_res(self, rm)
    }

    fn to_i32_rm(self, rm: RoundingMode)
        -> i32 {

        f32_to_i32_rm(self, rm)
    }
}

impl ToI32 for i64 {

    fn to_i32_res(self)
        -> ParseResultI32 {

        i64_to_i32_res(self)
    }

    fn to_i32(self)
        -> i32 {

        i64_to_i32(self)
    }
}

impl ToI32 for u64 {

    fn to_i32_res(self)
        -> ParseResultI32 {

        u64_to_i32_res(self)
    }

    fn to_i32(self)
        -> i32 {

        u64_to_i32(self)
    }
}

impl ToI32 for f64 {

    fn to_i32_res(self)
        -> ParseResultI32 {

        f64_to_i32_res(self)
    }

    fn to_i32(self)
        -> i32 {

        f64_to_i32(self)
    }
}

impl ToI32RM for f64 {

    fn to_i32_rm_res(self, rm: RoundingMode)
        -> ParseResultI32 {

        f64_to_i32_rm_res(self, rm)
    }

    fn to_i32_rm(self, rm: RoundingMode)
        -> i32 {

        f64_to_i32_rm(self, rm)
    }
}

impl ToI32 for isize {

    fn to_i32_res(self)
        -> ParseResultI32 {

        isize_to_i32_res(self)
    }

    fn to_i32(self)
        -> i32 {

        isize_to_i32(self)
    }
}

impl ToI32 for usize {

    fn to_i32_res(self)
        -> ParseResultI32 {

        usize_to_i32_res(self)
    }

    fn to_i32(self)
        -> i32 {

        usize_to_i32(self)
    }
}

impl ToI32 for String {

    fn to_i32_res(self)
        -> ParseResultI32 {

        string_to_i32_res(self)
    }

    fn to_i32(self)
        -> i32 {

        string_to_i32(self)
    }
}

impl ToI32 for &'static str {

    fn to_i32_res(self)
        -> ParseResultI32 {

        str_to_i32_res(self)
    }

    fn to_i32(self)
        -> i32 {

        str_to_i32(self)
    }
}
