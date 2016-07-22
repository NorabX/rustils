extern crate rustils;

use rustils::parse::ubyte;

#[test]
fn parse_i8_to_bool(){
    assert_eq!(true,ubyte::tobool(42_u8));
    assert_eq!(true,ubyte::tobool(1_u8));
    assert_eq!(false,ubyte::tobool(0_u8));
}

#[test]
fn parse_u8_to_i8(){
    assert_eq!(0_i8,ubyte::toi8p(0_u8));
    assert_eq!(127_i8,ubyte::toi8p(127_u8));
}