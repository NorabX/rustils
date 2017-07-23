extern crate rustils;

use rustils::parse::ushort::*;
use rustils::RoundingMode::*;
use std::f32;
use std::f64;

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
fn parse_i8_to_u16_0() {
    let a = 0_i8;
    let b = 127_i8;

    assert_eq!(0_u16, a.to_u16());
    assert_eq!(127_u16, b.to_u16());
}

#[test]
#[should_panic(expected = "Invalid Number: -1")]
fn parse_i8_to_u16_1() { (-1_i8).to_u16(); }

#[test]
fn parse_i16_to_u16_0() {
    let a = 0_i16;
    let b = 32767_i16;

    assert_eq!(0_u16, a.to_u16());
    assert_eq!(32767_u16, b.to_u16());
}

#[test]
#[should_panic(expected = "Invalid Number: -1")]
fn parse_i16_to_u16_1() { (-1_i32).to_u16(); }

#[test]
fn parse_i32_to_u16_0() {
    let a = 0_i32;
    let b = 65535_i32;

    assert_eq!(0_u16, a.to_u16());
    assert_eq!(65535_u16, b.to_u16());
}

#[test]
#[should_panic(expected = "Invalid Number: -1")]
fn parse_i32_to_u16_1() { (-1_i32).to_u16(); }

#[test]
#[should_panic(expected = "Invalid Number: 65536")]
fn parse_i32_to_u16_2() { (65536_i32).to_u16(); }

#[test]
fn parse_u32_to_u16_0() {
    let a = 0_u32;
    let b = 65535_u32;

    assert_eq!(0_u16, a.to_u16());
    assert_eq!(65535_u16, b.to_u16());
}

#[test]
#[should_panic(expected = "Invalid Number: 65536")]
fn parse_u32_to_u16_1() { (65536_u32).to_u16(); }

#[test]
fn parse_f32_to_u16_0() {
    let a = 0_f32;
    let b = 65535_f32;
    let c = 1.321_f32;
    let d = 1.546_f32;

    assert_eq!(0_u16, a.to_u16());
    assert_eq!(65535_u16, b.to_u16());
    assert_eq!(1_u16, c.to_u16_rm(Trunc));
    assert_eq!(2_u16, c.to_u16_rm(Ceil));
    assert_eq!(1_u16, c.to_u16_rm(Round));
    assert_eq!(1_u16, d.to_u16_rm(Trunc));
    assert_eq!(1_u16, d.to_u16_rm(Floor));
    assert_eq!(2_u16, d.to_u16_rm(Round));
}

#[test]
#[should_panic(expected = "Invalid Number: -1")]
fn parse_f32_to_u16_1() { (-1_f32).to_u16(); }

#[test]
#[should_panic(expected = "Invalid Number: 65536")]
fn parse_f32_to_u16_2() { (65536_f32).to_u16(); }

#[test]
#[should_panic(expected = "Invalid Number: NaN")]
fn parse_f32_to_u16_3() { (f32::NAN).to_u16(); }

#[test]
fn parse_i64_to_u16_0() {
    let a = 0_i64;
    let b = 65535_i64;

    assert_eq!(0_u16, a.to_u16());
    assert_eq!(65535_u16, b.to_u16());
}

#[test]
#[should_panic(expected = "Invalid Number: -1")]
fn parse_i64_to_u16_1() { (-1_i64).to_u16(); }

#[test]
#[should_panic(expected = "Invalid Number: 65536")]
fn parse_i64_to_u16_2() { (65536_i64).to_u16(); }

#[test]
fn parse_u64_to_u16_0() {
    let a = 0_u64;
    let b = 65535_u64;

    assert_eq!(0_u16, a.to_u16());
    assert_eq!(65535_u16, b.to_u16());
}

#[test]
#[should_panic(expected = "Invalid Number: 65536")]
fn parse_u64_to_u16_1() { (65536_u64).to_u16(); }

#[test]
fn parse_f64_to_u16_0() {
    let a = 0_f64;
    let b = 65535_f64;
    let c = 1.321_f64;
    let d = 1.546_f64;

    assert_eq!(0_u16, a.to_u16());
    assert_eq!(65535_u16, b.to_u16());
    assert_eq!(1_u16, c.to_u16_rm(Trunc));
    assert_eq!(2_u16, c.to_u16_rm(Ceil));
    assert_eq!(1_u16, c.to_u16_rm(Round));
    assert_eq!(1_u16, d.to_u16_rm(Trunc));
    assert_eq!(1_u16, d.to_u16_rm(Floor));
    assert_eq!(2_u16, d.to_u16_rm(Round));
}

#[test]
#[should_panic(expected = "Invalid Number: -1")]
fn parse_f64_to_u16_1() { (-1_f64).to_u16(); }

#[test]
#[should_panic(expected = "Invalid Number: 65536")]
fn parse_f64_to_u16_2() { (65536_f64).to_u16(); }

#[test]
#[should_panic(expected = "Invalid Number: NaN")]
fn parse_f64_to_u16_3() { (f64::NAN).to_u16(); }

#[test]
fn parse_isize_to_u16_0() {
    let a = 0_isize;
    let b = 65535_isize;

    assert_eq!(0_u16, a.to_u16());
    assert_eq!(65535_u16, b.to_u16());
}

#[test]
#[should_panic(expected = "Invalid Number: -1")]
fn parse_isize_to_u16_1() { (-1_isize).to_u16(); }

#[test]
#[should_panic(expected = "Invalid Number: 65536")]
fn parse_isize_to_u16_2() { (65536_isize).to_u16(); }

#[test]
fn parse_usize_to_u16_0() {
    let a = 0_usize;
    let b = 65535_usize;

    assert_eq!(0_u16, a.to_u16());
    assert_eq!(65535_u16, b.to_u16());
}

#[test]
#[should_panic(expected = "Invalid Number: 65536")]
fn parse_usize_to_u16_1() { (65536_usize).to_u16(); }

#[test]
fn parse_string_to_u16_0() {
    let a = String::from("0");
    let b = "0";

    let c = String::from("65535");
    let d = "65535";

    assert_eq!(0_u16, a.to_u16());
    assert_eq!(0_u16, b.to_u16());

    assert_eq!(65535_u16, c.to_u16());
    assert_eq!(65535_u16, d.to_u16());
}

#[test]
#[should_panic(expected = "Invalid Number: -1")]
fn parse_string_to_u16_1() { String::from("-1").to_u16(); }

#[test]
#[should_panic(expected = "Invalid Number: 65536")]
fn parse_string_to_u16_2() { ("65536").to_u16(); }
