extern crate rustils;

use rustils::parse::uint::*;
use rustils::RoundingMode::*;
use std::f32;
use std::f64;

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
fn parse_i8_to_u32_0() {
    let a = 0_i8;
    let b = 127_i8;

    assert_eq!(0_u32, a.to_u32());
    assert_eq!(127_u32, b.to_u32());
}

#[test]
#[should_panic(expected = "Invalid Number: -1")]
fn parse_i8_to_u32_1() { (-1_i8).to_u32(); }

#[test]
fn parse_i16_to_u32_0() {
    let a = 0_i16;
    let b = 32767_i16;

    assert_eq!(0_u32, a.to_u32());
    assert_eq!(32767_u32, b.to_u32());
}

#[test]
#[should_panic(expected = "Invalid Number: -1")]
fn parse_i16_to_u32_1() { (-1_i32).to_u32(); }


#[test]
fn parse_i32_to_u32_0() {
    let a = 0_i32;
    let b = 2147483647_i32;

    assert_eq!(0_u32, a.to_u32());
    assert_eq!(2147483647_u32, b.to_u32());
}

#[test]
#[should_panic(expected = "Invalid Number: -1")]
fn parse_i32_to_u32_1() { (-1_i32).to_u32(); }

#[test]
fn parse_f32_to_u32_0() {
    let a = 0_f32;
    let b = 16777215_f32;
    let c = 1.321_f32;
    let d = 1.546_f32;

    assert_eq!(0_u32, a.to_u32());
    assert_eq!(16777215_u32, b.to_u32());
    assert_eq!(1_u32, c.to_u32_rm(Trunc));
    assert_eq!(2_u32, c.to_u32_rm(Ceil));
    assert_eq!(1_u32, c.to_u32_rm(Round));
    assert_eq!(1_u32, d.to_u32_rm(Trunc));
    assert_eq!(1_u32, d.to_u32_rm(Floor));
    assert_eq!(2_u32, d.to_u32_rm(Round));
}

#[test]
#[should_panic(expected = "Invalid Number: -1")]
fn parse_f32_to_u32_1() { (-1_f32).to_u32(); }

#[test]
#[should_panic(expected = "Invalid Number: 16777216")]
fn parse_f32_to_u32_2() { (16777216_f32).to_u32(); }

#[test]
#[should_panic(expected = "Invalid Number: NaN")]
fn parse_f32_to_u32_3() { (f32::NAN).to_u32(); }

#[test]
fn parse_i64_to_u32_0() {
    let a = 0_i64;
    let b = 4294967295_i64;

    assert_eq!(0_u32, a.to_u32());
    assert_eq!(4294967295_u32, b.to_u32());
}

#[test]
#[should_panic(expected = "Invalid Number: -1")]
fn parse_i64_to_u32_1() { (-1_i64).to_u32(); }

#[test]
#[should_panic(expected = "Invalid Number: 4294967296")]
fn parse_i64_to_u32_2() { (4294967296_i64).to_u32(); }

#[test]
fn parse_u64_to_u32_0() {
    let a = 0_u64;
    let b = 4294967295_u64;

    assert_eq!(0_u32, a.to_u32());
    assert_eq!(4294967295_u32, b.to_u32());
}

#[test]
#[should_panic(expected = "Invalid Number: 4294967296")]
fn parse_u64_to_u32_1() { (4294967296_u64).to_u32(); }

#[test]
fn parse_f64_to_u32_0() {
    let a = 0_f64;
    let b = 4294967295_f64;
    let c = 1.321_f64;
    let d = 1.546_f64;

    assert_eq!(0_u32, a.to_u32());
    assert_eq!(4294967295_u32, b.to_u32());
    assert_eq!(1_u32, c.to_u32_rm(Trunc));
    assert_eq!(2_u32, c.to_u32_rm(Ceil));
    assert_eq!(1_u32, c.to_u32_rm(Round));
    assert_eq!(1_u32, d.to_u32_rm(Trunc));
    assert_eq!(1_u32, d.to_u32_rm(Floor));
    assert_eq!(2_u32, d.to_u32_rm(Round));
}

#[test]
#[should_panic(expected = "Invalid Number: -1")]
fn parse_f64_to_u32_1() { (-1_f64).to_u32(); }

#[test]
#[should_panic(expected = "Invalid Number: 4294967296")]
fn parse_f64_to_u32_2() { (4294967296_f64).to_u32(); }

#[test]
#[should_panic(expected = "Invalid Number: NaN")]
fn parse_f64_to_u32_3() { (f64::NAN).to_u32(); }

#[test]
fn parse_isize_to_u32_0() {
    let a = 0_isize;
    let b = 4294967295_isize;

    assert_eq!(0_u32, a.to_u32());
    assert_eq!(4294967295_u32, b.to_u32());
}

#[test]
#[should_panic(expected = "Invalid Number: -1")]
fn parse_isize_to_u32_1() { (-1_isize).to_u32(); }

#[test]
#[should_panic(expected = "Invalid Number: 4294967296")]
fn parse_isize_to_u32_2() { (4294967296_isize).to_u32(); }

#[test]
fn parse_usize_to_u32_0() {
    let a = 0_usize;
    let b = 4294967295_usize;

    assert_eq!(0_u32, a.to_u32());
    assert_eq!(4294967295_u32, b.to_u32());
}

#[test]
#[should_panic(expected = "Invalid Number: 4294967296")]
fn parse_usize_to_u32_1() { (4294967296_usize).to_u32(); }

#[test]
fn parse_string_to_u32_0() {
    let a = String::from("0");
    let b = "0";

    let c = String::from("4294967295");
    let d = "4294967295";

    assert_eq!(0_u32, a.to_u32());
    assert_eq!(0_u32, b.to_u32());

    assert_eq!(4294967295_u32, c.to_u32());
    assert_eq!(4294967295_u32, d.to_u32());
}

#[test]
#[should_panic(expected = "Invalid Number: -1")]
fn parse_string_to_u32_1() { String::from("-1").to_u32(); }

#[test]
#[should_panic(expected = "Invalid Number: 4294967296")]
fn parse_string_to_u32_2() { ("4294967296").to_u32(); }
