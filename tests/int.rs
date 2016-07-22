extern crate rustils;

use rustils::parse::int;

#[test]
fn parse_i32_to_bool(){
    assert_eq!(true,int::tobool(-1_i32));
    assert_eq!(true,int::tobool(1_i32));
    assert_eq!(false,int::tobool(0_i32));
}

#[test]
fn parse_i32_to_i8(){
    assert_eq!(0_i8,int::toi8p(0_i32));
    assert_eq!(127_i8,int::toi8p(127_i32));
    assert_eq!(-128_i8,int::toi8p(-128_i32));
}

#[test]
fn parse_i32_to_i16(){
    assert_eq!(0_i16,int::toi16p(0_i32));
    assert_eq!(32767_i16,int::toi16p(32767_i32));
    assert_eq!(-32768_i16,int::toi16p(-32768_i32));
}

#[test]
fn parse_i32_to_u8(){
    assert_eq!(0_u8,int::tou8p(0_i32));
    assert_eq!(255_u8,int::tou8p(255_i32));
}

#[test]
fn parse_i32_to_u16(){
    assert_eq!(0_u16,int::tou16p(0_i32));
    assert_eq!(65535_u16,int::tou16p(65535_i32));
}

#[test]
fn parse_i32_to_u32(){
    assert_eq!(0_u32,int::tou32p(0_i32));
    assert_eq!(2147483647_u32,int::tou32p(2147483647_i32));
}

#[test]
fn parse_i32_to_u64(){
    assert_eq!(0_u64,int::tou64p(0_i32));
    assert_eq!(2147483647_u64,int::tou64p(2147483647_i32));
}
