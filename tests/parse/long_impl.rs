extern crate rustils;

use rustils::parse::long::*;
use rustils::RoundingMode::*;
use std::f32;
use std::f64;

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
fn parse_f32_to_i64_0() {
    let a = -16777215_f32;
    let b = 16777215_f32;
    let c = 1.321_f32;
    let d = 1.546_f32;

    assert_eq!(-16777215_i64, a.to_i64());
    assert_eq!(16777215_i64, b.to_i64());
    assert_eq!(1_i64, c.to_i64_rm(Trunc));
    assert_eq!(2_i64, c.to_i64_rm(Ceil));
    assert_eq!(1_i64, c.to_i64_rm(Round));
    assert_eq!(1_i64, d.to_i64_rm(Trunc));
    assert_eq!(1_i64, d.to_i64_rm(Floor));
    assert_eq!(2_i64, d.to_i64_rm(Round));
}

#[test]
#[should_panic(expected = "Invalid Number: -16777216")]
fn parse_f32_to_i64_1() { (-16777216_f32).to_i64(); }

#[test]
#[should_panic(expected = "Invalid Number: 16777216")]
fn parse_f32_to_i64_2() { (16777216_f32).to_i64(); }

#[test]
#[should_panic(expected = "Invalid Number: NaN")]
fn parse_f32_to_i64_3() { (f32::NAN).to_i64(); }

#[test]
fn parse_u64_to_i64_0() {
    let a = 0_u64;
    let b = 9223372036854775807_u64;

    assert_eq!(0_i64, a.to_i64());
    assert_eq!(9223372036854775807_i64, b.to_i64());
}

#[test]
#[should_panic(expected = "Invalid Number: 9223372036854775808")]
fn parse_u64_to_i64_1() { (9223372036854775808_u64).to_i64(); }

#[test]
fn parse_f64_to_i64_0() {
    let a = -9007199254740991_f64;
    let b = 9007199254740991_f64;
    let c = 1.321_f64;
    let d = 1.546_f64;

    assert_eq!(-9007199254740991_i64, a.to_i64());
    assert_eq!(9007199254740991_i64, b.to_i64());
    assert_eq!(1_i64, c.to_i64_rm(Trunc));
    assert_eq!(2_i64, c.to_i64_rm(Ceil));
    assert_eq!(1_i64, c.to_i64_rm(Round));
    assert_eq!(1_i64, d.to_i64_rm(Trunc));
    assert_eq!(1_i64, d.to_i64_rm(Floor));
    assert_eq!(2_i64, d.to_i64_rm(Round));
}

#[test]
#[should_panic(expected = "Invalid Number: -9007199254740992")]
fn parse_f64_to_i64_1() { (-9007199254740992_f64).to_i64(); }

#[test]
#[should_panic(expected = "Invalid Number: 9007199254740992")]
fn parse_f64_to_i64_2() { (9007199254740992_f64).to_i64(); }

#[test]
#[should_panic(expected = "Invalid Number: NaN")]
fn parse_f64_to_i64_3() { (f64::NAN).to_i64(); }

#[test]
fn parse_usize_to_i64_0() {
    let a = 0_usize;
    let b = 9223372036854775807_usize;

    assert_eq!(0_i64, a.to_i64());
    assert_eq!(9223372036854775807_i64, b.to_i64());
}

#[test]
#[should_panic(expected = "Invalid Number: 9223372036854775808")]
fn parse_usize_to_i64_1() { (9223372036854775808_usize).to_i64(); }

#[test]
fn parse_string_to_i64_0() {
    let a = String::from("-9223372036854775808");
    let b = "-9223372036854775808";

    let c = String::from("9223372036854775807");
    let d = "9223372036854775807";

    assert_eq!(-9223372036854775808_i64, a.to_i64());
    assert_eq!(-9223372036854775808_i64, b.to_i64());

    assert_eq!(9223372036854775807_i64, c.to_i64());
    assert_eq!(9223372036854775807_i64, d.to_i64());
}

#[test]
#[should_panic(expected = "Invalid Number: -9223372036854775809")]
fn parse_string_to_i64_1() { String::from("-9223372036854775809").to_i64(); }

#[test]
#[should_panic(expected = "Invalid Number: 9223372036854775808")]
fn parse_string_to_i64_2() { ("9223372036854775808").to_i64(); }
