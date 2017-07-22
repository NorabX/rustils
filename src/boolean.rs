// <editor-fold> # Macros

#[macro_export]
macro_rules! and {
    ( $( $x:expr ),* ) => {
        {
            let mut and = true;
            $(
                and &= $x;
            )*
            and
        }
    };
}

#[macro_export]
macro_rules! or {
    ( $( $x:expr ),* ) => {
        {
            let mut or = false;
            $(
                or |= $x;
            )*
            or
        }
    };
}

#[macro_export]
macro_rules! xor {
    ( $( $x:expr ),* ) => {
        {
            let mut xor = false;
            $(
                xor ^= $x;
            )*
            xor
        }
    };
}
// </editor-fold>
