extern crate rustils;

use rustils::parse::boolean::*;
use rustils::parse::byte::*;
use rustils::parse::int::*;
use rustils::parse::isize::*;
use rustils::parse::long::*;
use rustils::parse::short::*;
use rustils::parse::ubyte::*;
use rustils::parse::uint::*;
use rustils::parse::ushort::*;

#[test]
fn parse_u64_to_bool() {
    let a = 1_u64;
    let b = 0_u64;

    assert_eq!(true, a.to_bool());
    assert_eq!(false, b.to_bool());
}


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
fn parse_u64_to_isize_0() {
    let a = 0_u64;
    let b = 9223372036854775807_u64;

    assert_eq!(0_isize, a.to_isize());
    assert_eq!(9223372036854775807_isize, b.to_isize());
}

#[test]
#[should_panic(expected = "Invalid Number: 9223372036854775808")]
fn parse_u64_to_isize_1() { (9223372036854775808_u64).to_isize(); }


#[test]
fn parse_u64_to_u8_0() {
    let a = 0_u64;
    let b = 255_u64;

    assert_eq!(0_u8, a.to_u8());
    assert_eq!(255_u8, b.to_u8());
}

#[test]
#[should_panic(expected = "Invalid Number: 256")]
fn parse_u64_to_u8_1() { (256_u16).to_u8(); }


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
fn parse_u64_to_u32_0() {
    let a = 0_u64;
    let b = 4294967295_u64;

    assert_eq!(0_u32, a.to_u32());
    assert_eq!(4294967295_u32, b.to_u32());
}

#[test]
#[should_panic(expected = "Invalid Number: 4294967296")]
fn parse_u64_to_u32_1() { (4294967296_u64).to_u32(); }
