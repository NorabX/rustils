extern crate rustils;
use rustils::stringutils;

#[test]
fn reversep_str(){
    let str1 = "Hello World";
    let str2 = stringutils::reversep(str1);

    assert_eq!("Hello World",str1);
    assert_eq!("dlroW olleH",str2);
}

#[test]
fn reversep_string(){
    let str1 = String::from("Hello World");
    let str2 = stringutils::reversep(&str1);

    assert_eq!(String::from("Hello World"),str1);
    assert_eq!(String::from("dlroW olleH"),str2);
}
