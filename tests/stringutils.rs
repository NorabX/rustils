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

#[test]
fn adv_contains_any_str(){
    assert_eq!((true,6,String::from("World")),
        stringutils::adv_contains_any_str("Hello World",&["Goodbye","World"]));

    assert_eq!((true,0,String::from("Hello")),
        stringutils::adv_contains_any_str("Hello World",&["Goodbye",&String::from("Hello")]));

    assert_eq!((false,0,String::new()),
        stringutils::adv_contains_any_str("Hello World",&["Goodbye","Rust"]));

}

#[test]
fn contains_any_str(){
    assert_eq!(true,
        stringutils::contains_any_str("Hello World",&["Goodbye","World"]));

    assert_eq!(true,
        stringutils::contains_any_str("Hello World",&["Goodbye",&String::from("Hello")]));

    assert_eq!(false,
        stringutils::contains_any_str("Hello World",&["Goodbye","Rust"]));

}

#[test]
fn adv_contains_all_strs(){
    assert_eq!((true,vec!(6,0),vec!(String::from("World"),String::from("Hello"))),
        stringutils::adv_contains_all_strs("Hello World",&["World","Hello"]));

    assert_eq!((true,vec!(0,6),vec!(String::from("Hello"),String::from("World"))),
        stringutils::adv_contains_all_strs("Hello World",&[&String::from("Hello"),"World"]));

    assert_eq!((false,Vec::<usize>::new(),Vec::<String>::new()),
        stringutils::adv_contains_all_strs("Hello World",&["Goodbye","Hello"]));

}

#[test]
fn contains_all_strs(){
    assert_eq!(true,
        stringutils::contains_all_strs("Hello World",&["World","Hello"]));

    assert_eq!(true,
        stringutils::contains_all_strs("Hello World",&[&String::from("Hello"),"World"]));

    assert_eq!(false,
        stringutils::contains_all_strs("Hello World",&["Goodbye","Hello"]));

}

#[test]
fn adv_contains_any_char(){
    assert_eq!((true,8,'r'),
        stringutils::adv_contains_any_char("Hello World",&['b','a','r']));

    assert_eq!((false,0,' '),
        stringutils::adv_contains_any_char("Hello World",&['x','y','z']));
}

#[test]
fn contains_any_char(){
    assert_eq!(true,
        stringutils::contains_any_char("Hello World",&['b','a','r']));

    assert_eq!(false,
        stringutils::contains_any_char("Hello World",&['x','y','z']));
}

#[test]
fn adv_contains_all_chars(){
    assert_eq!((true,vec!(1,4,10),vec!('e','o','d')),
        stringutils::adv_contains_all_chars("Hello World",&['e','o','d']));

    assert_eq!((false,Vec::<usize>::new(),Vec::<char>::new()),
        stringutils::adv_contains_all_chars("Hello World",&['f','o','l']));
}

#[test]
fn contains_all_chars(){
    assert_eq!(true,
        stringutils::contains_all_chars("Hello World",&['e','o','d']));

    assert_eq!(false,
        stringutils::contains_all_chars("Hello World",&['f','o','l']));
}

#[test]
fn adv_contains_none_str(){
    assert_eq!((true,0,String::new()),
        stringutils::adv_contains_none_str("Hello World",&["Test","Function"]));

    assert_eq!((false,1,String::from("Hello")),
        stringutils::adv_contains_none_str("Hello World",&["Goodbye","Hello"]));

}

#[test]
fn contains_none_str(){
    assert_eq!(true,
        stringutils::contains_none_str("Hello World",&["Test","Function"]));

    assert_eq!(false,
        stringutils::contains_none_str("Hello World",&["Goodbye","Hello"]));

}

#[test]
fn adv_contains_none_char(){
    assert_eq!((true,0,' '),
        stringutils::adv_contains_none_char("Hello World",&['a','b','c','D']));

    assert_eq!((false,2,'d'),
        stringutils::adv_contains_none_char("Hello World",&['b','c','d','E']));

}

#[test]
fn contains_none_char(){
    assert_eq!(true,
        stringutils::contains_none_char("Hello World",&['a','b','c','D']));

    assert_eq!(false,
        stringutils::contains_none_char("Hello World",&['b','c','d','E']));

}
