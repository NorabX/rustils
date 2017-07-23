extern crate rustils;

use rustils::parse::double::*;

#[test]
pub fn parse_bool_to_f64() {
    let t = true;
    let f = false;

    assert_eq!(Ok(1.0), t.to_f64_res());
    assert_eq!(Ok(0.0), f.to_f64_res());

    assert_eq!(1.0, t.to_f64());
    assert_eq!(0.0, f.to_f64());
}
