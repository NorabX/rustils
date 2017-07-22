// <editor-fold> # Uses

use parse::isize::*;
use RoundingMode;
// </editor-fold>

// <editor-fold> # Impl

// <editor-fold> ## ToIsize

impl ToIsize for bool {

    fn to_isize_res(self)
        -> ParseResultIsize {

        bool_to_isize_res(self)
    }

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
// </editor-fold>

// </editor-fold>
