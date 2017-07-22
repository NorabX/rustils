// <editor-fold> # Uses

use std::mem::{ forget, transmute };
use parse::string::*;
// </editor-fold>

// <editor-fold> # Impls

// <editor-fold> ## ToStr

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
// </editor-fold>

// </editor-fold>
