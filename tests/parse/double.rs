extern crate rustils;

use std::f64;
use rustils::parse::double;
use rustils::utils::RoundingMode::{TRUNC,ROUND,CEIL,FLOOR};
use std::error::Error;

#[test]
fn parse_f64_to_bool() {
    assert_eq!(true,double::to_bool(-1_f64));
    assert_eq!(true,double::to_bool(1_f64));
    assert_eq!(false,double::to_bool(0_f64));
    assert_eq!(true,double::to_bool(0.21_f64));
    assert_eq!(false,double::to_bool(f64::NAN));
}

#[test]
fn parse_f64_to_i8r() {
    assert_eq!(-128_i8,double::to_i8r(-128_f64,TRUNC).unwrap());
    assert_eq!(127_i8,double::to_i8r(127_f64,TRUNC).unwrap());
    assert_eq!(1_i8,double::to_i8r(1.321_f64,TRUNC).unwrap());
    assert_eq!(2_i8,double::to_i8r(1.321_f64,CEIL).unwrap());
    assert_eq!(1_i8,double::to_i8r(1.321_f64,ROUND).unwrap());
    assert_eq!(1_i8,double::to_i8r(1.546_f64,TRUNC).unwrap());
    assert_eq!(1_i8,double::to_i8r(1.546_f64,FLOOR).unwrap());
    assert_eq!(2_i8,double::to_i8r(1.546_f64,ROUND).unwrap());

    assert_eq!("Invalid Number",
        double::to_i8r(-129_f64,TRUNC).err().unwrap().description());

    assert_eq!("Invalid Number",
        double::to_i8r(128_f64,TRUNC).err().unwrap().description());

    assert_eq!("Invalid Number",
        double::to_i8r(f64::NAN,TRUNC).err().unwrap().description());
}

#[test]
#[should_panic(expected = "Invalid Number: -129")]
fn parse_f64_to_i8pr_0() { double::to_i8pr(-129_f64,TRUNC); }

#[test]
#[should_panic(expected = "Invalid Number: 128")]
fn parse_f64_to_i8pr_1() { double::to_i8pr(128_f64,TRUNC); }

#[test]
#[should_panic(expected = "Invalid Number: NaN")]
fn parse_f64_to_i8pr_2() { double::to_i8pr(f64::NAN,TRUNC); }

#[test]
fn parse_f64_to_i16r() {
    assert_eq!(-32768_i16,double::to_i16r(-32768_f64,TRUNC).unwrap());
    assert_eq!(32767_i16,double::to_i16r(32767_f64,TRUNC).unwrap());
    assert_eq!(1_i16,double::to_i16r(1.321_f64,TRUNC).unwrap());
    assert_eq!(2_i16,double::to_i16r(1.321_f64,CEIL).unwrap());
    assert_eq!(1_i16,double::to_i16r(1.321_f64,ROUND).unwrap());
    assert_eq!(1_i16,double::to_i16r(1.546_f64,TRUNC).unwrap());
    assert_eq!(1_i16,double::to_i16r(1.546_f64,FLOOR).unwrap());
    assert_eq!(2_i16,double::to_i16r(1.546_f64,ROUND).unwrap());

    assert_eq!("Invalid Number",
        double::to_i16r(-32769_f64,TRUNC).err().unwrap().description());

    assert_eq!("Invalid Number",
        double::to_i16r(32768_f64,TRUNC).err().unwrap().description());

    assert_eq!("Invalid Number",
        double::to_i16r(f64::NAN,TRUNC).err().unwrap().description());
}

#[test]
#[should_panic(expected = "Invalid Number: -32769")]
fn parse_f64_to_i16pr_0() { double::to_i16pr(-32769_f64,TRUNC); }

#[test]
#[should_panic(expected = "Invalid Number: 32768")]
fn parse_f64_to_i16pr_1() { double::to_i16pr(32768_f64,TRUNC); }

#[test]
#[should_panic(expected = "Invalid Number: NaN")]
fn parse_f64_to_i16pr_2() { double::to_i16pr(f64::NAN,TRUNC); }

#[test]
fn parse_f64_to_i32r() {
    assert_eq!(-2147483648_i32,double::to_i32r(-2147483648_f64,TRUNC).unwrap());
    assert_eq!(2147483647_i32,double::to_i32r(2147483647_f64,TRUNC).unwrap());
    assert_eq!(1_i32,double::to_i32r(1.321_f64,TRUNC).unwrap());
    assert_eq!(2_i32,double::to_i32r(1.321_f64,CEIL).unwrap());
    assert_eq!(1_i32,double::to_i32r(1.321_f64,ROUND).unwrap());
    assert_eq!(1_i32,double::to_i32r(1.546_f64,TRUNC).unwrap());
    assert_eq!(1_i32,double::to_i32r(1.546_f64,FLOOR).unwrap());
    assert_eq!(2_i32,double::to_i32r(1.546_f64,ROUND).unwrap());

    assert_eq!("Invalid Number",
        double::to_i32r(-2147483649_f64,TRUNC).err().unwrap().description());

    assert_eq!("Invalid Number",
        double::to_i32r(2147483648_f64,TRUNC).err().unwrap().description());

    assert_eq!("Invalid Number",
        double::to_i32r(f64::NAN,TRUNC).err().unwrap().description());
}

#[test]
#[should_panic(expected = "Invalid Number: -2147483649")]
fn parse_f64_to_i32pr_0() { double::to_i32pr(-2147483649_f64,TRUNC); }

#[test]
#[should_panic(expected = "Invalid Number: 2147483648")]
fn parse_f64_to_i32pr_1() { double::to_i32pr(2147483648_f64,TRUNC); }

#[test]
#[should_panic(expected = "Invalid Number: NaN")]
fn parse_f64_to_i32pr_2() { double::to_i32pr(f64::NAN,TRUNC); }

#[test]
fn parse_f64_to_i64r() {
    assert_eq!(-9007199254740991_i64,
        double::to_i64r(-9007199254740991_f64,TRUNC).unwrap());

    assert_eq!(9007199254740991_i64,
        double::to_i64r(9007199254740991_f64,TRUNC).unwrap());

    assert_eq!(1_i64,double::to_i64r(1.321_f64,TRUNC).unwrap());
    assert_eq!(2_i64,double::to_i64r(1.321_f64,CEIL).unwrap());
    assert_eq!(1_i64,double::to_i64r(1.321_f64,ROUND).unwrap());
    assert_eq!(1_i64,double::to_i64r(1.546_f64,TRUNC).unwrap());
    assert_eq!(1_i64,double::to_i64r(1.546_f64,FLOOR).unwrap());
    assert_eq!(2_i64,double::to_i64r(1.546_f64,ROUND).unwrap());

    assert_eq!("Invalid Number",
        double::to_i64r(-9007199254740992_f64,TRUNC).err().unwrap().description());

    assert_eq!("Invalid Number",
        double::to_i64r(9007199254740992_f64,TRUNC).err().unwrap().description());

    assert_eq!("Invalid Number",
        double::to_i64r(f64::NAN,TRUNC).err().unwrap().description());
}

#[test]
#[should_panic(expected = "Invalid Number: -9007199254740992")]
fn parse_f64_to_i64pr_0() { double::to_i64pr(-9007199254740992_f64,TRUNC); }

#[test]
#[should_panic(expected = "Invalid Number: 9007199254740992")]
fn parse_f64_to_i64pr_1() { double::to_i64pr(9007199254740992_f64,TRUNC); }

#[test]
#[should_panic(expected = "Invalid Number: NaN")]
fn parse_f64_to_i64pr_2() { double::to_i64pr(f64::NAN,TRUNC); }

#[test]
fn parse_f64_to_isizer() {
    assert_eq!(-9007199254740991_isize,
        double::to_isizer(-9007199254740991_f64,TRUNC).unwrap());

    assert_eq!(9007199254740991_isize,
        double::to_isizer(9007199254740991_f64,TRUNC).unwrap());

    assert_eq!(1_isize,double::to_isizer(1.321_f64,TRUNC).unwrap());
    assert_eq!(2_isize,double::to_isizer(1.321_f64,CEIL).unwrap());
    assert_eq!(1_isize,double::to_isizer(1.321_f64,ROUND).unwrap());
    assert_eq!(1_isize,double::to_isizer(1.546_f64,TRUNC).unwrap());
    assert_eq!(1_isize,double::to_isizer(1.546_f64,FLOOR).unwrap());
    assert_eq!(2_isize,double::to_isizer(1.546_f64,ROUND).unwrap());

    assert_eq!("Invalid Number",
        double::to_isizer(-9007199254740992_f64,TRUNC).err().unwrap().description());

    assert_eq!("Invalid Number",
        double::to_isizer(9007199254740992_f64,TRUNC).err().unwrap().description());

    assert_eq!("Invalid Number",
        double::to_isizer(f64::NAN,TRUNC).err().unwrap().description());
}

#[test]
#[should_panic(expected = "Invalid Number: -9007199254740992")]
fn parse_f64_to_isizepr_0() { double::to_isizepr(-9007199254740992_f64,TRUNC); }

#[test]
#[should_panic(expected = "Invalid Number: 9007199254740992")]
fn parse_f64_to_isizepr_1() { double::to_isizepr(9007199254740992_f64,TRUNC); }

#[test]
#[should_panic(expected = "Invalid Number: NaN")]
fn parse_f64_to_isizepr_2() { double::to_isizepr(f64::NAN,TRUNC); }

#[test]
fn parse_f64_to_u8r() {
    assert_eq!(0_u8,double::to_u8r(0_f64,TRUNC).unwrap());
    assert_eq!(255_u8,double::to_u8r(255_f64,TRUNC).unwrap());
    assert_eq!(1_u8,double::to_u8r(1.321_f64,TRUNC).unwrap());
    assert_eq!(2_u8,double::to_u8r(1.321_f64,CEIL).unwrap());
    assert_eq!(1_u8,double::to_u8r(1.321_f64,ROUND).unwrap());
    assert_eq!(1_u8,double::to_u8r(1.546_f64,TRUNC).unwrap());
    assert_eq!(1_u8,double::to_u8r(1.546_f64,FLOOR).unwrap());
    assert_eq!(2_u8,double::to_u8r(1.546_f64,ROUND).unwrap());

    assert_eq!("Invalid Number",
        double::to_u8r(-1_f64,TRUNC).err().unwrap().description());

    assert_eq!("Invalid Number",
        double::to_u8r(256_f64,TRUNC).err().unwrap().description());

    assert_eq!("Invalid Number",
        double::to_i8r(f64::NAN,TRUNC).err().unwrap().description());
}

#[test]
#[should_panic(expected = "Invalid Number: -1")]
fn parse_f64_to_u8pr_0() { double::to_u8pr(-1_f64,TRUNC); }

#[test]
#[should_panic(expected = "Invalid Number: 256")]
fn parse_f64_to_u8pr_1() { double::to_u8pr(256_f64,TRUNC); }

#[test]
#[should_panic(expected = "Invalid Number: NaN")]
fn parse_f64_to_u8pr_2() { double::to_u8pr(f64::NAN,TRUNC); }

#[test]
fn parse_f64_to_u16r() {
    assert_eq!(0_u16,double::to_u16r(0_f64,TRUNC).unwrap());
    assert_eq!(65535_u16,double::to_u16r(65535_f64,TRUNC).unwrap());
    assert_eq!(1_u16,double::to_u16r(1.321_f64,TRUNC).unwrap());
    assert_eq!(2_u16,double::to_u16r(1.321_f64,CEIL).unwrap());
    assert_eq!(1_u16,double::to_u16r(1.321_f64,ROUND).unwrap());
    assert_eq!(1_u16,double::to_u16r(1.546_f64,TRUNC).unwrap());
    assert_eq!(1_u16,double::to_u16r(1.546_f64,FLOOR).unwrap());
    assert_eq!(2_u16,double::to_u16r(1.546_f64,ROUND).unwrap());

    assert_eq!("Invalid Number",
        double::to_u16r(-1_f64,TRUNC).err().unwrap().description());

    assert_eq!("Invalid Number",
        double::to_u16r(65536_f64,TRUNC).err().unwrap().description());

    assert_eq!("Invalid Number",
        double::to_u16r(f64::NAN,TRUNC).err().unwrap().description());
}

#[test]
#[should_panic(expected = "Invalid Number: -1")]
fn parse_f64_to_u16pr_0() { double::to_u16pr(-1_f64,TRUNC); }

#[test]
#[should_panic(expected = "Invalid Number: 65536")]
fn parse_f64_to_u16pr_1() { double::to_u16pr(65536_f64,TRUNC); }

#[test]
#[should_panic(expected = "Invalid Number: NaN")]
fn parse_f64_to_u16pr_2() { double::to_u16pr(f64::NAN,TRUNC); }

#[test]
fn parse_f64_to_u32r() {
    assert_eq!(0_u32,double::to_u32r(0_f64,TRUNC).unwrap());
    assert_eq!(4294967295_u32,double::to_u32r(4294967295_f64,TRUNC).unwrap());
    assert_eq!(1_u32,double::to_u32r(1.321_f64,TRUNC).unwrap());
    assert_eq!(2_u32,double::to_u32r(1.321_f64,CEIL).unwrap());
    assert_eq!(1_u32,double::to_u32r(1.321_f64,ROUND).unwrap());
    assert_eq!(1_u32,double::to_u32r(1.546_f64,TRUNC).unwrap());
    assert_eq!(1_u32,double::to_u32r(1.546_f64,FLOOR).unwrap());
    assert_eq!(2_u32,double::to_u32r(1.546_f64,ROUND).unwrap());

    assert_eq!("Invalid Number",
        double::to_u32r(-1_f64,TRUNC).err().unwrap().description());

    assert_eq!("Invalid Number",
        double::to_u32r(4294967296_f64,TRUNC).err().unwrap().description());

    assert_eq!("Invalid Number",
        double::to_u32r(f64::NAN,TRUNC).err().unwrap().description());
}

#[test]
#[should_panic(expected = "Invalid Number: -1")]
fn parse_f64_to_u32pr_0() { double::to_u32pr(-1_f64,TRUNC); }

#[test]
#[should_panic(expected = "Invalid Number: 4294967296")]
fn parse_f64_to_u32pr_1() { double::to_u32pr(4294967296_f64,TRUNC); }

#[test]
#[should_panic(expected = "Invalid Number: NaN")]
fn parse_f64_to_u32pr_2() { double::to_u32pr(f64::NAN,TRUNC); }

#[test]
fn parse_f64_to_u64r() {
    assert_eq!(0_u64,double::to_u64r(0_f64,TRUNC).unwrap());

    assert_eq!(9007199254740991_u64,
        double::to_u64r(9007199254740991_f64,TRUNC).unwrap());

    assert_eq!(1_u64,double::to_u64r(1.321_f64,TRUNC).unwrap());
    assert_eq!(2_u64,double::to_u64r(1.321_f64,CEIL).unwrap());
    assert_eq!(1_u64,double::to_u64r(1.321_f64,ROUND).unwrap());
    assert_eq!(1_u64,double::to_u64r(1.546_f64,TRUNC).unwrap());
    assert_eq!(1_u64,double::to_u64r(1.546_f64,FLOOR).unwrap());
    assert_eq!(2_u64,double::to_u64r(1.546_f64,ROUND).unwrap());

    assert_eq!("Invalid Number",
        double::to_u64r(-1_f64,TRUNC).err().unwrap().description());

    assert_eq!("Invalid Number",
        double::to_u64r(9007199254740992_f64,TRUNC).err().unwrap().description());

    assert_eq!("Invalid Number",
        double::to_u64r(f64::NAN,TRUNC).err().unwrap().description());
}

#[test]
#[should_panic(expected = "Invalid Number: -1")]
fn parse_f64_to_u64pr_0() { double::to_u64pr(-1_f64,TRUNC); }

#[test]
#[should_panic(expected = "Invalid Number: 9007199254740992")]
fn parse_f64_to_u64pr_1() { double::to_u64pr(9007199254740992_f64,TRUNC); }

#[test]
#[should_panic(expected = "Invalid Number: NaN")]
fn parse_f64_to_u64pr_2() { double::to_u64pr(f64::NAN,TRUNC); }

#[test]
fn parse_f64_to_usizer() {
    assert_eq!(0_usize,double::to_usizer(0_f64,TRUNC).unwrap());

    assert_eq!(9007199254740991_usize,
        double::to_usizer(9007199254740991_f64,TRUNC).unwrap());

    assert_eq!(1_usize,double::to_usizer(1.321_f64,TRUNC).unwrap());
    assert_eq!(2_usize,double::to_usizer(1.321_f64,CEIL).unwrap());
    assert_eq!(1_usize,double::to_usizer(1.321_f64,ROUND).unwrap());
    assert_eq!(1_usize,double::to_usizer(1.546_f64,TRUNC).unwrap());
    assert_eq!(1_usize,double::to_usizer(1.546_f64,FLOOR).unwrap());
    assert_eq!(2_usize,double::to_usizer(1.546_f64,ROUND).unwrap());

    assert_eq!("Invalid Number",
        double::to_usizer(-1_f64,TRUNC).err().unwrap().description());

    assert_eq!("Invalid Number",
        double::to_usizer(9007199254740992_f64,TRUNC).err().unwrap().description());

    assert_eq!("Invalid Number",
        double::to_usizer(f64::NAN,TRUNC).err().unwrap().description());
}

#[test]
#[should_panic(expected = "Invalid Number: -1")]
fn parse_f64_to_usizepr_0() { double::to_usizepr(-1_f64,TRUNC); }

#[test]
#[should_panic(expected = "Invalid Number: 9007199254740992")]
fn parse_f64_to_usizepr_1() { double::to_usizepr(9007199254740992_f64,TRUNC); }

#[test]
#[should_panic(expected = "Invalid Number: NaN")]
fn parse_f64_to_usizepr_2() { double::to_usizepr(f64::NAN,TRUNC); }
