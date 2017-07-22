// <editor-fold> # Uses

use parse::float::*;
// </editor-fold>

// <editor-fold> # Impls

// <editor-fold> ## ToF32

impl ToF32 for bool {

    fn to_f32_res(self)
        -> ParseResultF32 {

        bool_to_f32_res(self)
    }

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
// </editor-fold>

// </editor-fold>
