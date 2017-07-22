use std::mem::{ forget, transmute };
use parse::string::*;

impl ToStr for String {

    fn to_str(self)
        -> &'static str {

        unsafe {
            let ret = transmute(&self as &str);
            forget(self);
            ret
        }
    }
}
