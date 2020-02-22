use std::iter::Iterator;
use parse::string::ToStr;
use regex::Regex;
use CharProp;
use has_char_property;
use is_char_property;

pub trait StringUtils {

    fn adv_contains_all_chars(&self, search: &[char])
        -> (bool, Vec<usize>, Vec<char>);

    fn adv_contains_all_strs(&self, search: &[&str])
        -> (bool, Vec<usize>, Vec<String>);

    fn adv_contains_any_char(&self, search: &[char])
        -> (bool, usize, char);

    fn adv_contains_any_str(&self, search: &[&str])
        -> (bool, usize, String);
    fn adv_contains_none_char(&self, search: &[char])
        -> (bool, usize, char);

    fn adv_contains_none_str(&self, search: &[&str])
        -> (bool, usize, String);

    fn adv_ends_with(&self, search: &str)
        -> (bool, String);

    fn adv_has_alpha(&self)
        -> (bool, Vec<bool>);

    fn adv_has_alphanumeric(&self)
        -> (bool, Vec<bool>);

    fn adv_has_alphanumeric_space(&self)
        -> (bool, Vec<bool>);

    fn adv_has_alpha_space(&self)
        -> (bool, Vec<bool>);

    fn adv_has_lowercase(&self)
        -> (bool, Vec<bool>);

    fn adv_has_numeric(&self)
        -> (bool, Vec<bool>);

    fn adv_has_numeric_space(&self)
        -> (bool, Vec<bool>);

    fn adv_has_uppercase(&self)
        -> (bool, Vec<bool>);

    fn adv_has_whitespace(&self)
        -> (bool, Vec<bool>);

    fn adv_is_alpha(&self)
        -> (bool, Vec<bool>);

    fn adv_is_alphanumeric(&self)
        -> (bool, Vec<bool>);

    fn adv_is_alphanumeric_space(&self)
        -> (bool, Vec<bool>);

    fn adv_is_alpha_space(&self)
        -> (bool, Vec<bool>);

    fn adv_is_lowercase(&self)
        -> (bool, Vec<bool>);

    fn adv_is_numeric(&self)
        -> (bool, Vec<bool>);

    fn adv_is_numeric_space(&self)
        -> (bool, Vec<bool>);

    fn adv_is_uppercase(&self)
        -> (bool, Vec<bool>);

    fn adv_is_whitespace(&self)
        -> (bool, Vec<bool>);

    // fn adv_remove_all_regex(&self, regex: &str)
    //     -> (bool, Vec<(usize, usize)>, String);

    // fn adv_remove_regex(&self, regex: &str)
    //     -> (bool, usize, String);

    fn adv_starts_with(&self, search: &str)
        -> (bool, String);

    fn contains_all_chars(&self, search: &[char])
        -> bool;

    fn contains_all_strs(&self, search: &[&str])
        -> bool;

    fn contains_any_char(&self, search: &[char])
        -> bool;

    fn contains_any_str(&self, search: &[&str])
        -> bool;

    fn contains_none_char(&self, search: &[char])
        -> bool;

    fn contains_none_str(&self, search: &[&str])
        -> bool;

    fn cmp_ignore_case(&self, cmp: &str)
        -> bool;

    fn difference(&self, diff: &str)
        -> Vec<usize>;

    fn find_char(&self, search: char)
        -> usize;

    fn find_char_opt(&self, search: char)
        -> Option<usize>;

    fn has_alpha(&self)
        -> bool;

    fn has_alphanumeric(&self)
        -> bool;

    fn has_alphanumeric_space(&self)
        -> bool;

    fn has_alpha_space(&self)
        -> bool;

    fn has_lowercase(&self)
        -> bool;

    fn has_numeric(&self)
        -> bool;

    fn has_numeric_space(&self)
        -> bool;

    fn has_uppercase(&self)
        -> bool;

    fn has_whitespace(&self)
        -> bool;

    fn is_alpha(&self)
        -> bool;

    fn is_alphanumeric(&self)
        -> bool;

    fn is_alphanumeric_space(&self)
        -> bool;

    fn is_alpha_space(&self)
        -> bool;

    fn is_lowercase(&self)
        -> bool;

    fn is_numeric(&self)
        -> bool;

    fn is_numeric_space(&self)
        -> bool;

    fn is_uppercase(&self)
        -> bool;

    fn is_whitespace(&self)
        -> bool;

    fn peek(&self)
        -> char;

    fn peek_opt(&self)
        -> Option<char>;

    // fn remove_all_regex(&self, regex: &str)
    //     -> String;
    //
    // fn remove_all_regex_mut(&mut self, regex: &str)
    //     -> bool;
    //
    // fn remove_regex(&self, regex: &str)
    //     -> String;
    //
    // fn remove_regex_mut(&mut self, regex: &str)
    //     -> bool;

    fn reverse(&self)
        -> String;

    fn reverse_mut(&mut self);

    fn reverse_str(&self)
        -> &'static str;
}

pub fn adv_contains_all_chars(s: &String, search: &[char])
    -> (bool, Vec<usize>, Vec<char>) {

    let mut idxs = Vec::<usize>::new();
    let mut chars = Vec::<char>::new();

    for i in 0..search.len() {
        match s.find(search[i]) {
            Some(n) => {
                idxs.push(n);
                chars.push(search[i]);
            },
            None => return (false, idxs, chars)
        }
    }

    (true, idxs, chars)
}

pub fn adv_contains_all_strs(s: &String, search: &[&str])
    -> (bool, Vec<usize>, Vec<String>) {

    let mut idxs = Vec::<usize>::new();
    let mut strs = Vec::<String>::new();

    for i in 0..search.len() {
        match s.find(search[i]) {
            Some(n) => {
                idxs.push(n);
                strs.push(String::from(search[i]));
            },
            None => return (false, idxs, strs)
        }
    }

    (true, idxs, strs)
}

pub fn adv_contains_any_char(s: &String, search: &[char])
    -> (bool, usize, char) {

    for i in 0..search.len() {
        match s.find(search[i]) {
            Some(n) => return (true, n, search[i]),
            None => {}
        }
    }

    (false, 0, ' ')
}

pub fn adv_contains_any_str(s: &String, search: &[&str])
    -> (bool, usize, String) {

    for i in 0..search.len() {
        match s.find(search[i]) {
            Some(n) => return (true, n, String::from(search[i])),
            None => {}
        }
    }

    (false, 0, String::new())
}

pub fn adv_contains_none_char(s: &String, search: &[char])
    -> (bool, usize, char) {

    for i in 0..search.len() {
        match s.find(search[i]) {
            Some(_) => return (false, i, search[i]),
            None => {}
        }
    }

    (true, 0, ' ')
}

pub fn adv_contains_none_str(s: &String, search: &[&str])
    -> (bool, usize, String) {

    for i in 0..search.len() {
        match s.find(search[i]) {
            Some(_) => return (false, i, String::from(search[i])),
            None => {}
        }
    }

    (true, 0, String::new())
}

pub fn contains_all_chars(s: &String, search: &[char])
    -> bool {

    adv_contains_all_chars(s, search).0
}

pub fn contains_all_strs(s: &String, search: &[&str])
    -> bool {

    adv_contains_all_strs(s, search).0
}

pub fn contains_any_char(s: &String, search: &[char])
    -> bool {

    adv_contains_any_char(s, search).0
}

pub fn contains_any_str(s: &String, search: &[&str])
    -> bool {

    adv_contains_any_str(s, search).0
}

pub fn contains_none_char(s: &String, search: &[char])
    -> bool {

    adv_contains_none_char(s, search).0
}

pub fn contains_none_str(s: &String, search: &[&str])
    -> bool {

    adv_contains_none_str(s, search).0
}

pub fn adv_ends_with(s: &String, search: &str)
    -> (bool, String) {

    let temp = s.clone();
    let len = temp.len();

    if temp.ends_with(search) {
        (true, String::from(&temp[len-search.len()..len]))
    } else {
        (false, String::new())
    }
}

pub fn adv_starts_with(s: &String, search: &str)
    -> (bool, String) {

    let temp = s.clone();

    if temp.starts_with(search) {
        (true, String::from(&temp[..search.len()]))
    } else {
        (false, String::new())
    }
}

pub fn adv_has_alpha(s: &String)
    -> (bool, Vec<bool>) {

    has_char_property(s, CharProp::Alpha)
}

pub fn adv_has_alphanumeric(s: &String)
    -> (bool, Vec<bool>) {

    has_char_property(s, CharProp::AlphaNumeric)
}

pub fn adv_has_alphanumeric_space(s: &String)
    -> (bool, Vec<bool>) {

    has_char_property(s, CharProp::AlphaNumericSpace)
}

pub fn adv_has_alpha_space(s: &String)
    -> (bool, Vec<bool>) {

    has_char_property(s, CharProp::AlphaSpace)
}

pub fn adv_has_lowercase(s: &String)
    -> (bool, Vec<bool>) {

    has_char_property(s, CharProp::Lower)
}

pub fn adv_has_numeric(s: &String)
    -> (bool, Vec<bool>) {

    has_char_property(s, CharProp::Numeric)
}

pub fn adv_has_numeric_space(s: &String)
    -> (bool, Vec<bool>) {

    has_char_property(s, CharProp::NumericSpace)
}

pub fn adv_has_uppercase(s: &String)
    -> (bool, Vec<bool>) {

    has_char_property(s, CharProp::Upper)
}

pub fn adv_has_whitespace(s: &String)
    -> (bool, Vec<bool>) {

    has_char_property(s, CharProp::Whitespace)
}

pub fn has_alpha(s: &String)
    -> bool {

    adv_has_alpha(s).0
}

pub fn has_alphanumeric(s: &String)
    -> bool {

    adv_has_alphanumeric(s).0
}

pub fn has_alphanumeric_space(s: &String)
    -> bool {

    adv_has_alphanumeric_space(s).0
}

pub fn has_alpha_space(s: &String)
    -> bool {

    adv_has_alpha_space(s).0
}

pub fn has_lowercase(s: &String)
    -> bool {

    adv_has_lowercase(s).0
}

pub fn has_numeric(s: &String)
    -> bool {

    adv_has_numeric(s).0
}

pub fn has_numeric_space(s: &String)
    -> bool {

    adv_has_numeric_space(s).0
}

pub fn has_uppercase(s: &String)
    -> bool {

    adv_has_uppercase(s).0
}

pub fn has_whitespace(s: &String)
    -> bool {

    adv_has_whitespace(s).0
}

pub fn adv_is_alpha(s: &String)
    -> (bool, Vec<bool>) {

    is_char_property(s, CharProp::Alpha)
}

pub fn adv_is_alphanumeric(s: &String)
    -> (bool, Vec<bool>) {

    is_char_property(s, CharProp::AlphaNumeric)
}

pub fn adv_is_alphanumeric_space(s: &String)
    -> (bool, Vec<bool>) {

    is_char_property(s, CharProp::AlphaNumericSpace)
}

pub fn adv_is_alpha_space(s: &String)
    -> (bool, Vec<bool>) {

    is_char_property(s, CharProp::AlphaSpace)
}

pub fn adv_is_lowercase(s: &String)
    -> (bool, Vec<bool>) {

    is_char_property(s, CharProp::Lower)
}

pub fn adv_is_numeric(s: &String)
    -> (bool, Vec<bool>) {

    is_char_property(s, CharProp::Numeric)
}

pub fn adv_is_numeric_space(s: &String)
    -> (bool, Vec<bool>) {

    is_char_property(s, CharProp::NumericSpace)
}

pub fn adv_is_uppercase(s: &String)
    -> (bool, Vec<bool>) {

    is_char_property(s, CharProp::Upper)
}

pub fn adv_is_whitespace(s: &String)
    -> (bool, Vec<bool>) {

    is_char_property(s, CharProp::Whitespace)
}

pub fn is_alpha(s: &String)
    -> bool {

    adv_is_alpha(s).0
}

pub fn is_alphanumeric(s: &String)
    -> bool {

    adv_is_alphanumeric(s).0
}

pub fn is_alphanumeric_space(s: &String)
    -> bool {

    adv_is_alphanumeric_space(s).0
}

pub fn is_alpha_space(s: &String)
    -> bool {

    adv_is_alpha_space(s).0
}

pub fn is_lowercase(s: &String)
    -> bool {

    adv_is_lowercase(s).0
}

pub fn is_numeric(s: &String)
    -> bool {

    adv_is_numeric(s).0
}

pub fn is_numeric_space(s: &String)
    -> bool {

    adv_is_numeric_space(s).0
}

pub fn is_uppercase(s: &String)
    -> bool {

    adv_is_uppercase(s).0
}

pub fn is_whitespace(s: &String)
    -> bool {

    adv_is_whitespace(s).0
}

// pub fn adv_remove_all_regex(s: &String, regex: &str)
//     -> (bool, Vec<(usize, usize)>, String) {
//
//     let mut target = s.to_string();
//     let mut vec = Vec::<(usize, usize)>::new();
//
//     match Regex::new(regex) {
//         Ok(r) => {
//             let mut i = 0;
//             for mat in r.find_iter(s){
//                 let j = mat.0 - i;
//                 let mut k = 0;
//
//                 vec.push(mat);
//
//                 while k < mat.1 - mat.0{
//                     let n = target.remove(j).len_utf8();
//                     k = k + n;
//                     i = i + n;
//                 }
//             }
//
//             return (true, vec, target);
//         },
//         Err(_) => {
//             return (false, Vec::<(usize, usize)>::new(), String::new());
//         }
//     }
// }
//
// pub fn adv_remove_regex(s: &String, regex: &str)
//     -> (bool, usize, String) {
//
//     let mut target = s.to_string();
//     let j: usize;
//
//     match Regex::new(regex) {
//         Ok(r) => {
//             let mat = r.find(s).unwrap();
//
//             j = mat.0;
//             let mut k = 0;
//
//             while k < mat.1 - mat.0{
//                 let n = target.remove(j).len_utf8();
//                 k = k + n;
//             }
//
//             return (true, j, target);
//
//         },
//         Err(_) => { return (false, 0, String::new()); }
//     }
// }
//
// pub fn remove_all_regex(s: &String, regex: &str)
//     -> String {
//
//     let temp = adv_remove_all_regex(s, regex);
//     if temp.0 { temp.2 }
//     else { panic!("regex err") }
// }
//
// pub fn remove_all_regex_mut(s: &mut String, regex: &str)
//     -> bool {
//
//     let temp = adv_remove_all_regex(s, regex);
//
//     if temp.0 {
//         s.clear();
//         s.push_str(&temp.2);
//     }
//
//     temp.0
// }
//
// pub fn remove_regex(s: &String, regex: &str)
//     -> String {
//
//     let temp = adv_remove_regex(s, regex);
//     if temp.0 { temp.2 }
//     else { panic!("regex err") }
// }
//
// pub fn remove_regex_mut(s: &mut String, regex: &str)
//     -> bool {
//
//     let temp = adv_remove_regex(s, regex);
//
//     if temp.0 {
//         s.clear();
//         s.push_str(&temp.2);
//     }
//
//     temp.0
// }

pub fn reverse(s: &String)
    -> String {

    s.chars().rev().collect::<String>()
}

pub fn reverse_mut(s: &mut String) {

    let temp = s.reverse();
    s.clear();
    s.push_str(&temp)
}

pub fn reverse_str(s: &String)
    -> &'static str {

    reverse(s).to_str()
}

pub fn cmp_ignore_case(s: &String, cmp: &str)
    -> bool {

    s.to_lowercase() == String::from(cmp).to_lowercase()
}

pub fn difference(s: &String, diff: &str)
    -> Vec<usize> {

    let mut c1 = (*s).chars();
    let mut c2 = diff.chars();
    let mut vec = Vec::<usize>::new();
    let mut i: usize = 0;

    loop {
        let n1 = c1.next();
        let n2 = c2.next();

        if n1 != None || n2 != None {
            if n1 != n2 { vec.push(i); }
        } else { break; }

        i = i + 1;
    }

    vec
}

pub fn find_char(s: &String, search: char)
    -> usize {

    match s.find_char_opt(search) {
        Some(n) => n,
        None => panic!(format!("string doesn't contain {}", search))
    }
}

pub fn find_char_opt(s: &String, search: char)
    -> Option<usize> {

    let mut c = (*s).chars();
    let mut i: usize = 0;

    loop {
        let n = c.next();
        if n == None { return None; }
        if n == Some(search) { return Some(i); }

        i = i + 1;
    }
}

pub fn peek(s: &String)
    -> char {

    match s.peek_opt() {
        Some(n) => n,
        None => panic!("string is empty")
    }
}

pub fn peek_opt(s: &String)
    -> Option<char> {

    s.chars().last()
}
