extern crate rustils;

use rustils::parse::byte::*;
use rustils::RoundingMode::*;
use std::f32;
use std::f64;

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
fn parse_u8_to_i8_0() {
    let a = 0_u8;
    let b = 127_u8;

    assert_eq!(0_i8, a.to_i8());
    assert_eq!(127_i8, b.to_i8());
}

#[test]
#[should_panic(expected = "Invalid Number: 128")]
fn parse_u8_to_i8_1() { (128_u8).to_i8(); }

#[test]
fn parse_i16_to_i8_0() {
    let a = -128_i16;
    let b = 127_i16;

    assert_eq!(-128_i8, a.to_i8());
    assert_eq!(127_i8, b.to_i8());
}

#[test]
#[should_panic(expected = "Invalid Number: -129")]
fn parse_i16_to_i8_1() { (-129_i32).to_i8(); }

#[test]
#[should_panic(expected = "Invalid Number: 128")]
fn parse_i16_to_i8_2() { (128_i32).to_i8(); }

#[test]
fn parse_u16_to_i8_0() {
    let a = 0_u16;
    let b = 127_u16;

    assert_eq!(0_i8, a.to_i8());
    assert_eq!(127_i8, b.to_i8());
}

#[test]
#[should_panic(expected = "Invalid Number: 128")]
fn parse_u16_to_i8_1() { (128_u16).to_i8(); }

#[test]
fn parse_i32_to_i8_0() {
    let a = -128_i32;
    let b = 127_i32;

    assert_eq!(-128_i8, a.to_i8());
    assert_eq!(127_i8, b.to_i8());
}

#[test]
#[should_panic(expected = "Invalid Number: -129")]
fn parse_i32_to_i8_1() { (-129_i32).to_i8(); }

#[test]
#[should_panic(expected = "Invalid Number: 128")]
fn parse_i32_to_i8_2() { (128_i32).to_i8(); }

#[test]
fn parse_u32_to_i8_0() {
    let a = 0_u32;
    let b = 127_u32;

    assert_eq!(0_i8, a.to_i8());
    assert_eq!(127_i8, b.to_i8());
}

#[test]
#[should_panic(expected = "Invalid Number: 128")]
fn parse_u32_to_i8_1() { (128_u16).to_i8(); }

#[test]
fn parse_f32_to_i8_0() {
    let a = -128_f32;
    let b = 127_f32;
    let c = 1.321_f32;
    let d = 1.546_f32;

    assert_eq!(-128_i8, a.to_i8());
    assert_eq!(127_i8, b.to_i8());
    assert_eq!(1_i8, c.to_i8_rm(Trunc));
    assert_eq!(2_i8, c.to_i8_rm(Ceil));
    assert_eq!(1_i8, c.to_i8_rm(Round));
    assert_eq!(1_i8, d.to_i8_rm(Trunc));
    assert_eq!(1_i8, d.to_i8_rm(Floor));
    assert_eq!(2_i8, d.to_i8_rm(Round));
}

#[test]
#[should_panic(expected = "Invalid Number: -129")]
fn parse_f32_to_i8_1() { (-129_f32).to_i8(); }

#[test]
#[should_panic(expected = "Invalid Number: 128")]
fn parse_f32_to_i8_2() { (128_f32).to_i8(); }

#[test]
#[should_panic(expected = "Invalid Number: NaN")]
fn parse_f32_to_i8_3() { (f32::NAN).to_i8(); }

#[test]
fn parse_i64_to_i8_0() {
    let a = -128_i64;
    let b = 127_i64;

    assert_eq!(-128_i8, a.to_i8());
    assert_eq!(127_i8, b.to_i8());
}

#[test]
#[should_panic(expected = "Invalid Number: -129")]
fn parse_i64_to_i8_1() { (-129_i64).to_i8(); }

#[test]
fn parse_u64_to_i8_0() {
    let a = 0_u64;
    let b = 127_u64;

    assert_eq!(0_i8, a.to_i8());
    assert_eq!(127_i8, b.to_i8());
}

#[test]
#[should_panic(expected = "Invalid Number: 128")]
fn parse_u64_to_i8_1() { (128_u16).to_i8(); }

#[test]
#[should_panic(expected = "Invalid Number: 128")]
fn parse_i64_to_i8_2() { (128_i64).to_i8(); }

#[test]
fn parse_f64_to_i8_0() {
    let a = -128_f64;
    let b = 127_f64;
    let c = 1.321_f64;
    let d = 1.546_f64;

    assert_eq!(-128_i8, a.to_i8());
    assert_eq!(127_i8, b.to_i8());
    assert_eq!(1_i8, c.to_i8_rm(Trunc));
    assert_eq!(2_i8, c.to_i8_rm(Ceil));
    assert_eq!(1_i8, c.to_i8_rm(Round));
    assert_eq!(1_i8, d.to_i8_rm(Trunc));
    assert_eq!(1_i8, d.to_i8_rm(Floor));
    assert_eq!(2_i8, d.to_i8_rm(Round));
}

#[test]
#[should_panic(expected = "Invalid Number: -129")]
fn parse_f64_to_i8_1() { (-129_f64).to_i8(); }

#[test]
#[should_panic(expected = "Invalid Number: 128")]
fn parse_f64_to_i8_2() { (128_f64).to_i8(); }

#[test]
#[should_panic(expected = "Invalid Number: NaN")]
fn parse_f64_to_i8_3() { (f64::NAN).to_i8(); }

#[test]
fn parse_isize_to_i8_0() {
    let a = -128_isize;
    let b = 127_isize;

    assert_eq!(-128_i8, a.to_i8());
    assert_eq!(127_i8, b.to_i8());
}

#[test]
#[should_panic(expected = "Invalid Number: -129")]
fn parse_isize_to_i8_1() { (-129_isize).to_i8(); }

#[test]
#[should_panic(expected = "Invalid Number: 128")]
fn parse_isize_to_i8_2() { (128_isize).to_i8(); }

#[test]
fn parse_usize_to_i8_0() {
    let a = 0_usize;
    let b = 127_usize;

    assert_eq!(0_i8, a.to_i8());
    assert_eq!(127_i8, b.to_i8());
}

#[test]
#[should_panic(expected = "Invalid Number: 128")]
fn parse_usize_to_i8_1() { (128_u16).to_i8(); }

#[test]
fn parse_string_to_i8_0() {
    let a = String::from("-128");
    let b = "-128";

    let c = String::from("127");
    let d = "127";

    assert_eq!(-128_i8, a.to_i8());
    assert_eq!(-128_i8, b.to_i8());

    assert_eq!(127_i8, c.to_i8());
    assert_eq!(127_i8, d.to_i8());
}

#[test]
#[should_panic(expected = "Invalid Number: -129")]
fn parse_string_to_i8_1() { String::from("-129").to_i8(); }

#[test]
#[should_panic(expected = "Invalid Number: 128")]
fn parse_string_to_i8_2() { ("128").to_i8(); }
