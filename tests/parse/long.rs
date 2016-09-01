extern crate rustils;

use rustils::parse::long;

#[test]
fn parse_i64_to_bool(){
    assert_eq!(true,long::to_bool(-1_i64));
    assert_eq!(true,long::to_bool(1_i64));
    assert_eq!(false,long::to_bool(0_i64));
}

#[test]
fn parse_i64_to_i8(){
    assert_eq!(0_i8,long::toi8p(0_i64));
    assert_eq!(127_i8,long::toi8p(127_i64));
    assert_eq!(-128_i8,long::toi8p(-128_i64));
}

#[test]
fn parse_i64_to_i16(){
    assert_eq!(0_i16,long::toi16p(0_i64));
    assert_eq!(32767_i16,long::toi16p(32767_i64));
    assert_eq!(-32768_i16,long::toi16p(-32768_i64));
}

#[test]
fn parse_i64_to_i32(){
    assert_eq!(0_i32,long::toi32p(0_i64));
    assert_eq!(2147483647_i32,long::toi32p(2147483647_i64));
    assert_eq!(-2147483648_i32,long::toi32p(-2147483648_i64));
}

#[test]
fn parse_i64_to_u8(){
    assert_eq!(0_u8,long::tou8p(0_i64));
    assert_eq!(255_u8,long::tou8p(255_i64));
}

#[test]
fn parse_i64_to_u16(){
    assert_eq!(0_u16,long::tou16p(0_i64));
    assert_eq!(65535_u16,long::tou16p(65535_i64));
}

#[test]
fn parse_i64_to_u32(){
    assert_eq!(0_u32,long::tou32p(0_i64));
    assert_eq!(4294967295_u32,long::tou32p(4294967295_i64));
}

#[test]
fn parse_i64_to_u64(){
    assert_eq!(0_u64,long::tou64p(0_i64));
    assert_eq!(9223372036854775807_u64,long::tou64p(9223372036854775807_i64));
}
