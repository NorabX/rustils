use parse::double::*;

impl ToF64 for bool {

    fn to_f64_res(self)
        -> ParseResultF64 {

        bool_to_f64_res(self)
    }

    fn to_f64(self)
        -> f64 {

        bool_to_f64(self)
    }
}

impl ToF64 for String {

    fn to_f64_res(self)
        -> ParseResultF64 {

        string_to_f64_res(self)
    }

    fn to_f64(self)
        -> f64 {

        string_to_f64(self)
    }
}

impl ToF64 for &'static str {

    fn to_f64_res(self)
        -> ParseResultF64 {

        str_to_f64_res(self)
    }

    fn to_f64(self)
        -> f64 {

        str_to_f64(self)
    }
}
