extern crate rustils;

use rustils::parse::short;

#[test]
fn parse_i16_to_bool(){
    assert_eq!(true,short::tobool(-1_i16));
    assert_eq!(true,short::tobool(1_i16));
    assert_eq!(false,short::tobool(0_i16));
}

#[test]
fn parse_i16_to_i8(){
    assert_eq!(0_i8,short::toi8p(0_i16));
    assert_eq!(127_i8,short::toi8p(127_i16));
    assert_eq!(-128_i8,short::toi8p(-128_i16));
}

#[test]
fn parse_i16_to_u8(){
    assert_eq!(0_u8,short::tou8p(0_i16));
    assert_eq!(255_u8,short::tou8p(255_i16));
}

#[test]
fn parse_i16_to_u16(){
    assert_eq!(0_u16,short::tou16p(0_i16));
    assert_eq!(32767_u16,short::tou16p(32767_i16));
}

#[test]
fn parse_i16_to_u32(){
    assert_eq!(0_u32,short::tou32p(0_i16));
    assert_eq!(32767_u32,short::tou32p(32767_i16));
}

#[test]
fn parse_i16_to_u64(){
    assert_eq!(0_u64,short::tou64p(0_i16));
    assert_eq!(32767_u64,short::tou64p(32767_i16));
}
