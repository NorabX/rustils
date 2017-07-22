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
fn parse_u32_to_bool() {
    let a = 1_u32;
    let b = 0_u32;

    assert_eq!(true, a.to_bool());
    assert_eq!(false, b.to_bool());
}


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
fn parse_u32_to_i16_0() {
    let a = 0_u32;
    let b = 32767_u32;

    assert_eq!(0_i16, a.to_i16());
    assert_eq!(32767_i16, b.to_i16());
}

#[test]
#[should_panic(expected = "Invalid Number: 32768")]
fn parse_u32_to_i16_1() { (32768_u16).to_i16(); }


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
fn parse_u32_to_u8_0() {
    let a = 0_u32;
    let b = 255_u32;

    assert_eq!(0_u8, a.to_u8());
    assert_eq!(255_u8, b.to_u8());
}

#[test]
#[should_panic(expected = "Invalid Number: 256")]
fn parse_u32_to_u8_1() { (256_u16).to_u8(); }


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
