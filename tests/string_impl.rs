extern crate rustils;
use rustils::string::StringUtils;

#[test]
pub fn test_adv_contains_all_chars_string() {
    let chars1 = ['好','l'];
    let chars2 = ['a','？'];
    let chars3 = ['l',':'];
    let text = "Hello, World! 你好吗？".to_string();

    assert_eq!(
        (true, vec!(17,2), vec!('好','l')),
        text.adv_contains_all_chars(&chars1)
    );

    assert_eq!(
        (false, vec!(), vec!()),
        text.adv_contains_all_chars(&chars2)
    );

    assert_eq!(
        (false, vec!(2), vec!('l')),
        text.adv_contains_all_chars(&chars3)
    );
}

#[test]
pub fn test_adv_contains_all_strs_string() {
    let strs1 = ["好吗？","l"];
    let strs2 = ["foo","World"];
    let strs3 = ["Hello","bar"];
    let text = "Hello, World! 你好吗？".to_string();

    assert_eq!(
        (true, vec!(17,2), vec!("好吗？".to_string(),"l".to_string())),
        text.adv_contains_all_strs(&strs1)
    );

    assert_eq!(
        (false, vec!(), vec!()),
        text.adv_contains_all_strs(&strs2)
    );

    assert_eq!(
        (false, vec!(0), vec!("Hello".to_string())),
        text.adv_contains_all_strs(&strs3)
    );
}

#[test]
pub fn test_adv_contains_any_char_string() {
    let chars1 = ['你','l'];
    let chars2 = ['a','？'];
    let chars3 = [';',':'];
    let text = "Hello, World! 你好吗？".to_string();

    assert_eq!(
        (true, 14, '你'),
        text.adv_contains_any_char(&chars1)
    );

    assert_eq!(
        (true, 23, '？'),
        text.adv_contains_any_char(&chars2)
    );

    assert_eq!(
        (false, 0, ' '),
        text.adv_contains_any_char(&chars3)
    );
}

#[test]
pub fn test_adv_contains_any_str_string() {
    let strs1 = ["好吗？","l"];
    let strs2 = ["foo","World"];
    let strs3 = ["foo","bar"];
    let text = "Hello, World! 你好吗？".to_string();

    assert_eq!(
        (true, 17, "好吗？".to_string()),
        text.adv_contains_any_str(&strs1)
    );

    assert_eq!(
        (true, 7, "World".to_string()),
        text.adv_contains_any_str(&strs2)
    );

    assert_eq!(
        (false, 0, String::new()),
        text.adv_contains_any_str(&strs3)
    );
}

#[test]
pub fn test_adv_contains_none_char_string() {
    let chars1 = ['你','l'];
    let chars2 = ['a','？'];
    let chars3 = [';',':'];
    let text = "Hello, World! 你好吗？".to_string();

    assert_eq!(
        (false, 0, '你'),
        text.adv_contains_none_char(&chars1)
    );

    assert_eq!(
        (false, 1, '？'),
        text.adv_contains_none_char(&chars2)
    );

    assert_eq!(
        (true, 0, ' '),
        text.adv_contains_none_char(&chars3)
    );
}

#[test]
pub fn test_adv_contains_none_str_string() {
    let strs1 = ["好吗？","l"];
    let strs2 = ["foo","World"];
    let strs3 = ["foo","bar"];
    let text = "Hello, World! 你好吗？".to_string();

    assert_eq!(
        (false, 0, "好吗？".to_string()),
        text.adv_contains_none_str(&strs1)
    );

    assert_eq!(
        (false, 1, "World".to_string()),
        text.adv_contains_none_str(&strs2)
    );

    assert_eq!(
        (true, 0, String::new()),
        text.adv_contains_none_str(&strs3)
    );
}

#[test]
pub fn test_adv_ends_with_string() {
    let search1 = "吗？";
    let search2 = "World";
    let text = "Hello, World! 你好吗？".to_string();

    assert_eq!(
        (true, "Hello, World! 你好".to_string()),
        text.adv_ends_with(search1)
    );

    assert_eq!(
        (false, String::new()),
        text.adv_ends_with(search2)
    );
}

#[test]
pub fn test_adv_has_alpha_string(){
    let text1 = "Hello!".to_string();
    let text2 = "2017".to_string();

    assert_eq!(
        (true, vec!(true, true, true, true, true, false)),
        text1.adv_has_alpha()
    );

    assert_eq!(
        (false, vec!(false, false, false, false)),
        text2.adv_has_alpha()
    );
}

#[test]
pub fn test_adv_has_alphanumeric_string(){
    let text1 = "H3ll0!".to_string();
    let text2 = " ! ".to_string();

    assert_eq!(
        (true, vec!(true, true, true, true, true, false)),
        text1.adv_has_alphanumeric()
    );

    assert_eq!(
        (false, vec!(false, false, false)),
        text2.adv_has_alphanumeric()
    );
}

#[test]
pub fn test_adv_has_alphanumeric_space_string(){
    let text1 = "By3 bye!".to_string();
    let text2 = ":?!".to_string();

    assert_eq!(
        (true, vec!(true, true, true, true, true, true, true, false)),
        text1.adv_has_alphanumeric_space()
    );

    assert_eq!(
        (false, vec!(false, false, false)),
        text2.adv_has_alphanumeric_space()
    );
}

#[test]
pub fn test_adv_has_alpha_space_string(){
    let text1 = "By3 bye!".to_string();
    let text2 = "42!".to_string();

    assert_eq!(
        (true, vec!(true, true, false, true, true, true, true, false)),
        text1.adv_has_alpha_space()
    );

    assert_eq!(
        (false, vec!(false, false, false)),
        text2.adv_has_alpha_space()
    );
}

#[test]
pub fn test_adv_has_lowercase_string(){
    let text1 = "Hello!".to_string();
    let text2 = "HELLO!".to_string();

    assert_eq!(
        (true, vec!(false, true, true, true, true, false)),
        text1.adv_has_lowercase()
    );

    assert_eq!(
        (false, vec!(false, false, false, false, false, false)),
        text2.adv_has_lowercase()
    );
}

#[test]
pub fn test_adv_has_numeric_string(){
    let text1 = "H3ll0!".to_string();
    let text2 = "Hello!".to_string();

    assert_eq!(
        (true, vec!(false, true, false, false, true, false)),
        text1.adv_has_numeric()
    );

    assert_eq!(
        (false, vec!(false, false, false, false, false, false)),
        text2.adv_has_numeric()
    );
}

#[test]
pub fn test_adv_has_numeric_space_string(){
    let text1 = "(1, 2)".to_string();
    let text2 = "Hello!".to_string();

    assert_eq!(
        (true, vec!(false, true, false, true, true, false)),
        text1.adv_has_numeric_space()
    );

    assert_eq!(
        (false, vec!(false, false, false, false, false, false)),
        text2.adv_has_numeric_space()
    );
}

#[test]
pub fn test_adv_has_uppercase_string(){
    let text1 = "Hello!".to_string();
    let text2 = "hello".to_string();

    assert_eq!(
        (true, vec!(true, false, false, false, false, false)),
        text1.adv_has_uppercase()
    );

    assert_eq!(
        (false, vec!(false, false, false, false, false)),
        text2.adv_has_uppercase()
    );
}

#[test]
pub fn test_adv_has_whitespace_string(){
    let text1 = "Bye bye".to_string();
    let text2 = "hello".to_string();

    assert_eq!(
        (true, vec!(false, false, false, true, false, false, false)),
        text1.adv_has_whitespace()
    );

    assert_eq!(
        (false, vec!(false, false, false, false, false)),
        text2.adv_has_whitespace()
    );
}

#[test]
pub fn test_adv_is_alpha_string(){
    let text1 = "Hello".to_string();
    let text2 = "H3ll0".to_string();

    assert_eq!(
        (true, vec!(true, true, true, true, true)),
        text1.adv_is_alpha()
    );

    assert_eq!(
        (false, vec!(true, false, true, true, false)),
        text2.adv_is_alpha()
    );
}

#[test]
pub fn test_adv_is_alphanumeric_string(){
    let text1 = "H3ll0".to_string();
    let text2 = " ! ".to_string();

    assert_eq!(
        (true, vec!(true, true, true, true, true)),
        text1.adv_is_alphanumeric()
    );

    assert_eq!(
        (false, vec!(false, false, false)),
        text2.adv_is_alphanumeric()
    );
}

#[test]
pub fn test_adv_is_alphanumeric_space_string(){
    let text1 = "By3 bye".to_string();
    let text2 = ":?!".to_string();

    assert_eq!(
        (true, vec!(true, true, true, true, true, true, true)),
        text1.adv_is_alphanumeric_space()
    );

    assert_eq!(
        (false, vec!(false, false, false)),
        text2.adv_is_alphanumeric_space()
    );
}

#[test]
pub fn test_adv_is_alpha_space_string(){
    let text1 = "Bye bye".to_string();
    let text2 = "By3 bye".to_string();

    assert_eq!(
        (true, vec!(true, true, true, true, true, true, true)),
        text1.adv_is_alpha_space()
    );

    assert_eq!(
        (false, vec!(true, true, false, true, true, true, true)),
        text2.adv_is_alpha_space()
    );
}

#[test]
pub fn test_adv_is_lowercase_string(){
    let text1 = "hello".to_string();
    let text2 = "Hello".to_string();

    assert_eq!(
        (true, vec!(true, true, true, true, true)),
        text1.adv_is_lowercase()
    );

    assert_eq!(
        (false, vec!(false, true, true, true, true)),
        text2.adv_is_lowercase()
    );
}

#[test]
pub fn test_adv_is_numeric_string(){
    let text1 = "2017".to_string();
    let text2 = "2OI7".to_string();

    assert_eq!(
        (true, vec!(true, true, true, true)),
        text1.adv_is_numeric()
    );

    assert_eq!(
        (false, vec!(true, false, false, true)),
        text2.adv_is_numeric()
    );
}

#[test]
pub fn test_adv_is_numeric_space_string(){
    let text1 = "1 2".to_string();
    let text2 = "(1, 2)".to_string();

    assert_eq!(
        (true, vec!(true, true, true)),
        text1.adv_is_numeric_space()
    );

    assert_eq!(
        (false, vec!(false, true, false, true, true, false)),
        text2.adv_is_numeric_space()
    );
}

#[test]
pub fn test_adv_is_uppercase_string(){
    let text1 = "HELLO".to_string();
    let text2 = "Hello".to_string();

    assert_eq!(
        (true, vec!(true, true, true, true, true)),
        text1.adv_is_uppercase()
    );

    assert_eq!(
        (false, vec!(true, false, false, false, false)),
        text2.adv_is_uppercase()
    );
}

#[test]
pub fn test_adv_is_whitespace_string(){
    let text1 = "   ".to_string();
    let text2 = " ! ".to_string();

    assert_eq!(
        (true, vec!(true, true, true)),
        text1.adv_is_whitespace()
    );

    assert_eq!(
        (false, vec!(true, false, true)),
        text2.adv_is_whitespace()
    );
}

#[test]
pub fn test_adv_starts_with_string() {
    let search1 = "Hello";
    let search2 = "你好";
    let text = "Hello, World! 你好吗？".to_string();

    assert_eq!(
        (true, ", World! 你好吗？".to_string()),
        text.adv_starts_with(search1)
    );

    assert_eq!(
        (false, String::new()),
        text.adv_starts_with(search2)
    );
}

#[test]
pub fn test_contains_all_chars_string() {
    let chars1 = ['你','l'];
    let chars2 = ['a','？'];
    let chars3 = ['l',':'];
    let text = "Hello, World! 你好吗？".to_string();

    assert_eq!(true, text.contains_all_chars(&chars1));
    assert_eq!(false, text.contains_all_chars(&chars2));
    assert_eq!(false, text.contains_all_chars(&chars3));
}

#[test]
pub fn test_contains_all_strs_string() {
    let strs1 = ["好吗？","l"];
    let strs2 = ["foo","World"];
    let strs3 = ["Hello","bar"];
    let text = "Hello, World! 你好吗？".to_string();

    assert_eq!(true, text.contains_all_strs(&strs1));
    assert_eq!(false, text.contains_all_strs(&strs2));
    assert_eq!(false, text.contains_all_strs(&strs3));
}

#[test]
pub fn test_contains_any_char_string() {
    let chars1 = ['你','l'];
    let chars2 = ['a','？'];
    let chars3 = [';',':'];
    let text = "Hello, World! 你好吗？".to_string();

    assert_eq!(true, text.contains_any_char(&chars1));
    assert_eq!(true, text.contains_any_char(&chars2));
    assert_eq!(false, text.contains_any_char(&chars3));
}

#[test]
pub fn test_contains_any_str_string() {
    let strs1 = ["好吗？","l"];
    let strs2 = ["foo","World"];
    let strs3 = ["foo","bar"];
    let text = "Hello, World! 你好吗？".to_string();

    assert_eq!(true, text.contains_any_str(&strs1));
    assert_eq!(true, text.contains_any_str(&strs2));
    assert_eq!(false, text.contains_any_str(&strs3));
}

#[test]
pub fn test_contains_none_char_string() {
    let chars1 = ['你','l'];
    let chars2 = ['a','？'];
    let chars3 = [';',':'];
    let text = "Hello, World! 你好吗？".to_string();

    assert_eq!(false, text.contains_none_char(&chars1));
    assert_eq!(false, text.contains_none_char(&chars2));
    assert_eq!(true, text.contains_none_char(&chars3));
}

#[test]
pub fn test_contains_none_str_string() {
    let strs1 = ["好吗？","l"];
    let strs2 = ["foo","World"];
    let strs3 = ["foo","bar"];
    let text = "Hello, World! 你好吗？".to_string();

    assert_eq!(false, text.contains_none_str(&strs1));
    assert_eq!(false, text.contains_none_str(&strs2));
    assert_eq!(true, text.contains_none_str(&strs3));
}

#[test]
pub fn test_cmp_ignore_case_string(){
    let text1 = "Hello, World! 你好吗？".to_string();
    let text2 = "hello, WORLD! 你好吗？".to_string();
    let text3 = "hello! 你好吗？".to_string();

    assert_eq!(true, text1.cmp_ignore_case(&text2));
    assert_eq!(false, text1.cmp_ignore_case(&text3));
}

#[test]
pub fn test_differnce_string(){
    let text1 = "Hello, World! 你好吗？".to_string();
    let text2 = "Hallo, Welt ! 你好嗎？".to_string();
    let text3 = "Hello!".to_string();

    assert_eq!(
        vec!(1, 8, 9, 10, 11, 16),
        text1.difference(&text2)
    );

    assert_eq!(
        vec!(5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17),
        text1.difference(&text3)
    );

    assert_eq!(
        vec!(5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17),
        text3.difference(&text1)
    );

    assert!(text1.difference(&text2) == text2.difference(&text1));
}

#[test]
pub fn test_find_char_string_0(){
    let text = "Hello, World! 你好吗？".to_string();
    assert_eq!(15, text.find_char('好'));
}

#[test]
#[should_panic(expected = "string doesn't contain #")]
pub fn test_find_char_string_1(){
    let text = "Hello, World! 你好吗？".to_string();
    text.find_char('#');
}

#[test]
pub fn test_find_char_opt_string(){
    let text = "Hello, World! 你好吗？".to_string();

    assert_eq!(Some(15), text.find_char_opt('好'));
    assert_eq!(None, text.find_char_opt('#'));
}

#[test]
pub fn test_has_alpha_string(){
    let text1 = "Hello!".to_string();
    let text2 = "2017".to_string();

    assert_eq!(true, text1.has_alpha());
    assert_eq!(false, text2.has_alpha());
}

#[test]
pub fn test_has_alphanumeric_string(){
    let text1 = "H3ll0!".to_string();
    let text2 = " ! ".to_string();

    assert_eq!(true, text1.has_alphanumeric());
    assert_eq!(false, text2.has_alphanumeric());
}

#[test]
pub fn test_has_alphanumeric_space_string(){
    let text1 = "By3 bye!".to_string();
    let text2 = ":?!".to_string();

    assert_eq!(true, text1.has_alphanumeric_space());
    assert_eq!(false, text2.has_alphanumeric_space());
}

#[test]
pub fn test_has_alpha_space_string(){
    let text1 = "By3 bye!".to_string();
    let text2 = "42!".to_string();

    assert_eq!(true, text1.has_alpha_space());
    assert_eq!(false, text2.has_alpha_space());
}

#[test]
pub fn test_has_lowercase_string(){
    let text1 = "Hello!".to_string();
    let text2 = "HELLO!".to_string();

    assert_eq!(true, text1.has_lowercase());
    assert_eq!(false, text2.has_lowercase());
}

#[test]
pub fn test_has_numeric_string(){
    let text1 = "H3ll0!".to_string();
    let text2 = "Hello!".to_string();

    assert_eq!(true, text1.has_numeric());
    assert_eq!(false, text2.has_numeric());
}

#[test]
pub fn test_has_numeric_space_string(){
    let text1 = "(1, 2)".to_string();
    let text2 = "Hello!".to_string();

    assert_eq!(true, text1.has_numeric_space());
    assert_eq!(false, text2.has_numeric_space());
}

#[test]
pub fn test_has_uppercase_string(){
    let text1 = "Hello!".to_string();
    let text2 = "hello".to_string();

    assert_eq!(true, text1.has_uppercase());
    assert_eq!(false, text2.has_uppercase());
}

#[test]
pub fn test_has_whitespace_string(){
    let text1 = "Bye bye".to_string();
    let text2 = "hello".to_string();

    assert_eq!(true, text1.has_whitespace());
    assert_eq!(false, text2.has_whitespace());
}

#[test]
pub fn test_is_alpha_string(){
    let text1 = "Hello".to_string();
    let text2 = "H3ll0".to_string();

    assert_eq!(true, text1.is_alpha());
    assert_eq!(false, text2.is_alpha());
}

#[test]
pub fn test_is_alphanumeric_string(){
    let text1 = "H3ll0".to_string();
    let text2 = " ! ".to_string();

    assert_eq!(true, text1.is_alphanumeric());
    assert_eq!(false, text2.is_alphanumeric());
}

#[test]
pub fn test_is_alphanumeric_space_string(){
    let text1 = "By3 bye".to_string();
    let text2 = ":?!".to_string();

    assert_eq!(true, text1.is_alphanumeric_space());
    assert_eq!(false, text2.is_alphanumeric_space());
}

#[test]
pub fn test_is_alpha_space_string(){
    let text1 = "Bye bye".to_string();
    let text2 = "By3 bye".to_string();

    assert_eq!(true, text1.is_alpha_space());
    assert_eq!(false, text2.is_alpha_space());
}

#[test]
pub fn test_is_lowercase_string(){
    let text1 = "hello".to_string();
    let text2 = "Hello".to_string();

    assert_eq!(true, text1.is_lowercase());
    assert_eq!(false, text2.is_lowercase());
}

#[test]
pub fn test_is_numeric_string(){
    let text1 = "2017".to_string();
    let text2 = "2OI7".to_string();

    assert_eq!(true, text1.is_numeric());
    assert_eq!(false, text2.is_numeric());
}

#[test]
pub fn test_is_numeric_space_string(){
    let text1 = "1 2".to_string();
    let text2 = "(1, 2)".to_string();

    assert_eq!(true, text1.is_numeric_space());
    assert_eq!(false, text2.is_numeric_space());
}

#[test]
pub fn test_is_uppercase_string(){
    let text1 = "HELLO".to_string();
    let text2 = "Hello".to_string();

    assert_eq!(true, text1.is_uppercase());
    assert_eq!(false, text2.is_uppercase());
}

#[test]
pub fn test_is_whitespace_string(){
    let text1 = "   ".to_string();
    let text2 = " ! ".to_string();

    assert_eq!(true, text1.is_whitespace());
    assert_eq!(false, text2.is_whitespace());
}

#[test]
pub fn test_peek_string_0(){
    let text = "Hello, World! 你好吗？".to_string();
    assert_eq!('？', text.peek());
}

#[test]
#[should_panic(expected = "string is empty")]
pub fn test_peek_string_1(){
    String::new().peek();
}

#[test]
pub fn test_peek_opt_string(){
    let text = "Hello, World! 你好吗？".to_string();
    let empty = String::new();

    assert_eq!(Some('？'), text.peek_opt());
    assert_eq!(None, empty.peek_opt());
}

// #[test]
// pub fn test_remove_all_regex_string_0(){
//     let text = "Hello, World! 你好吗？".to_string();
//     let regex = r"[好]+|[aeiuo]+";
//
//     assert_eq!(
//         "Hll, Wrld! 你吗？".to_string(),
//         text.remove_all_regex(regex)
//     );
// }
//
// #[test]
// #[should_panic]
// pub fn test_remove_all_regex_string_1(){
//     let text = "Hello, World! 你好吗？".to_string();
//     let regex = "Hello||World";
//
//     text.remove_all_regex(regex);
// }
//
// #[test]
// pub fn test_remove_all_regex_mut_string_0(){
//     let text = &mut "Hello, World! 你好吗？".to_string();
//     let regex = r"[好]+|[aeiuo]+";
//
//     assert_eq!(true, text.remove_all_regex_mut(regex));
//     assert_eq!(&mut "Hll, Wrld! 你吗？".to_string(), text);
// }
//
// #[test]
// pub fn test_remove_all_regex_mut_string_1(){
//     let text = &mut "Hello, World! 你好吗？".to_string();
//     let regex = "Hello||World";
//
//     assert_eq!(false, text.remove_all_regex_mut(regex));
//     assert_eq!(&mut "Hello, World! 你好吗？".to_string(), text);
// }
//
// #[test]
// pub fn test_remove_regex_string_0(){
//     let text = "Hello, World! 你好吗？".to_string();
//     let regex = r"[好]+|[aeiuo]+";
//
//     assert_eq!(
//         "Hllo, World! 你好吗？".to_string(),
//         text.remove_regex(regex)
//     );
// }
//
// #[test]
// #[should_panic]
// pub fn test_remove_regex_string_1(){
//     let text = "Hello, World! 你好吗？".to_string();
//     let regex = "Hello||World";
//
//     text.remove_regex(regex);
// }
//
// #[test]
// pub fn test_remove_regex_mut_string_0(){
//     let text = &mut "Hello, World! 你好吗？".to_string();
//     let regex = r"[好]+|[aeiuo]+";
//
//     assert_eq!(true, text.remove_regex_mut(regex));
//     assert_eq!(&mut "Hllo, World! 你好吗？".to_string(), text);
// }
//
// #[test]
// pub fn test_remove_regex_mut_string_1(){
//     let text = &mut "Hello, World! 你好吗？".to_string();
//     let regex = "Hello||World";
//
//     assert_eq!(false, text.remove_regex_mut(regex));
//     assert_eq!(&mut "Hello, World! 你好吗？".to_string(), text);
// }

#[test]
pub fn test_reverse_string(){
    let text = "Hello, World! 你好吗？".to_string();
    let temp = text.reverse();

    assert_eq!("？吗好你 !dlroW ,olleH".to_string(), temp);
    assert_eq!(text, temp.reverse());
}

#[test]
pub fn test_reverse_mut_string(){
    let text = &mut "Hello, World! 你好吗？".to_string();
    text.reverse_mut();

    assert_eq!(&mut "？吗好你 !dlroW ,olleH".to_string(), text);
}

#[test]
pub fn test_reverse_str_string(){
    let text = "Hello, World! 你好吗？".to_string();
    let temp = text.reverse_str();

    assert_eq!("？吗好你 !dlroW ,olleH", temp);
    assert_eq!(&text, temp.to_string().reverse_str());
}
