use std::iter::Iterator;
use parse::ToStr;
use StringUtils;
use regex::Regex;
use CharProp;
use has_char_property;
use is_char_property;

impl StringUtils for String {

    fn adv_contains_all_chars(&self, search: &[char])
        -> (bool, Vec<usize>, Vec<char>) {

        let mut idxs = Vec::<usize>::new();
        let mut chars = Vec::<char>::new();

        for i in 0..search.len() {
            match self.find(search[i]) {
                Some(n) => {
                    idxs.push(n);
                    chars.push(search[i]);
                },
                None => return (false, idxs, chars)
            }
        }

        (true, idxs, chars)
    }

    fn adv_contains_all_strs(&self, search: &[&str])
        -> (bool,Vec<usize>,Vec<String>) {

        let mut idxs = Vec::<usize>::new();
        let mut strs = Vec::<String>::new();

        for i in 0..search.len() {
            match self.find(search[i]) {
                Some(n) => {
                    idxs.push(n);
                    strs.push(String::from(search[i]));
                },
                None => return (false, idxs, strs)
            }
        }

        (true, idxs, strs)
    }

    fn adv_contains_any_char(&self, search: &[char]) -> (bool, usize, char) {
        for i in 0..search.len() {
            match self.find(search[i]) {
                Some(n) => return (true, n, search[i]),
                None => {}
            }
        }

        (false, 0, ' ')
    }

    fn adv_contains_any_str(&self, search: &[&str]) -> (bool, usize, String) {
        for i in 0..search.len() {
            match self.find(search[i]) {
                Some(n) => return (true, n, String::from(search[i])),
                None => {}
            }
        }

        (false, 0, String::new())
    }

    fn adv_contains_none_char(&self, search: &[char]) -> (bool, usize, char) {
        for i in 0..search.len() {
            match self.find(search[i]) {
                Some(_) => return (false, i, search[i]),
                None => {}
            }
        }

        (true, 0, ' ')
    }

    fn adv_contains_none_str(&self, search: &[&str]) -> (bool, usize, String) {
        for i in 0..search.len() {
            match self.find(search[i]) {
                Some(_) => return (false, i, String::from(search[i])),
                None => {}
            }
        }

        (true, 0, String::new())
    }

    fn adv_ends_with(&self, search: &str) -> (bool, String){
        let mut temp = self.clone();
        let len = temp.len();

        if temp.ends_with(search) {
            temp.drain(len-search.len()..len).collect::<String>();
            (true, temp)
        } else {
            (false, String::new())
        }
    }

    fn adv_has_alpha(&self) -> (bool, Vec<bool>) {
        has_char_property(self, CharProp::Alpha)
    }

    fn adv_has_alphanumeric(&self) -> (bool, Vec<bool>) {
        has_char_property(self, CharProp::AlphaNumeric)
    }

    fn adv_has_alphanumeric_space(&self) -> (bool, Vec<bool>) {
        has_char_property(self, CharProp::AlphaNumericSpace)
    }

    fn adv_has_alpha_space(&self) -> (bool, Vec<bool>) {
        has_char_property(self, CharProp::AlphaSpace)
    }


    fn adv_has_lowercase(&self) -> (bool, Vec<bool>) {
        has_char_property(self, CharProp::Lower)
    }

    fn adv_has_numeric(&self) -> (bool, Vec<bool>) {
        has_char_property(self, CharProp::Numeric)
    }

    fn adv_has_uppercase(&self) -> (bool, Vec<bool>) {
        has_char_property(self, CharProp::Upper)
    }

    fn adv_has_whitespace(&self) -> (bool, Vec<bool>) {
        has_char_property(self, CharProp::Whitespace)
    }

    fn adv_is_alpha(&self) -> (bool, Vec<bool>) {
        is_char_property(self, CharProp::Alpha)
    }

    fn adv_is_alphanumeric(&self) -> (bool, Vec<bool>) {
        is_char_property(self, CharProp::AlphaNumeric)
    }

    fn adv_is_numeric(&self) -> (bool, Vec<bool>) {
        is_char_property(self, CharProp::Numeric)
    }

    fn adv_is_lowercase(&self) -> (bool, Vec<bool>) {
        is_char_property(self, CharProp::Lower)
    }

    fn adv_is_uppercase(&self) -> (bool, Vec<bool>) {
        is_char_property(self, CharProp::Upper)
    }

    fn adv_is_whitespace(&self) -> (bool, Vec<bool>) {
        is_char_property(self, CharProp::Whitespace)
    }

    fn adv_remove_all_regex(&self, regex: &str) -> (bool, Vec<(usize, usize)>, String) {
        let mut target = self.to_string();
        let mut vec = Vec::<(usize, usize)>::new();

        match Regex::new(regex) {
            Ok(r) => {
                let mut i = 0;
                for mat in r.find_iter(&self){
                    let j = mat.0 - i;
                    let mut k = 0;

                    vec.push(mat);

                    while k < mat.1 - mat.0{
                        let n = target.remove(j).len_utf8();
                        k = k + n;
                        i = i + n;
                    }
                }

                return (true, vec, target);
            },
            Err(_) => {
                return (false, Vec::<(usize, usize)>::new(), String::new());
            }
        }
    }

    fn adv_remove_regex(&self, regex: &str) -> (bool, usize, String) {
        let mut target = self.to_string();
        let j: usize;

        match Regex::new(regex) {
            Ok(r) => {
                let mat = r.find(&self).unwrap();

                j = mat.0;
                let mut k = 0;

                while k < mat.1 - mat.0{
                    let n = target.remove(j).len_utf8();
                    k = k + n;
                }

                return (true, j, target);

            },
            Err(_) => { return (false, 0, String::new()); }
        }
    }

    fn adv_starts_with(&self, search: &str) -> (bool, String) {
        let mut temp = self.clone();

        if temp.starts_with(search) {
            temp.drain(..search.len()).collect::<String>();
            (true, temp)
        } else {
            (false, String::new())
        }
    }

    fn contains_all_chars(&self, search: &[char]) -> bool {
        self.adv_contains_all_chars(search).0
    }

    fn contains_all_strs(&self, search: &[&str]) -> bool {
        self.adv_contains_all_strs(search).0
    }

    fn contains_any_char(&self, search: &[char]) -> bool {
        self.adv_contains_any_char(search).0
    }

    fn contains_any_str(&self, search: &[&str]) -> bool {
        self.adv_contains_any_str(search).0
    }

    fn contains_none_char(&self, search: &[char]) -> bool {
        self.adv_contains_none_char(search).0
    }

    fn contains_none_str(&self, search: &[&str]) -> bool {
        self.adv_contains_none_str(search).0
    }

    fn cmp_ignore_case(&self, cmp: &str) -> bool{
        self.to_lowercase() == String::from(cmp).to_lowercase()
    }

    fn difference(&self, diff: &str) -> Vec<usize>{
        let mut c1 = (*self).chars();
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

    fn find_char(&self, search: char) -> usize {
        match self.find_char_opt(search) {
            Some(n) => n,
            None => panic!(format!("string doesn't contain {}", search))
        }
    }

    fn find_char_opt(&self, search: char) -> Option<usize> {
        let mut c = (*self).chars();
        let mut i: usize = 0;

        loop {
            let n = c.next();
            if n == None { return None; }
            if n == Some(search) { return Some(i); }

            i = i + 1;
        }
    }

    fn has_alpha(&self) -> bool {
        self.adv_has_alpha().0
    }

    fn has_alphanumeric(&self) -> bool {
        self.adv_has_alphanumeric().0
    }

    fn has_lowercase(&self) -> bool {
        self.adv_has_lowercase().0
    }

    fn has_numeric(&self) -> bool {
        self.adv_has_numeric().0
    }

    fn has_uppercase(&self) -> bool {
        self.adv_has_uppercase().0
    }

    fn has_whitespace(&self) -> bool {
        self.adv_has_whitespace().0
    }

    fn is_alpha(&self) -> bool {
        self.adv_is_alpha().0
    }

    fn is_alphanumeric(&self) -> bool {
        self.adv_is_alphanumeric().0
    }

    fn is_lowercase(&self) -> bool {
        self.adv_is_lowercase().0
    }

    fn is_numeric(&self) -> bool {
        self.adv_is_numeric().0
    }

    fn is_uppercase(&self) -> bool {
        self.adv_is_lowercase().0
    }

    fn is_whitespace(&self) -> bool {
        self.adv_is_whitespace().0
    }

    fn peek(&self) -> char{
        match self.peek_opt() {
            Some(n) => n,
            None => panic!("string is empty")
        }
    }

    fn peek_opt(&self) -> Option<char> {
        self.chars().last()
    }

    fn remove_all_regex(&self, regex: &str) -> String {
        let temp = self.adv_remove_all_regex(regex);
        if temp.0 { temp.2 }
        else { panic!("regex err") }
    }

    fn remove_all_regex_mut(&mut self, regex: &str) -> bool {
        let temp = self.adv_remove_all_regex(regex);

        if temp.0 {
            self.clear();
            self.push_str(&temp.2);
        }

        temp.0
    }

    fn remove_regex(&self, regex: &str) -> String {
        let temp = self.adv_remove_regex(regex);
        if temp.0 { temp.2 }
        else { panic!("regex err") }
    }

    fn remove_regex_mut(&mut self, regex: &str) -> bool{
        let temp = self.adv_remove_regex(regex);

        if temp.0 {
            self.clear();
            self.push_str(&temp.2);
        }

        temp.0
    }

    fn reverse(&self) -> String {
        self.chars().rev().collect::<String>()
    }

    fn reverse_mut(&mut self) {
        let temp = self.reverse();
        self.clear();
        self.push_str(&temp)
    }

    fn reverse_str(&self) -> &'static str{
        self.reverse().to_str()
    }
}
