extern crate rustils;

use rustils::parse::int::*;
use rustils::RoundingMode::*;
use std::f32;
use std::f64;

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
fn parse_u32_to_i32_0() {
    let a = 0_u32;
    let b = 2147483647_u32;

    assert_eq!(0_i32, a.to_i32());
    assert_eq!(2147483647_i32, b.to_i32());
}

#[test]
#[should_panic(expected = "Invalid Number: 2147483648")]
fn parse_u32_to_i32_1() { (2147483648_u32).to_i32(); }

#[test]
fn parse_f32_to_i32_0() {
    let a = -16777215_f32;
    let b = 16777215_f32;
    let c = 1.321_f32;
    let d = 1.546_f32;

    assert_eq!(-16777215_i32, a.to_i32());
    assert_eq!(16777215_i32, b.to_i32());
    assert_eq!(1_i32, c.to_i32_rm(Trunc));
    assert_eq!(2_i32, c.to_i32_rm(Ceil));
    assert_eq!(1_i32, c.to_i32_rm(Round));
    assert_eq!(1_i32, d.to_i32_rm(Trunc));
    assert_eq!(1_i32, d.to_i32_rm(Floor));
    assert_eq!(2_i32, d.to_i32_rm(Round));
}

#[test]
#[should_panic(expected = "Invalid Number: -16777216")]
fn parse_f32_to_i32_1() { (-16777216_f32).to_i32(); }

#[test]
#[should_panic(expected = "Invalid Number: 16777216")]
fn parse_f32_to_i32_2() { (16777216_f32).to_i32(); }

#[test]
#[should_panic(expected = "Invalid Number: NaN")]
fn parse_f32_to_i32_3() { (f32::NAN).to_i32(); }

#[test]
fn parse_i64_to_i32_0() {
    let a = -2147483648_i64;
    let b = 2147483647_i64;

    assert_eq!(-2147483648_i32, a.to_i32());
    assert_eq!(2147483647_i32, b.to_i32());
}

#[test]
#[should_panic(expected = "Invalid Number: -2147483649")]
fn parse_i64_to_i32_1() { (-2147483649_i64).to_i32(); }

#[test]
#[should_panic(expected = "Invalid Number: 2147483648")]
fn parse_i64_to_i32_2() { (2147483648_i64).to_i32(); }

#[test]
fn parse_u64_to_i32_0() {
    let a = 0_u64;
    let b = 2147483647_u64;

    assert_eq!(0_i32, a.to_i32());
    assert_eq!(2147483647_i32, b.to_i32());
}

#[test]
#[should_panic(expected = "Invalid Number: 2147483648")]
fn parse_u64_to_i32_1() { (2147483648_u64).to_i32(); }

#[test]
fn parse_f64_to_i32_0() {
    let a = -2147483648_f64;
    let b = 2147483647_f64;
    let c = 1.321_f64;
    let d = 1.546_f64;

    assert_eq!(-2147483648_i32, a.to_i32());
    assert_eq!(2147483647_i32, b.to_i32());
    assert_eq!(1_i32, c.to_i32_rm(Trunc));
    assert_eq!(2_i32, c.to_i32_rm(Ceil));
    assert_eq!(1_i32, c.to_i32_rm(Round));
    assert_eq!(1_i32, d.to_i32_rm(Trunc));
    assert_eq!(1_i32, d.to_i32_rm(Floor));
    assert_eq!(2_i32, d.to_i32_rm(Round));
}

#[test]
#[should_panic(expected = "Invalid Number: -2147483649")]
fn parse_f64_to_i32_1() { (-2147483649_f64).to_i32(); }

#[test]
#[should_panic(expected = "Invalid Number: 2147483648")]
fn parse_f64_to_i32_2() { (2147483648_f64).to_i32(); }

#[test]
#[should_panic(expected = "Invalid Number: NaN")]
fn parse_f64_to_i32_3() { (f64::NAN).to_i32(); }

#[test]
fn parse_isize_to_i32_0() {
    let a = -2147483648_isize;
    let b = 2147483647_isize;

    assert_eq!(-2147483648_i32, a.to_i32());
    assert_eq!(2147483647_i32, b.to_i32());
}

#[test]
#[should_panic(expected = "Invalid Number: -2147483649")]
fn parse_isize_to_i32_1() { (-2147483649_isize).to_i32(); }

#[test]
#[should_panic(expected = "Invalid Number: 2147483648")]
fn parse_isize_to_i32_2() { (2147483648_isize).to_i32(); }

#[test]
fn parse_usize_to_i32_0() {
    let a = 0_usize;
    let b = 2147483647_usize;

    assert_eq!(0_i32, a.to_i32());
    assert_eq!(2147483647_i32, b.to_i32());
}

#[test]
#[should_panic(expected = "Invalid Number: 2147483648")]
fn parse_usize_to_i32_1() { (2147483648_usize).to_i32(); }

#[test]
fn parse_string_to_i32_0() {
    let a = String::from("-2147483648");
    let b = "-2147483648";

    let c = String::from("2147483647");
    let d = "2147483647";

    assert_eq!(-2147483648_i32, a.to_i32());
    assert_eq!(-2147483648_i32, b.to_i32());

    assert_eq!(2147483647_i32, c.to_i32());
    assert_eq!(2147483647_i32, d.to_i32());
}

#[test]
#[should_panic(expected = "Invalid Number: -2147483649")]
fn parse_string_to_i32_1() { String::from("-2147483649").to_i32(); }

#[test]
#[should_panic(expected = "Invalid Number: 2147483648")]
fn parse_string_to_i32_2() { ("2147483648").to_i32(); }
