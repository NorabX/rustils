// <editor-fold> # Uses

use parse::boolean::*;
// </editor-fold>

// <editor-fold> # Impls

// <editor-fold> ## ToBool

impl ToBool for i8 {

    fn to_bool(self)
        -> bool {

        i8_to_bool(self)
    }
}

impl ToBool for u8 {
    fn to_bool(self)
        -> bool {

        u8_to_bool(self)
    }
}

impl ToBool for i16 {

    fn to_bool(self)
        -> bool {

        i16_to_bool(self)
    }
}

impl ToBool for u16 {
    fn to_bool(self)
        -> bool {

        u16_to_bool(self)
    }
}

impl ToBool for i32 {

    fn to_bool(self)
        -> bool {

        i32_to_bool(self)
    }
}

impl ToBool for u32 {
    fn to_bool(self)
        -> bool {

        u32_to_bool(self)
    }
}

impl ToBool for f32 {

    fn to_bool(self)
        -> bool {

        f32_to_bool(self)
    }
}

impl ToBool for i64 {

    fn to_bool(self)
        -> bool {

        i64_to_bool(self)
    }
}

impl ToBool for u64 {
    fn to_bool(self)
        -> bool {

        u64_to_bool(self)
    }
}

impl ToBool for f64 {

    fn to_bool(self)
        -> bool {

        f64_to_bool(self)
    }
}

impl ToBool for isize {

    fn to_bool(self)
        -> bool {

        isize_to_bool(self)
    }
}

impl ToBool for usize {

    fn to_bool(self)
        -> bool {

        usize_to_bool(self)
    }
}

impl ToBool for String {

    fn to_bool(self)
        -> bool {

        string_to_bool(self)
    }
}

impl ToBool for &'static str {

    fn to_bool(self)
        -> bool {

        str_to_bool(self)
    }
}
// </editor-fold>

// </editor-fold>
