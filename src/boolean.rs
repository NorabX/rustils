pub fn to_string(b: bool, true_str: &str, false_str: &str) -> String {
    if b { return String::from(true_str); }
    String::from(false_str)
}

pub fn to_value<T>(b: bool, true_val: T, false_val: T) -> T {
    if b { return true_val; }
    false_val
}

pub fn is_true<T: PartialEq>(val: T, false_val: T) -> bool {
    if val == false_val { return false; }
    true
}

pub fn is_false<T: PartialEq>(val: T, false_val: T) -> bool {
    if val == false_val { return true; }
    false
}


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
