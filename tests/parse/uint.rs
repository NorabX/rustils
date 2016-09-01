extern crate rustils;

use rustils::parse::uint;

#[test]
fn parse_u32_to_bool(){
    assert_eq!(true,uint::to_bool(42_u32));
    assert_eq!(true,uint::to_bool(1_u32));
    assert_eq!(false,uint::to_bool(0_u32));
}

#[test]
fn parse_u32_to_i8(){
    assert_eq!(0_i8,uint::toi8p(0_u32));
    assert_eq!(127_i8,uint::toi8p(127_u32));
}

#[test]
fn parse_u32_to_i16(){
    assert_eq!(0_i16,uint::toi16p(0_u32));
    assert_eq!(32767_i16,uint::toi16p(32767_u32));
}

#[test]
fn parse_u32_to_i32(){
    assert_eq!(0_i32,uint::toi32p(0_u32));
    assert_eq!(2147483647_i32,uint::toi32p(2147483647_u32));
}

#[test]
fn parse_u32_to_u8(){
    assert_eq!(0_u8,uint::tou8p(0_u32));
    assert_eq!(255_u8,uint::tou8p(255_u32));
}

#[test]
fn parse_u32_to_u16(){
    assert_eq!(0_u16,uint::tou16p(0_u32));
    assert_eq!(65535_u16,uint::tou16p(65535_u32));
}
