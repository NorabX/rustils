extern crate rustils;

use rustils::parse::usize::*;
use rustils::RoundingMode::*;
use std::f32;
use std::f64;

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
fn parse_i8_to_usize_0() {
    let a = 0_i8;
    let b = 127_i8;

    assert_eq!(0_usize, a.to_usize());
    assert_eq!(127_usize, b.to_usize());
}

#[test]
#[should_panic(expected = "Invalid Number: -1")]
fn parse_i8_to_usize_1() { (-1_i8).to_usize(); }

#[test]
fn parse_i16_to_usize_0() {
    let a = 0_i16;
    let b = 32767_i16;

    assert_eq!(0_usize, a.to_usize());
    assert_eq!(32767_usize, b.to_usize());
}

#[test]
#[should_panic(expected = "Invalid Number: -1")]
fn parse_i16_to_usize_1() { (-1_i32).to_usize(); }

#[test]
fn parse_i32_to_usize_0() {
    let a = 0_i32;
    let b = 2147483647_i32;

    assert_eq!(0_usize, a.to_usize());
    assert_eq!(2147483647_usize, b.to_usize());
}

#[test]
#[should_panic(expected = "Invalid Number: -1")]
fn parse_i32_to_usize_1() { (-1_i32).to_usize(); }


#[test]
fn parse_f32_to_usize_0() {
    let a = 0_f32;
    let b = 16777215_f32;
    let c = 1.321_f32;
    let d = 1.546_f32;

    assert_eq!(0_usize, a.to_usize());
    assert_eq!(16777215_usize, b.to_usize());
    assert_eq!(1_usize, c.to_usize_rm(Trunc));
    assert_eq!(2_usize, c.to_usize_rm(Ceil));
    assert_eq!(1_usize, c.to_usize_rm(Round));
    assert_eq!(1_usize, d.to_usize_rm(Trunc));
    assert_eq!(1_usize, d.to_usize_rm(Floor));
    assert_eq!(2_usize, d.to_usize_rm(Round));
}

#[test]
#[should_panic(expected = "Invalid Number: -1")]
fn parse_f32_to_usize_1() { (-1_f32).to_usize(); }

#[test]
#[should_panic(expected = "Invalid Number: 16777216")]
fn parse_f32_to_usize_2() { (16777216_f32).to_usize(); }

#[test]
#[should_panic(expected = "Invalid Number: NaN")]
fn parse_f32_to_usize_3() { (f32::NAN).to_usize(); }

#[test]
fn parse_i64_to_usize_0() {
    let a = 0_i64;
    let b = 9223372036854775807_i64;

    assert_eq!(0_usize, a.to_usize());
    assert_eq!(9223372036854775807_usize, b.to_usize());
}

#[test]
#[should_panic(expected = "Invalid Number: -1")]
fn parse_i64_to_usize_1() { (-1_i64).to_usize(); }

#[test]
fn parse_f64_to_usize_0() {
    let a = 0_f64;
    let b = 9007199254740991_f64;
    let c = 1.321_f64;
    let d = 1.546_f64;

    assert_eq!(0_usize, a.to_usize());
    assert_eq!(9007199254740991_usize, b.to_usize());
    assert_eq!(1_usize, c.to_usize_rm(Trunc));
    assert_eq!(2_usize, c.to_usize_rm(Ceil));
    assert_eq!(1_usize, c.to_usize_rm(Round));
    assert_eq!(1_usize, d.to_usize_rm(Trunc));
    assert_eq!(1_usize, d.to_usize_rm(Floor));
    assert_eq!(2_usize, d.to_usize_rm(Round));
}

#[test]
#[should_panic(expected = "Invalid Number: -1")]
fn parse_f64_to_usize_1() { (-1_f64).to_usize(); }

#[test]
#[should_panic(expected = "Invalid Number: 9007199254740992")]
fn parse_f64_to_usize_2() { (9007199254740992_f64).to_usize(); }

#[test]
#[should_panic(expected = "Invalid Number: NaN")]
fn parse_f64_to_usize_3() { (f64::NAN).to_usize(); }

#[test]
fn parse_isize_to_usize_0() {
    let a = 0_isize;
    let b = 9223372036854775807_isize;

    assert_eq!(0_usize, a.to_usize());
    assert_eq!(9223372036854775807_usize, b.to_usize());
}

#[test]
#[should_panic(expected = "Invalid Number: -1")]
fn parse_isize_to_usize_1() { (-1_isize).to_usize(); }

#[test]
fn parse_string_to_usize_0() {
    let a = String::from("0");
    let b = "0";

    let c = String::from("18446744073709551615");
    let d = "18446744073709551615";

    assert_eq!(0_usize, a.to_usize());
    assert_eq!(0_usize, b.to_usize());

    assert_eq!(18446744073709551615_usize, c.to_usize());
    assert_eq!(18446744073709551615_usize, d.to_usize());
}

#[test]
#[should_panic(expected = "Invalid Number: -1")]
fn parse_string_to_usize_1() { String::from("-1").to_usize(); }

#[test]
#[should_panic(expected = "Invalid Number: 18446744073709551616")]
fn parse_string_to_usize_2() { ("18446744073709551616").to_usize(); }
