extern crate rustils;

use rustils::parse::*;

#[test]
fn parse_u16_to_bool() {
    let a = 1_u16;
    let b = 0_u16;

    assert_eq!(true, a.to_bool());
    assert_eq!(false, b.to_bool());
}


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
fn parse_u16_to_i16_0() {
    let a = 0_u16;
    let b = 32767_u16;

    assert_eq!(0_i16, a.to_i16());
    assert_eq!(32767_i16, b.to_i16());
}

#[test]
#[should_panic(expected = "Invalid Number: 32768")]
fn parse_u16_to_i16_1() { (32768_u16).to_i16(); }


#[test]
fn parse_u16_to_u8_0() {
    let a = 0_u16;
    let b = 255_u16;

    assert_eq!(0_u8, a.to_u8());
    assert_eq!(255_u8, b.to_u8());
}

#[test]
#[should_panic(expected = "Invalid Number: 256")]
fn parse_u16_to_u8_1() { (256_u16).to_u8(); }
