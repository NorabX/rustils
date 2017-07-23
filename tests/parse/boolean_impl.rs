extern crate rustils;

use rustils::parse::boolean::*;
use std::f32;
use std::f64;

#[test]
fn parse_i8_to_bool() {
    let a = -1_i8;
    let b = 1_i8;
    let c = 0_i8;

    assert_eq!(true, a.to_bool());
    assert_eq!(true, b.to_bool());
    assert_eq!(false, c.to_bool());
}

#[test]
fn parse_u8_to_bool() {
    let a = 1_u8;
    let b = 0_u8;

    assert_eq!(true, a.to_bool());
    assert_eq!(false, b.to_bool());
}

#[test]
fn parse_i16_to_bool() {
    let a = -1_i16;
    let b = 1_i8;
    let c = 0_i16;

    assert_eq!(true, a.to_bool());
    assert_eq!(true, b.to_bool());
    assert_eq!(false, c.to_bool());
}

#[test]
fn parse_u16_to_bool() {
    let a = 1_u16;
    let b = 0_u16;

    assert_eq!(true, a.to_bool());
    assert_eq!(false, b.to_bool());
}

#[test]
fn parse_i32_to_bool() {
    let a = -1_i32;
    let b = 1_i8;
    let c = 0_i32;

    assert_eq!(true, a.to_bool());
    assert_eq!(true, b.to_bool());
    assert_eq!(false, c.to_bool());
}

#[test]
fn parse_u32_to_bool() {
    let a = 1_u32;
    let b = 0_u32;

    assert_eq!(true, a.to_bool());
    assert_eq!(false, b.to_bool());
}

#[test]
fn parse_f32_to_bool() {
    let a = -1_i8;
    let b = 1_i8;
    let c = 0_i8;
    let d = 0.21_f32;
    let e = f32::NAN;

    assert_eq!(true, a.to_bool());
    assert_eq!(true, b.to_bool());
    assert_eq!(false, c.to_bool());
    assert_eq!(true, d.to_bool());
    assert_eq!(false, e.to_bool());
}

#[test]
fn parse_i64_to_bool() {
    let a = -1_i64;
    let b = 1_i8;
    let c = 0_i64;

    assert_eq!(true, a.to_bool());
    assert_eq!(true, b.to_bool());
    assert_eq!(false, c.to_bool());
}

#[test]
fn parse_u64_to_bool() {
    let a = 1_u64;
    let b = 0_u64;

    assert_eq!(true, a.to_bool());
    assert_eq!(false, b.to_bool());
}

#[test]
fn parse_f64_to_bool() {
    let a = -1_i8;
    let b = 1_i8;
    let c = 0_i8;
    let d = 0.21_f64;
    let e = f64::NAN;

    assert_eq!(true, a.to_bool());
    assert_eq!(true, b.to_bool());
    assert_eq!(false, c.to_bool());
    assert_eq!(true, d.to_bool());
    assert_eq!(false, e.to_bool());
}

#[test]
fn parse_isize_to_bool() {
    let a = -1_isize;
    let b = 1_i8;
    let c = 0_isize;

    assert_eq!(true, a.to_bool());
    assert_eq!(true, b.to_bool());
    assert_eq!(false, c.to_bool());
}

#[test]
fn parse_usize_to_bool() {
    let a = 1_usize;
    let b = 0_usize;

    assert_eq!(true, a.to_bool());
    assert_eq!(false, b.to_bool());
}

#[test]
fn parse_string_to_bool_0() {
    let a = String::from("TRue");
    let b = "FaLsE";
    let c = "0";

    assert_eq!(true, a.to_bool());
    assert_eq!(false, b.to_bool());
    assert_eq!(false, c.to_bool());
}

#[test]
#[should_panic(expected = "Invalid String: Hello World")]
fn parse_string_to_bool_1() { String::from("Hello World").to_bool(); }
