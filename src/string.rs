use std::iter::Iterator;
use parse::ToStr;
use StringUtils;
use regex::Regex;

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
        let mut target = self.to_string();

        match Regex::new(regex) {
            Ok(r) => {
                let mut i = 0;
                for mat in r.find_iter(&self){
                    let j = mat.0 - i;
                    let mut k = 0;

                    while k < mat.1 - mat.0{
                        let n = target.remove(j).len_utf8();
                        k = k + n;
                        i = i + n;
                    }
                }
            },
            Err(err) => panic!("{}", err)
        }

        target
    }

    fn remove_all_regex_mut(&mut self, regex: &str){
        let temp = self.remove_all_regex(regex);
        self.clear();
        self.push_str(&temp);
    }

    fn remove_regex(&self, regex: &str) -> String{
        let mut target = self.to_string();

        match Regex::new(regex) {
            Ok(r) => {
                let mat = r.find(&self).unwrap();

                let j = mat.0;
                let mut k = 0;

                while k < mat.1 - mat.0{
                    let n = target.remove(j).len_utf8();
                    k = k + n;
                }

            },
            Err(err) => panic!("{}", err)
        }

        target
    }

    fn remove_regex_mut(&mut self, regex: &str){
        let temp = self.remove_regex(regex);
        self.clear();
        self.push_str(&temp)
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

impl StringUtils for &'static str {
    fn adv_contains_all_chars(&self, search: &[char])
        -> (bool, Vec<usize>, Vec<char>) {

        self.to_string().adv_contains_all_chars(search)
    }

    fn adv_contains_all_strs(&self, search: &[&str])
        -> (bool,Vec<usize>,Vec<String>) {

        self.to_string().adv_contains_all_strs(search)
    }

    fn adv_contains_any_char(&self, search: &[char]) -> (bool, usize, char) {
        self.to_string().adv_contains_any_char(search)
    }

    fn adv_contains_any_str(&self, search: &[&str]) -> (bool, usize, String) {
        self.to_string().adv_contains_any_str(search)
    }

    fn adv_contains_none_char(&self, search: &[char]) -> (bool, usize, char) {
        self.to_string().adv_contains_none_char(search)
    }

    fn adv_contains_none_str(&self, search: &[&str]) -> (bool, usize, String) {
        self.to_string().adv_contains_none_str(search)
    }

    fn adv_ends_with(&self, search: &str) -> (bool, String){
        self.to_string().adv_ends_with(search)
    }

    fn adv_starts_with(&self, search: &str) -> (bool, String) {
        self.to_string().adv_starts_with(search)
    }

    fn contains_all_chars(&self, search: &[char]) -> bool {
        self.to_string().contains_all_chars(search)
    }

    fn contains_all_strs(&self, search: &[&str]) -> bool {
        self.to_string().contains_all_strs(search)
    }

    fn contains_any_char(&self, search: &[char]) -> bool {
        self.to_string().contains_any_char(search)
    }

    fn contains_any_str(&self, search: &[&str]) -> bool {
        self.to_string().contains_any_str(search)
    }

    fn contains_none_char(&self, search: &[char]) -> bool {
        self.to_string().contains_none_char(search)
    }

    fn contains_none_str(&self, search: &[&str]) -> bool {
        self.to_string().contains_none_str(search)
    }

    fn cmp_ignore_case(&self, cmp: &str) -> bool {
        self.to_string().cmp_ignore_case(cmp)
    }

    fn difference(&self, diff: &str) -> Vec<usize>{
        self.to_string().difference(diff)
    }

    fn find_char(&self, search: char) -> usize {
        self.to_string().find_char(search)
    }

    fn find_char_opt(&self, search: char) -> Option<usize> {
        self.to_string().find_char_opt(search)
    }

    fn peek(&self) -> char {
        self.to_string().peek()
    }

    fn peek_opt(&self) -> Option<char> {
        self.to_string().peek_opt()
    }

    fn remove_all_regex(&self, regex: &str) -> String {
        self.to_string().remove_all_regex(regex)
    }

    fn remove_all_regex_mut(&mut self, regex: &str){
        unimplemented!()
    }

    fn remove_regex(&self, regex: &str) -> String{
        self.to_string().remove_regex(regex)
    }

    fn remove_regex_mut(&mut self, regex: &str){
        unimplemented!()
    }

    fn reverse(&self) -> String {
        self.to_string().reverse()
    }

    fn reverse_mut(&mut self) {
        unimplemented!();
    }

    fn reverse_str(&self) -> &'static str{
        self.to_string().reverse_str()
    }
}
