extern crate rustils;

use rustils::parse::ulong;

#[test]
fn parse_u64_to_bool(){
    assert_eq!(true,ulong::to_bool(42_u64));
    assert_eq!(true,ulong::to_bool(1_u64));
    assert_eq!(false,ulong::to_bool(0_u64));
}

#[test]
fn parse_u64_to_i8(){
    assert_eq!(0_i8,ulong::toi8p(0_u64));
    assert_eq!(127_i8,ulong::toi8p(127_u64));
}

#[test]
fn parse_u64_to_i16(){
    assert_eq!(0_i16,ulong::toi16p(0_u64));
    assert_eq!(32767_i16,ulong::toi16p(32767_u64));
}

#[test]
fn parse_u64_to_i32(){
    assert_eq!(0_i32,ulong::toi32p(0_u64));
    assert_eq!(2147483647_i32,ulong::toi32p(2147483647_u64));
}

#[test]
fn parse_u64_to_i64(){
    assert_eq!(0_i64,ulong::toi64p(0_u64));
    assert_eq!(9223372036854775807_i64,ulong::toi64p(9223372036854775807_u64));
}

#[test]
fn parse_u64_to_u8(){
    assert_eq!(0_u8,ulong::tou8p(0_u64));
    assert_eq!(255_u8,ulong::tou8p(255_u64));
}

#[test]
fn parse_u64_to_u16(){
    assert_eq!(0_u16,ulong::tou16p(0_u64));
    assert_eq!(65535_u16,ulong::tou16p(65535_u64));
}

#[test]
fn parse_u64_to_u32(){
    assert_eq!(0_u32,ulong::tou32p(0_u64));
    assert_eq!(4294967295_u32,ulong::tou32p(4294967295_u64));
}
