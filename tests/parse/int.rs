extern crate rustils;

use rustils::parse::int;
use std::error::Error;

#[test]
fn parse_i32_to_bool(){
    assert_eq!(true,int::to_bool(-1_i32));
    assert_eq!(true,int::to_bool(1_i32));
    assert_eq!(false,int::to_bool(0_i32));
}

#[test]
fn parse_i32_to_i8(){
    assert_eq!(127_i8,int::to_i8(127_i32).unwrap());
    assert_eq!(-128_i8,int::to_i8(-128_i32).unwrap());

    assert_eq!("Invalid Number",
        int::to_i8(-129_i32).err().unwrap().description());

    assert_eq!("Invalid Number",
        int::to_i8(128_i32).err().unwrap().description());
}

#[test]
fn parse_i32_to_i16(){
    assert_eq!(32767_i16,int::to_i16(32767_i32).unwrap());
    assert_eq!(-32768_i16,int::to_i16(-32768_i32).unwrap());

    assert_eq!("Invalid Number",
        int::to_i16(-32769_i32).err().unwrap().description());

    assert_eq!("Invalid Number",
        int::to_i16(32768_i32).err().unwrap().description());
}

#[test]
fn parse_i32_to_u8(){
    assert_eq!(0_u8,int::to_u8(0_i32).unwrap());
    assert_eq!(255_u8,int::to_u8(255_i32).unwrap());

    assert_eq!("Invalid Number",
        int::to_u8(-1_i32).err().unwrap().description());

    assert_eq!("Invalid Number",
        int::to_u8(256_i32).err().unwrap().description());
}

#[test]
fn parse_i32_to_u16(){
    assert_eq!(0_u16,int::to_u16(0_i32).unwrap());
    assert_eq!(65535_u16,int::to_u16(65535_i32).unwrap());

    assert_eq!("Invalid Number",
        int::to_u16(-1_i32).err().unwrap().description());

    assert_eq!("Invalid Number",
        int::to_u16(65536_i32).err().unwrap().description());
}

#[test]
fn parse_i32_to_u32(){
    assert_eq!(0_u32,int::to_u32(0_i32).unwrap());
    assert_eq!(2147483647_u32,int::to_u32(2147483647_i32).unwrap());

    assert_eq!("Invalid Number",
        int::to_u32(-1_i32).err().unwrap().description());
}

#[test]
fn parse_i32_to_u64(){
    assert_eq!(0_u64,int::to_u64(0_i32).unwrap());
    assert_eq!(2147483647_u64,int::to_u64(2147483647_i32).unwrap());

    assert_eq!("Invalid Number",
        int::to_u64(-1_i32).err().unwrap().description());
}


#[test]
fn parse_i32_to_usize(){
    assert_eq!(0_usize,int::to_usize(0_i32).unwrap());
    assert_eq!(2147483647_usize,int::to_usize(2147483647_i32).unwrap());

    assert_eq!("Invalid Number",
        int::to_usize(-1_i32).err().unwrap().description());
}

#[test]
#[should_panic(expected = "Invalid Number: -129")]
fn parse_i32_to_i8p_0() { int::to_i8p(-129_i32); }

#[test]
#[should_panic(expected = "Invalid Number: 128")]
fn parse_i32_to_i8p_1() { int::to_i8p(128_i32); }

#[test]
#[should_panic(expected = "Invalid Number: -32769")]
fn parse_i32_to_i16p_0() { int::to_i8p(-32769_i32); }

#[test]
#[should_panic(expected = "Invalid Number: 32768")]
fn parse_i32_to_i16p_1() { int::to_i8p(32768_i32); }

#[test]
#[should_panic(expected = "Invalid Number: -1")]
fn parse_i32_to_u8p_0() { int::to_u8p(-1_i32); }

#[test]
#[should_panic(expected = "Invalid Number: 256")]
fn parse_i32_to_u8p_1() { int::to_u8p(256_i32); }

#[test]
#[should_panic(expected = "Invalid Number: -1")]
fn parse_i32_to_u16p_0() { int::to_u16p(-1_i32); }

#[test]
#[should_panic(expected = "Invalid Number: 65536")]
fn parse_i32_to_u16p_1() { int::to_u16p(65536_i32); }

#[test]
#[should_panic(expected = "Invalid Number: -1")]
fn parse_i32_to_u32p_0() { int::to_u32p(-1_i32); }

#[test]
#[should_panic(expected = "Invalid Number: -1")]
fn parse_i32_to_u64p_0() { int::to_u64p(-1_i32); }

#[test]
#[should_panic(expected = "Invalid Number: -1")]
fn parse_i32_to_usizep_0() { int::to_usizep(-1_i32); }
