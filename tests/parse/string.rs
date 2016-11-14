extern crate rustils;

use rustils::parse::*;

#[test]
fn parse_string_to_bool_0() {
    let a = String::from("TRue");
    let b = "FaLsE";
    let c = "0";

    assert_eq!(true, a.to_bool());
    assert_eq!(false, b.to_bool());
    assert_eq!(false, c.to_bool());
}

#[test]
#[should_panic(expected = "Invalid String: Hello World")]
fn parse_string_to_bool_1() { String::from("Hello World").to_bool(); }


#[test]
fn parse_string_to_i8_0() {
    let a = String::from("-128");
    let b = "-128";

    let c = String::from("127");
    let d = "127";

    assert_eq!(-128_i8, a.to_i8());
    assert_eq!(-128_i8, b.to_i8());

    assert_eq!(127_i8, c.to_i8());
    assert_eq!(127_i8, d.to_i8());
}

#[test]
#[should_panic(expected = "Invalid Number: -129")]
fn parse_string_to_i8_1() { String::from("-129").to_i8(); }

#[test]
#[should_panic(expected = "Invalid Number: 128")]
fn parse_string_to_i8_2() { ("128").to_i8(); }


#[test]
fn parse_string_to_i16_0() {
    let a = String::from("-32768");
    let b = "-32768";

    let c = String::from("32767");
    let d = "32767";

    assert_eq!(-32768_i16, a.to_i16());
    assert_eq!(-32768_i16, b.to_i16());

    assert_eq!(32767_i16, c.to_i16());
    assert_eq!(32767_i16, d.to_i16());
}

#[test]
#[should_panic(expected = "Invalid Number: -32769")]
fn parse_string_to_i16_1() { String::from("-32769").to_i16(); }

#[test]
#[should_panic(expected = "Invalid Number: 32768")]
fn parse_string_to_i16_2() { ("32768").to_i16(); }


#[test]
fn parse_string_to_i32_0() {
    let a = String::from("-2147483648");
    let b = "-2147483648";

    let c = String::from("2147483647");
    let d = "2147483647";

    assert_eq!(-2147483648_i32, a.to_i32());
    assert_eq!(-2147483648_i32, b.to_i32());

    assert_eq!(2147483647_i32, c.to_i32());
    assert_eq!(2147483647_i32, d.to_i32());
}

#[test]
#[should_panic(expected = "Invalid Number: -2147483649")]
fn parse_string_to_i32_1() { String::from("-2147483649").to_i32(); }

#[test]
#[should_panic(expected = "Invalid Number: 2147483648")]
fn parse_string_to_i32_2() { ("2147483648").to_i32(); }


#[test]
fn parse_string_to_i64_0() {
    let a = String::from("-9223372036854775808");
    let b = "-9223372036854775808";

    let c = String::from("9223372036854775807");
    let d = "9223372036854775807";

    assert_eq!(-9223372036854775808_i64, a.to_i64());
    assert_eq!(-9223372036854775808_i64, b.to_i64());

    assert_eq!(9223372036854775807_i64, c.to_i64());
    assert_eq!(9223372036854775807_i64, d.to_i64());
}

#[test]
#[should_panic(expected = "Invalid Number: -9223372036854775809")]
fn parse_string_to_i64_1() { String::from("-9223372036854775809").to_i64(); }

#[test]
#[should_panic(expected = "Invalid Number: 9223372036854775808")]
fn parse_string_to_i64_2() { ("9223372036854775808").to_i64(); }


#[test]
fn parse_string_to_isize_0() {
    let a = String::from("-9223372036854775808");
    let b = "-9223372036854775808";

    let c = String::from("9223372036854775807");
    let d = "9223372036854775807";

    assert_eq!(-9223372036854775808_isize, a.to_isize());
    assert_eq!(-9223372036854775808_isize, b.to_isize());

    assert_eq!(9223372036854775807_isize, c.to_isize());
    assert_eq!(9223372036854775807_isize, d.to_isize());
}

#[test]
#[should_panic(expected = "Invalid Number: -9223372036854775809")]
fn parse_string_to_isize_1() { String::from("-9223372036854775809").to_isize(); }

#[test]
#[should_panic(expected = "Invalid Number: 9223372036854775808")]
fn parse_string_to_isize_2() { ("9223372036854775808").to_isize(); }


#[test]
fn parse_string_to_u8_0() {
    let a = String::from("0");
    let b = "0";

    let c = String::from("255");
    let d = "255";

    assert_eq!(0_u8, a.to_u8());
    assert_eq!(0_u8, b.to_u8());

    assert_eq!(255_u8, c.to_u8());
    assert_eq!(255_u8, d.to_u8());
}

#[test]
#[should_panic(expected = "Invalid Number: -1")]
fn parse_string_to_u8_1() { String::from("-1").to_u8(); }

#[test]
#[should_panic(expected = "Invalid Number: 256")]
fn parse_string_to_u8_2() { ("256").to_u8(); }


#[test]
fn parse_string_to_u16_0() {
    let a = String::from("0");
    let b = "0";

    let c = String::from("65535");
    let d = "65535";

    assert_eq!(0_u16, a.to_u16());
    assert_eq!(0_u16, b.to_u16());

    assert_eq!(65535_u16, c.to_u16());
    assert_eq!(65535_u16, d.to_u16());
}

#[test]
#[should_panic(expected = "Invalid Number: -1")]
fn parse_string_to_u16_1() { String::from("-1").to_u16(); }

#[test]
#[should_panic(expected = "Invalid Number: 65536")]
fn parse_string_to_u16_2() { ("65536").to_u16(); }


#[test]
fn parse_string_to_u32_0() {
    let a = String::from("0");
    let b = "0";

    let c = String::from("4294967295");
    let d = "4294967295";

    assert_eq!(0_u32, a.to_u32());
    assert_eq!(0_u32, b.to_u32());

    assert_eq!(4294967295_u32, c.to_u32());
    assert_eq!(4294967295_u32, d.to_u32());
}

#[test]
#[should_panic(expected = "Invalid Number: -1")]
fn parse_string_to_u32_1() { String::from("-1").to_u32(); }

#[test]
#[should_panic(expected = "Invalid Number: 4294967296")]
fn parse_string_to_u32_2() { ("4294967296").to_u32(); }


#[test]
fn parse_string_to_u64_0() {
    let a = String::from("0");
    let b = "0";

    let c = String::from("18446744073709551615");
    let d = "18446744073709551615";

    assert_eq!(0_u64, a.to_u64());
    assert_eq!(0_u64, b.to_u64());

    assert_eq!(18446744073709551615_u64, c.to_u64());
    assert_eq!(18446744073709551615_u64, d.to_u64());
}

#[test]
#[should_panic(expected = "Invalid Number: -1")]
fn parse_string_to_u64_1() { String::from("-1").to_u64(); }

#[test]
#[should_panic(expected = "Invalid Number: 18446744073709551616")]
fn parse_string_to_u64_2() { ("18446744073709551616").to_u64(); }


#[test]
fn parse_string_to_usize_0() {
    let a = String::from("0");
    let b = "0";

    let c = String::from("18446744073709551615");
    let d = "18446744073709551615";

    assert_eq!(0_usize, a.to_usize());
    assert_eq!(0_usize, b.to_usize());

    assert_eq!(18446744073709551615_usize, c.to_usize());
    assert_eq!(18446744073709551615_usize, d.to_usize());
}

#[test]
#[should_panic(expected = "Invalid Number: -1")]
fn parse_string_to_usize_1() { String::from("-1").to_usize(); }

#[test]
#[should_panic(expected = "Invalid Number: 18446744073709551616")]
fn parse_string_to_usize_2() { ("18446744073709551616").to_usize(); }
