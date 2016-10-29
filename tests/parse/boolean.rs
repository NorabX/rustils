extern crate rustils;
use rustils::parse::*;

#[test]
pub fn parse_bool_to_i8() {
    let t = true;
    let f = false;

    assert_eq!(Ok(1), t.to_i8_res());
    assert_eq!(Ok(0), f.to_i8_res());

    assert_eq!(1, t.to_i8());
    assert_eq!(0, f.to_i8());
}

#[test]
pub fn parse_bool_to_i16() {
    let t = true;
    let f = false;

    assert_eq!(Ok(1), t.to_i16_res());
    assert_eq!(Ok(0), f.to_i16_res());

    assert_eq!(1, t.to_i16());
    assert_eq!(0, f.to_i16());
}

#[test]
pub fn parse_bool_to_i32() {
    let t = true;
    let f = false;

    assert_eq!(Ok(1), t.to_i32_res());
    assert_eq!(Ok(0), f.to_i32_res());

    assert_eq!(1, t.to_i32());
    assert_eq!(0, f.to_i32());
}

#[test]
pub fn parse_bool_to_i64() {
    let t = true;
    let f = false;

    assert_eq!(Ok(1), t.to_i64_res());
    assert_eq!(Ok(0), f.to_i64_res());

    assert_eq!(1, t.to_i64());
    assert_eq!(0, f.to_i64());
}

#[test]
pub fn parse_bool_to_isize() {
    let t = true;
    let f = false;

    assert_eq!(Ok(1), t.to_isize_res());
    assert_eq!(Ok(0), f.to_isize_res());

    assert_eq!(1, t.to_isize());
    assert_eq!(0, f.to_isize());
}

#[test]
pub fn parse_bool_to_u8() {
    let t = true;
    let f = false;

    assert_eq!(Ok(1), t.to_u8_res());
    assert_eq!(Ok(0), f.to_u8_res());

    assert_eq!(1, t.to_u8());
    assert_eq!(0, f.to_u8());
}

#[test]
pub fn parse_bool_to_u16() {
    let t = true;
    let f = false;

    assert_eq!(Ok(1), t.to_u16_res());
    assert_eq!(Ok(0), f.to_u16_res());

    assert_eq!(1, t.to_u16());
    assert_eq!(0, f.to_u16());
}

#[test]
pub fn parse_bool_to_u32() {
    let t = true;
    let f = false;

    assert_eq!(Ok(1), t.to_u32_res());
    assert_eq!(Ok(0), f.to_u32_res());

    assert_eq!(1, t.to_u32());
    assert_eq!(0, f.to_u32());
}

#[test]
pub fn parse_bool_to_u64() {
    let t = true;
    let f = false;

    assert_eq!(Ok(1), t.to_u64_res());
    assert_eq!(Ok(0), f.to_u64_res());

    assert_eq!(1, t.to_u64());
    assert_eq!(0, f.to_u64());
}

#[test]
pub fn parse_bool_to_usize() {
    let t = true;
    let f = false;

    assert_eq!(Ok(1), t.to_usize_res());
    assert_eq!(Ok(0), f.to_usize_res());

    assert_eq!(1, t.to_usize());
    assert_eq!(0, f.to_usize());
}

#[test]
pub fn parse_bool_to_f32() {
    let t = true;
    let f = false;

    assert_eq!(Ok(1.0), t.to_f32_res());
    assert_eq!(Ok(0.0), f.to_f32_res());

    assert_eq!(1.0, t.to_f32());
    assert_eq!(0.0, f.to_f32());
}

#[test]
pub fn parse_bool_to_f64() {
    let t = true;
    let f = false;

    assert_eq!(Ok(1.0), t.to_f64_res());
    assert_eq!(Ok(0.0), f.to_f64_res());

    assert_eq!(1.0, t.to_f64());
    assert_eq!(0.0, f.to_f64());
}
