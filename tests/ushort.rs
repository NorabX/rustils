extern crate rustils;

use rustils::parse::ushort;

#[test]
fn parse_u16_to_bool(){
    assert_eq!(true,ushort::tobool(42_u16));
    assert_eq!(true,ushort::tobool(1_u16));
    assert_eq!(false,ushort::tobool(0_u16));
}

#[test]
fn parse_u16_to_i8(){
    assert_eq!(0_i8,ushort::toi8p(0_u16));
    assert_eq!(127_i8,ushort::toi8p(127_u16));
}

#[test]
fn parse_u16_to_i16(){
    assert_eq!(0_i16,ushort::toi16p(0_u16));
    assert_eq!(32767_i16,ushort::toi16p(32767_u16));
}

#[test]
fn parse_u16_to_u8(){
    assert_eq!(0_u8,ushort::tou8p(0_u16));
    assert_eq!(255_u8,ushort::tou8p(255_u16));
}
