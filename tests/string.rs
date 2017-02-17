extern crate rustils;
use rustils::StringUtils;

#[test]
pub fn test_adv_contains_all_chars_string() {
    let chars1 = ['你','l'];
    let chars2 = ['a','？'];
    let chars3 = ['l',':'];
    let text = "Hello, World! 你好吗？".to_string();

    assert_eq!(
        (true, vec!(14,2), vec!('你','l')),
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
pub fn test_adv_contains_all_chars_str() {
    let chars1 = ['你','l'];
    let chars2 = ['a','？'];
    let chars3 = ['l',':'];
    let text = "Hello, World! 你好吗？";

    assert_eq!(
        (true, vec!(14,2), vec!('你','l')),
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
pub fn test_cmp_ignore_case_string(){
    let text1 = "Hello, World! 你好吗？".to_string();
    let text2 = "hello, WORLD! 你好吗？".to_string();
    let text3 = "hello! 你好吗？".to_string();

    assert_eq!(true, text1.cmp_ignore_case(&text2));
    assert_eq!(false, text1.cmp_ignore_case(&text3));
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
pub fn test_peek_string_0(){
    let text = "Hello, World! 你好吗？".to_string();
    assert_eq!('？', text.peek());
}

#[test]
pub fn test_peek_str_0(){
    let text = "Hello, World! 你好吗？";
    assert_eq!('？', text.peek());
}

#[test]
#[should_panic(expected = "string is empty")]
pub fn test_peek_string_1(){
    String::new().peek();
}

#[test]
#[should_panic(expected = "string is empty")]
pub fn test_peek_str_1(){
    "".peek();
}

#[test]
pub fn test_peek_opt_string(){
    let text = "Hello, World! 你好吗？".to_string();
    let empty = String::new();

    assert_eq!(Some('？'), text.peek_opt());
    assert_eq!(None, empty.peek_opt());
}

#[test]
pub fn test_peek_opt_str(){
    let text = "Hello, World! 你好吗？";
    let empty = "";

    assert_eq!(Some('？'), text.peek_opt());
    assert_eq!(None, empty.peek_opt());
}

#[test]
pub fn test_reverse_string(){
    let text = "Hello, World! 你好吗？".to_string();
    let temp = text.reverse();

    assert_eq!("？吗好你 !dlroW ,olleH".to_string(), temp);
    assert_eq!(text, temp.reverse());
}

#[test]
pub fn test_reverse_str(){
    let text = "Hello, World! 你好吗？";
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
#[should_panic(expected = "not yet implemented")]
pub fn test_reverse_mut_str(){
    let text = &mut "Hello, World! 你好吗？";
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

#[test]
pub fn test_reverse_str_str(){
    let text = "Hello, World! 你好吗？";
    let temp = text.reverse_str();

    assert_eq!("？吗好你 !dlroW ,olleH", temp);
    assert_eq!(text, temp.reverse_str());
}
