extern crate rustils;

use rustils::parse::short::*;
use rustils::RoundingMode::*;
use std::f32;
use std::f64;

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
fn parse_i32_to_i16_0() {
    let a = -32768_i32;
    let b = 32767_i32;

    assert_eq!(-32768_i16, a.to_i16());
    assert_eq!(32767_i16, b.to_i16());
}

#[test]
#[should_panic(expected = "Invalid Number: -32769")]
fn parse_i32_to_i16_1() { (-32769_i32).to_i16(); }

#[test]
#[should_panic(expected = "Invalid Number: 32768")]
fn parse_i32_to_i16_2() { (32768_i32).to_i16(); }

#[test]
fn parse_u16_to_i16_0() {
    let a = 0_u16;
    let b = 32767_u16;

    assert_eq!(0_i16, a.to_i16());
    assert_eq!(32767_i16, b.to_i16());
}

#[test]
#[should_panic(expected = "Invalid Number: 32768")]
fn parse_u16_to_i16_1() { (32768_u16).to_i16(); }

#[test]
fn parse_u32_to_i16_0() {
    let a = 0_u32;
    let b = 32767_u32;

    assert_eq!(0_i16, a.to_i16());
    assert_eq!(32767_i16, b.to_i16());
}

#[test]
#[should_panic(expected = "Invalid Number: 32768")]
fn parse_u32_to_i16_1() { (32768_u16).to_i16(); }

#[test]
fn parse_f32_to_i16_0() {
    let a = -32768_f32;
    let b = 32767_f32;
    let c = 1.321_f32;
    let d = 1.546_f32;

    assert_eq!(-32768_i16, a.to_i16());
    assert_eq!(32767_i16, b.to_i16());
    assert_eq!(1_i16, c.to_i16_rm(Trunc));
    assert_eq!(2_i16, c.to_i16_rm(Ceil));
    assert_eq!(1_i16, c.to_i16_rm(Round));
    assert_eq!(1_i16, d.to_i16_rm(Trunc));
    assert_eq!(1_i16, d.to_i16_rm(Floor));
    assert_eq!(2_i16, d.to_i16_rm(Round));
}

#[test]
#[should_panic(expected = "Invalid Number: -32769")]
fn parse_f32_to_i16_1() { (-32769_f32).to_i16(); }

#[test]
#[should_panic(expected = "Invalid Number: 32768")]
fn parse_f32_to_i16_2() { (32768_f32).to_i16(); }

#[test]
#[should_panic(expected = "Invalid Number: NaN")]
fn parse_f32_to_i16_3() { (f32::NAN).to_i16(); }

#[test]
fn parse_i64_to_i16_0() {
    let a = -32768_i64;
    let b = 32767_i64;

    assert_eq!(-32768_i16, a.to_i16());
    assert_eq!(32767_i16, b.to_i16());
}

#[test]
#[should_panic(expected = "Invalid Number: -32769")]
fn parse_i64_to_i16_1() { (-32769_i64).to_i16(); }

#[test]
#[should_panic(expected = "Invalid Number: 32768")]
fn parse_i64_to_i16_2() { (32768_i64).to_i16(); }

#[test]
fn parse_u64_to_i16_0() {
    let a = 0_u64;
    let b = 32767_u64;

    assert_eq!(0_i16, a.to_i16());
    assert_eq!(32767_i16, b.to_i16());
}

#[test]
#[should_panic(expected = "Invalid Number: 32768")]
fn parse_u64_to_i16_1() { (32768_u16).to_i16(); }

#[test]
fn parse_f64_to_i16_0() {
    let a = -32768_f64;
    let b = 32767_f64;
    let c = 1.321_f64;
    let d = 1.546_f64;

    assert_eq!(-32768_i16, a.to_i16());
    assert_eq!(32767_i16, b.to_i16());
    assert_eq!(1_i16, c.to_i16_rm(Trunc));
    assert_eq!(2_i16, c.to_i16_rm(Ceil));
    assert_eq!(1_i16, c.to_i16_rm(Round));
    assert_eq!(1_i16, d.to_i16_rm(Trunc));
    assert_eq!(1_i16, d.to_i16_rm(Floor));
    assert_eq!(2_i16, d.to_i16_rm(Round));
}

#[test]
#[should_panic(expected = "Invalid Number: -32769")]
fn parse_f64_to_i16_1() { (-32769_f64).to_i16(); }

#[test]
#[should_panic(expected = "Invalid Number: 32768")]
fn parse_f64_to_i16_2() { (32768_f64).to_i16(); }

#[test]
#[should_panic(expected = "Invalid Number: NaN")]
fn parse_f64_to_i16_3() { (f64::NAN).to_i16(); }

#[test]
fn parse_isize_to_i16_0() {
    let a = -32768_isize;
    let b = 32767_isize;

    assert_eq!(-32768_i16, a.to_i16());
    assert_eq!(32767_i16, b.to_i16());
}

#[test]
#[should_panic(expected = "Invalid Number: -32769")]
fn parse_isize_to_i16_1() { (-32769_isize).to_i16(); }

#[test]
#[should_panic(expected = "Invalid Number: 32768")]
fn parse_isize_to_i16_2() { (32768_isize).to_i16(); }

#[test]
fn parse_usize_to_i16_0() {
    let a = 0_usize;
    let b = 32767_usize;

    assert_eq!(0_i16, a.to_i16());
    assert_eq!(32767_i16, b.to_i16());
}

#[test]
#[should_panic(expected = "Invalid Number: 32768")]
fn parse_usize_to_i16_1() { (32768_u16).to_i16(); }

#[test]
fn parse_string_to_i16_0() {
    let a = String::from("-32768");
    let b = "-32768";

    let c = String::from("32767");
    let d = "32767";

    assert_eq!(-32768_i16, a.to_i16());
    assert_eq!(-32768_i16, b.to_i16());

    assert_eq!(32767_i16, c.to_i16());
    assert_eq!(32767_i16, d.to_i16());
}

#[test]
#[should_panic(expected = "Invalid Number: -32769")]
fn parse_string_to_i16_1() { String::from("-32769").to_i16(); }

#[test]
#[should_panic(expected = "Invalid Number: 32768")]
fn parse_string_to_i16_2() { ("32768").to_i16(); }
