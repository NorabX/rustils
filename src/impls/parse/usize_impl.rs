use parse::usize::*;
use RoundingMode;

impl ToUsize for bool {

    fn to_usize_res(self)
        -> ParseResultUsize {

        bool_to_usize_res(self)
    }

    fn to_usize(self)
        -> usize {

        bool_to_usize(self)
    }
}

impl ToUsize for i8 {

    fn to_usize_res(self)
        -> ParseResultUsize {

        i8_to_usize_res(self)
    }

    fn to_usize(self)
        -> usize {

        i8_to_usize(self)
    }
}

impl ToUsize for i16 {

    fn to_usize_res(self)
        -> ParseResultUsize {

        i16_to_usize_res(self)
    }

    fn to_usize(self)
        -> usize {

        i16_to_usize(self)
    }
}

impl ToUsize for i32 {

    fn to_usize_res(self)
        -> ParseResultUsize {

        i32_to_usize_res(self)
    }

    fn to_usize(self)
        -> usize {

        i32_to_usize(self)
    }
}

impl ToUsize for f32 {

    fn to_usize_res(self)
        -> ParseResultUsize {

        f32_to_usize_res(self)
    }

    fn to_usize(self)
        -> usize {

        f32_to_usize(self)
    }
}

impl ToUsizeRM for f32 {

    fn to_usize_rm_res(self, rm: RoundingMode)
        -> ParseResultUsize {

        f32_to_usize_rm_res(self, rm)
    }

    fn to_usize_rm(self, rm: RoundingMode)
        -> usize {

        f32_to_usize_rm(self, rm)
    }
}

impl ToUsize for i64 {
    fn to_usize_res(self)
        -> ParseResultUsize {

        i64_to_usize_res(self)
    }

    fn to_usize(self)
        -> usize {

        i64_to_usize(self)
    }
}

impl ToUsize for f64 {

    fn to_usize_res(self)
        -> ParseResultUsize {

        f64_to_usize_res(self)
    }

    fn to_usize(self)
        -> usize {

        f64_to_usize(self)
    }
}

impl ToUsizeRM for f64 {

    fn to_usize_rm_res(self, rm: RoundingMode)
        -> ParseResultUsize {

        f64_to_usize_rm_res(self, rm)
    }

    fn to_usize_rm(self, rm: RoundingMode)
        -> usize {

        f64_to_usize_rm(self, rm)
    }
}

impl ToUsize for isize {

    fn to_usize_res(self)
        -> ParseResultUsize {

        isize_to_usize_res(self)
    }

    fn to_usize(self)
        -> usize {

        isize_to_usize(self)
    }
}

impl ToUsize for String {

    fn to_usize_res(self)
        -> ParseResultUsize {

        string_to_usize_res(self)
    }

    fn to_usize(self)
        -> usize {

        string_to_usize(self)
    }
}

impl ToUsize for &'static str {

    fn to_usize_res(self)
        -> ParseResultUsize {

        str_to_usize_res(self)
    }

    fn to_usize(self)
        -> usize {

        str_to_usize(self)
    }
}
