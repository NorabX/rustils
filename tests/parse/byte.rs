extern crate rustils;
use rustils::parse::*;
use std::error::Error;

#[test]
fn parse_i8_to_bool() {
    let a = -1_i8;
    let b = 1_i8;
    let c = 0_i8;

    assert_eq!(true,a.to_bool());
    assert_eq!(true,b.to_bool());
    assert_eq!(false,c.to_bool());
}

#[test]
fn parse_i8_to_u8() {
    let a = 0_i8;
    let b = 127_i8;
    let c = -1_i8;

    assert_eq!(Ok(0_u8),a.to_u8_res());
    assert_eq!(Ok(127_u8),b.to_u8_res());

    assert_eq!(0_u8,a.to_u8());
    assert_eq!(127_u8,b.to_u8());

    assert_eq!("Invalid Number", c.to_u8_res().err().unwrap().description());

}


#[test]
fn parse_i8_to_u16() {
    let a = 0_i8;
    let b = 127_i8;
    let c = -1_i8;

    assert_eq!(Ok(0_u16),a.to_u16_res());
    assert_eq!(Ok(127_u16),b.to_u16_res());

    assert_eq!(0_u16,a.to_u16());
    assert_eq!(127_u16,b.to_u16());

    assert_eq!("Invalid Number", c.to_u16_res().err().unwrap().description());

}
