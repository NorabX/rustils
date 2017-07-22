extern crate rustils;

use rustils::parse::*;
use rustils::parse::boolean::*;
use rustils::parse::byte::*;
use rustils::parse::double::*;
use rustils::parse::float::*;
use rustils::parse::int::*;
use rustils::parse::isize::*;
use rustils::parse::long::*;
use rustils::parse::short::*;
use rustils::parse::string::*;
use rustils::parse::ubyte::*;
use rustils::parse::uint::*;
use rustils::parse::ulong::*;
use rustils::parse::ushort::*;
use rustils::parse::usize::*;

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
fn parse_i32_to_u8_0() {
    let a = 0_i32;
    let b = 255_i32;

    assert_eq!(0_u8, a.to_u8());
    assert_eq!(255_u8, b.to_u8());
}

#[test]
#[should_panic(expected = "Invalid Number: -1")]
fn parse_i32_to_u8_1() { (-1_i32).to_u8(); }

#[test]
#[should_panic(expected = "Invalid Number: 256")]
fn parse_i32_to_u8_2() { (256_i32).to_u8(); }


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
fn parse_i32_to_u64_0() {
    let a = 0_i32;
    let b = 2147483647_i32;

    assert_eq!(0_u64, a.to_u64());
    assert_eq!(2147483647_u64, b.to_u64());
}

#[test]
#[should_panic(expected = "Invalid Number: -1")]
fn parse_i32_to_u64_1() { (-1_i32).to_u64(); }


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
