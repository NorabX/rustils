extern crate rustils;

use rustils::parse::*;

#[test]
fn parse_usize_to_bool() {
    let a = 1_usize;
    let b = 0_usize;

    assert_eq!(true, a.to_bool());
    assert_eq!(false, b.to_bool());
}


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
fn parse_usize_to_isize_0() {
    let a = 0_usize;
    let b = 9223372036854775807_usize;

    assert_eq!(0_isize, a.to_isize());
    assert_eq!(9223372036854775807_isize, b.to_isize());
}

#[test]
#[should_panic(expected = "Invalid Number: 9223372036854775808")]
fn parse_usize_to_isize_1() { (9223372036854775808_usize).to_isize(); }


#[test]
fn parse_usize_to_u8_0() {
    let a = 0_usize;
    let b = 255_usize;

    assert_eq!(0_u8, a.to_u8());
    assert_eq!(255_u8, b.to_u8());
}

#[test]
#[should_panic(expected = "Invalid Number: 256")]
fn parse_usize_to_u8_1() { (256_u16).to_u8(); }


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
fn parse_usize_to_u32_0() {
    let a = 0_usize;
    let b = 4294967295_usize;

    assert_eq!(0_u32, a.to_u32());
    assert_eq!(4294967295_u32, b.to_u32());
}

#[test]
#[should_panic(expected = "Invalid Number: 4294967296")]
fn parse_usize_to_u32_1() { (4294967296_usize).to_u32(); }
