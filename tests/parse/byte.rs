extern crate rustils;

use rustils::parse::byte;
use std::error::Error;

#[test]
fn parse_i8_to_bool() {
    assert_eq!(true,byte::to_bool(-1_i8));
    assert_eq!(true,byte::to_bool(1_i8));
    assert_eq!(false,byte::to_bool(0_i8));
}

#[test]
fn parse_i8_to_u8() {
    assert_eq!(0_u8,byte::to_u8(0_i8).unwrap());
    assert_eq!(127_u8,byte::to_u8(127_i8).unwrap());

    assert_eq!("Invalid Number",
        byte::to_u8(-1_i8).err().unwrap().description());
}

#[test]
fn parse_i8_to_u16() {
    assert_eq!(0_u16,byte::to_u16(0_i8).unwrap());
    assert_eq!(127_u16,byte::to_u16(127_i8).unwrap());

    assert_eq!("Invalid Number",
        byte::to_u8(-1_i8).err().unwrap().description());
}

#[test]
fn parse_i8_to_u32() {
    assert_eq!(0_u32,byte::to_u32(0_i8).unwrap());
    assert_eq!(127_u32,byte::to_u32(127_i8).unwrap());

    assert_eq!("Invalid Number",
        byte::to_u8(-1_i8).err().unwrap().description());
}

#[test]
fn parse_i8_to_u64() {
    assert_eq!(0_u64,byte::to_u64(0_i8).unwrap());
    assert_eq!(127_u64,byte::to_u64(127_i8).unwrap());

    assert_eq!("Invalid Number",
        byte::to_u8(-1_i8).err().unwrap().description());
}

#[test]
fn parse_i8_to_usize() {
    assert_eq!(0_usize,byte::to_usize(0_i8).unwrap());
    assert_eq!(127_usize,byte::to_usize(127_i8).unwrap());

    assert_eq!("Invalid Number",
        byte::to_u8(-1_i8).err().unwrap().description());
}

#[test]
#[should_panic(expected = "Invalid Number: -1")]
fn parse_i8_to_u8p() { byte::to_u8p(-1_i8); }

#[test]
#[should_panic(expected = "Invalid Number: -1")]
fn parse_i8_to_u16p() { byte::to_u16p(-1_i8); }

#[test]
#[should_panic(expected = "Invalid Number: -1")]
fn parse_i8_to_u32p() { byte::to_u32p(-1_i8); }

#[test]
#[should_panic(expected = "Invalid Number: -1")]
fn parse_i8_to_u64p() { byte::to_u64p(-1_i8); }

#[test]
#[should_panic]
fn parse_i8_to_usizep() { byte::to_usizep(-1_i8); }
