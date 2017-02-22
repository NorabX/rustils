/*
extern crate rustils;
use rustils::StringUtils;

#[test]
pub fn test_adv_contains_all_chars_str() {
    let chars1 = ['好','l'];
    let chars2 = ['a','？'];
    let chars3 = ['l',':'];
    let text = "Hello, World! 你好吗？";

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
pub fn test_adv_contains_all_strs_str() {
    let strs1 = ["好吗？","l"];
    let strs2 = ["foo","World"];
    let strs3 = ["Hello","bar"];
    let text = "Hello, World! 你好吗？";

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
pub fn test_adv_contains_any_char_str() {
    let chars1 = ['你','l'];
    let chars2 = ['a','？'];
    let chars3 = [';',':'];
    let text = "Hello, World! 你好吗？";

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
pub fn test_adv_contains_any_str_str() {
    let strs1 = ["好吗？","l"];
    let strs2 = ["foo","World"];
    let strs3 = ["foo","bar"];
    let text = "Hello, World! 你好吗？";

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
pub fn test_adv_contains_none_char_str() {
    let chars1 = ['你','l'];
    let chars2 = ['a','？'];
    let chars3 = [';',':'];
    let text = "Hello, World! 你好吗？";

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
pub fn test_adv_contains_none_str_str() {
    let strs1 = ["好吗？","l"];
    let strs2 = ["foo","World"];
    let strs3 = ["foo","bar"];
    let text = "Hello, World! 你好吗？";

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
pub fn test_adv_ends_with_str() {
    let search1 = "吗？";
    let search2 = "World";
    let text = "Hello, World! 你好吗？";

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
pub fn test_adv_starts_with_str() {
    let search1 = "Hello";
    let search2 = "你好";
    let text = "Hello, World! 你好吗？";

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
pub fn test_contains_all_chars_str() {
    let chars1 = ['你','l'];
    let chars2 = ['a','？'];
    let chars3 = ['l',':'];
    let text = "Hello, World! 你好吗？";

    assert_eq!(true, text.contains_all_chars(&chars1));
    assert_eq!(false, text.contains_all_chars(&chars2));
    assert_eq!(false, text.contains_all_chars(&chars3));
}

#[test]
pub fn test_contains_all_strs_str() {
    let strs1 = ["好吗？","l"];
    let strs2 = ["foo","World"];
    let strs3 = ["Hello","bar"];
    let text = "Hello, World! 你好吗？";

    assert_eq!(true, text.contains_all_strs(&strs1));
    assert_eq!(false, text.contains_all_strs(&strs2));
    assert_eq!(false, text.contains_all_strs(&strs3));
}

#[test]
pub fn test_contains_any_char_str() {
    let chars1 = ['你','l'];
    let chars2 = ['a','？'];
    let chars3 = [';',':'];
    let text = "Hello, World! 你好吗？";

    assert_eq!(true, text.contains_any_char(&chars1));
    assert_eq!(true, text.contains_any_char(&chars2));
    assert_eq!(false, text.contains_any_char(&chars3));
}

#[test]
pub fn test_contains_any_str_str() {
    let strs1 = ["好吗？","l"];
    let strs2 = ["foo","World"];
    let strs3 = ["foo","bar"];
    let text = "Hello, World! 你好吗？";

    assert_eq!(true, text.contains_any_str(&strs1));
    assert_eq!(true, text.contains_any_str(&strs2));
    assert_eq!(false, text.contains_any_str(&strs3));
}

#[test]
pub fn test_contains_none_char_str() {
    let chars1 = ['你','l'];
    let chars2 = ['a','？'];
    let chars3 = [';',':'];
    let text = "Hello, World! 你好吗？";

    assert_eq!(false, text.contains_none_char(&chars1));
    assert_eq!(false, text.contains_none_char(&chars2));
    assert_eq!(true, text.contains_none_char(&chars3));
}

#[test]
pub fn test_contains_none_str_str() {
    let strs1 = ["好吗？","l"];
    let strs2 = ["foo","World"];
    let strs3 = ["foo","bar"];
    let text = "Hello, World! 你好吗？";

    assert_eq!(false, text.contains_none_str(&strs1));
    assert_eq!(false, text.contains_none_str(&strs2));
    assert_eq!(true, text.contains_none_str(&strs3));
}

#[test]
pub fn test_cmp_ignore_case_str(){
    let text1 = "Hello, World! 你好吗？";
    let text2 = "hello, WORLD! 你好吗？";
    let text3 = "hello! 你好吗？";

    assert_eq!(true, text1.cmp_ignore_case(&text2));
    assert_eq!(false, text1.cmp_ignore_case(&text3));
}

#[test]
pub fn test_differnce_str(){
    let text1 = "Hello, World! 你好吗？";
    let text2 = "Hallo, Welt ! 你好嗎？";
    let text3 = "Hello!";

    assert_eq!(
        vec!(1, 8, 9, 10, 11, 16),
        text1.difference(text2)
    );

    assert_eq!(
        vec!(5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17),
        text1.difference(text3)
    );

    assert_eq!(
        vec!(5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17),
        text3.difference(text1)
    );

    assert!(text1.difference(text2) == text2.difference(text1));
}

#[test]
pub fn test_find_char_str_0(){
    let text = "Hello, World! 你好吗？";
    assert_eq!(15, text.find_char('好'));
}

#[test]
#[should_panic(expected = "string doesn't contain #")]
pub fn test_find_char_str_1(){
    let text = "Hello, World! 你好吗？";
    text.find_char('#');
}

#[test]
pub fn test_find_char_opt_str(){
    let text = "Hello, World! 你好吗？";

    assert_eq!(Some(15), text.find_char_opt('好'));
    assert_eq!(None, text.find_char_opt('#'));
}

#[test]
pub fn test_peek_str_0(){
    let text = "Hello, World! 你好吗？";
    assert_eq!('？', text.peek());
}

#[test]
#[should_panic(expected = "string is empty")]
pub fn test_peek_str_1(){
    "".peek();
}

#[test]
pub fn test_peek_opt_str(){
    let text = "Hello, World! 你好吗？";
    let empty = "";

    assert_eq!(Some('？'), text.peek_opt());
    assert_eq!(None, empty.peek_opt());
}

#[test]
pub fn test_remove_all_regex_str_0(){
    let text = "Hello, World! 你好吗？";
    let regex = r"[好]+|[aeiuo]+";

    assert_eq!(
        "Hll, Wrld! 你吗？",
        text.remove_all_regex(regex)
    );
}

#[test]
#[should_panic]
pub fn test_remove_all_regex_str_1(){
    let text = "Hello, World! 你好吗？";
    let regex = "Hello||World";

    text.remove_all_regex(regex);
}

#[test]
#[should_panic(expected = "not yet implemented")]
pub fn test_remove_all_regex_mut_str_0(){
    let text = &mut "Hello, World! 你好吗？";
    let regex = r"[好]+|[aeiuo]+";

    text.remove_all_regex_mut(regex);

    assert_eq!(&mut "Hll, Wrld! 你吗？", text);
}

#[test]
#[should_panic(expected = "not yet implemented")]
pub fn test_remove_all_regex_mut_str_1(){
    let text = &mut "Hello, World! 你好吗？";
    let regex = "Hello||World";

    text.remove_all_regex_mut(regex);
}

#[test]
pub fn test_remove_regex_str_0(){
    let text = "Hello, World! 你好吗？";
    let regex = r"[好]+|[aeiuo]+";

    assert_eq!(
        "Hllo, World! 你好吗？",
        text.remove_regex(regex)
    );
}

#[test]
#[should_panic]
pub fn test_remove_regex_str_1(){
    let text = "Hello, World! 你好吗？";
    let regex = "Hello||World";

    text.remove_regex(regex);
}

#[test]
#[should_panic(expected = "not yet implemented")]
pub fn test_remove_regex_mut_str_0(){
    let text = &mut "Hello, World! 你好吗？";
    let regex = r"[好]+|[aeiuo]+";

    text.remove_regex_mut(regex);

    assert_eq!(&mut "Hllo, World! 你好吗？", text);
}

#[test]
#[should_panic(expected = "not yet implemented")]
pub fn test_remove_regex_mut_str_1(){
    let text = &mut "Hello, World! 你好吗？";
    let regex = "Hello||World";

    text.remove_regex_mut(regex);
}

#[test]
pub fn test_reverse_str(){
    let text = "Hello, World! 你好吗？";
    let temp = text.reverse();

    assert_eq!("？吗好你 !dlroW ,olleH".to_string(), temp);
    assert_eq!(text, temp.reverse());
}

#[test]
#[should_panic(expected = "not yet implemented")]
pub fn test_reverse_mut_str(){
    let text = &mut "Hello, World! 你好吗？";
    text.reverse_mut();

    assert_eq!(&mut "？吗好你 !dlroW ,olleH".to_string(), text);
}

#[test]
pub fn test_reverse_str_str(){
    let text = "Hello, World! 你好吗？";
    let temp = text.reverse_str();

    assert_eq!("？吗好你 !dlroW ,olleH", temp);
    assert_eq!(text, temp.reverse_str());
}
*/
