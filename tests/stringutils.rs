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

#[test]
fn adv_starts_with(){
    assert_eq!((true,String::from(" World")),
        stringutils::adv_starts_with("Hello World","Hello"));

    assert_eq!((false,String::from("o World")),
        stringutils::adv_starts_with("Hello World","ello"));
}

#[test]
fn starts_with(){
    assert_eq!(true,stringutils::starts_with("Hello World","Hello"));
    assert_eq!(false,stringutils::starts_with("Hello World","ello"));
}

#[test]
fn adv_ends_with(){
    assert_eq!((true,String::from("Hello ")),
        stringutils::adv_ends_with("Hello World","World"));

    assert_eq!((false,String::from("Hello W")),
        stringutils::adv_ends_with("Hello World","Worl"));
}

#[test]
fn ends_with(){
    assert_eq!(true,stringutils::ends_with("Hello World","World"));
    assert_eq!(false,stringutils::ends_with("Hello World","Worl"));
}

#[test]
fn cmp_ingnore_case(){
    assert_eq!(true,stringutils::cmp_ingnore_case("Hello World","HeLLo WORld"));
    assert_eq!(false,stringutils::cmp_ingnore_case("Hello World","Hell World"));
}

#[test]
fn peek(){
    assert_eq!(Some('d'),stringutils::peek("Hello World"));
    assert_eq!(None,stringutils::peek(&String::new()));
}

#[test]
#[should_panic]
fn peekp(){
    assert_eq!('d',stringutils::peekp("Hello World"));
    stringutils::peekp(&String::new());
}

#[test]
fn string_to_str(){
    assert_eq!("",String::new());
    assert_eq!("Hello",String::from("Hello"));
}
