use std::string::FromUtf8Error;
use std::iter::Iterator;
use parse::ToStr;


pub trait StringUtils {
    fn reverse_res(self) -> Result<String,FromUtf8Error>;
    fn reverse(self) -> String;
    fn adv_starts_with(self, search: &str) -> (bool, String);
    fn starts_with(self, search: &str) -> bool;
    fn adv_ends_with(self, search: &str) -> (bool, String);
    fn ends_with(self, search: &str) -> bool;
    fn cmp_ingnore_case(self, cmp: &str) -> bool;
    fn peek_opt(self) -> Option<char>;
    fn peek(self) -> char;
    fn adv_contains_any_str(self, search: &[&str]) -> (bool, usize, String);
    fn contains_any_str(self, search: &[&str]) -> bool;
    fn adv_contains_all_strs(self, search: &[&str])
        -> (bool, Vec<usize>, Vec<String>);
    fn contains_all_strs(self, search: &[&str]) -> bool;
    fn adv_contains_any_char(self, search: &[char]) -> (bool, usize, char);
    fn contains_any_char(self, search: &[char]) -> bool;
    fn adv_contains_all_chars(self, search: &[char])
        -> (bool, Vec<usize>, Vec<char>);
    fn contains_all_chars(self, search: &[char]) -> bool;
    fn adv_contains_none_str(self, search: &[&str]) -> (bool, usize, String);
    fn contains_none_str(self, search: &[&str]) -> bool;
    fn adv_contains_none_char(self, search: &[char]) -> (bool, usize, char);
    fn contains_none_char(self, search: &[char]) -> bool;
    fn difference_res(self, diff: &str)
        -> (Vec<usize>, Result<String,FromUtf8Error>);
    fn difference(self, diff: &str) -> (Vec<usize>, String);
}

impl StringUtils for &'static str {
    fn reverse_res(self) -> Result<String,FromUtf8Error> {
        let mut bytes = String::from(self).into_bytes();
        let len = bytes.len();

        for i in 0..len/2 { bytes.swap(i,len-1-i); }

        String::from_utf8(bytes)
    }

    fn reverse(self) -> String {
        match self.reverse_res(){
            Ok(n) => n,
            Err(err) => panic!("{}",err)
        }
    }

    fn adv_starts_with(self, search: &str) -> (bool, String) {
        let mut temp = String::from(self);
        let x: String = temp.drain(..search.len()).collect();
        (x == search, temp)
    }

    fn starts_with(self, search: &str) -> bool {
        self.adv_starts_with(search).0
    }

    fn adv_ends_with(self, search: &str) -> (bool, String) {
        let mut temp = String::from(self);
        let len = temp.len();
        let x: String = temp.drain(len-search.len()..len).collect();
        (x == search, temp)
    }

    fn ends_with(self, search: &str) -> bool {
        self.adv_ends_with(search).0
    }

    fn cmp_ingnore_case(self, cmp: &str) -> bool {
        String::from(self).to_lowercase() == String::from(cmp).to_lowercase()
    }

    fn peek_opt(self) -> Option<char> {
        String::from(self).chars().last()
    }

    fn peek(self) -> char{
        match self.peek_opt() {
            Some(n) => n,
            None => panic!("No more char")
        }
    }

    fn adv_contains_any_str(self, search: &[&str]) -> (bool, usize, String) {
        for i in 0..search.len() {
            match self.find(search[i]) {
                Some(n) => return (true, n, String::from(search[i])),
                None => {}
            }
        }

        (false, 0, String::new())
    }

    fn contains_any_str(self, search: &[&str]) -> bool {
        self.adv_contains_any_str(search).0
    }

    fn adv_contains_all_strs(self, search: &[&str])
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

    fn contains_all_strs(self, search: &[&str]) -> bool {
        self.adv_contains_all_strs(search).0
    }

    fn adv_contains_any_char(self, search: &[char]) -> (bool, usize, char) {
        for i in 0..search.len() {
            match self.find(search[i]) {
                Some(n) => return (true, n, search[i]),
                None => {}
            }
        }

        (false, 0, ' ')
    }

    fn contains_any_char(self, search: &[char]) -> bool {
        self.adv_contains_any_char(search).0
    }

    fn adv_contains_all_chars(self, search: &[char])
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

    fn contains_all_chars(self, search: &[char]) -> bool {
        self.adv_contains_all_chars(search).0
    }

    fn adv_contains_none_str(self, search: &[&str]) -> (bool, usize, String) {
        for i in 0..search.len() {
            match self.find(search[i]) {
                Some(_) => return (false, i, String::from(search[i])),
                None => {}
            }
        }

        (true, 0, String::new())
    }

    fn contains_none_str(self, search: &[&str]) -> bool {
        self.adv_contains_none_str(search).0
    }

    fn adv_contains_none_char(self, search: &[char]) -> (bool, usize, char) {
        for i in 0..search.len() {
            match self.find(search[i]) {
                Some(_) => return (false, i, search[i]),
                None => {}
            }
        }

        (true, 0, ' ')
    }

    fn contains_none_char(self, search: &[char]) -> bool {
        self.adv_contains_none_char(search).0
    }

    fn difference_res(self, diff: &str)
        -> (Vec<usize>, Result<String,FromUtf8Error>) {

        let bytes1 = String::from(self).into_bytes();
        let bytes2 = String::from(diff).into_bytes();
        let mut idxs = Vec::<usize>::new();
        let mut res = Vec::<u8>::new();
        let mut i: usize = 0;

        while i < bytes1.len() && i < bytes2.len() {
            if bytes1[i] != bytes2[i] {
                idxs.push(i);
                res.push(bytes2[i]);
            }

            i = i + 1;
        }

        (idxs, String::from_utf8(res))
    }

    fn difference(self, diff: &str) -> (Vec<usize>, String) {
        let temp = self.difference_res(diff);
        let idxs = temp.0;
        let s = match self.difference_res(diff).1{
            Ok(n) => n,
            Err(err) => panic!("{}",err)
        };

        (idxs, s)
    }
}

impl StringUtils for String {

    fn reverse_res(self) -> Result<String,FromUtf8Error> {
        self.to_str().reverse_res()
    }

    fn reverse(self) -> String {
        self.to_str().reverse()
    }

    fn adv_starts_with(self, search: &str) -> (bool, String) {
        self.to_str().adv_starts_with(search)
    }

    fn starts_with(self, search: &str) -> bool {
        self.to_str().starts_with(search)
    }

    fn adv_ends_with(self, search: &str) -> (bool, String) {
        self.to_str().adv_ends_with(search)
    }

    fn ends_with(self, search: &str) -> bool {
        self.to_str().ends_with(search)
    }

    fn cmp_ingnore_case(self, cmp: &str) -> bool {
        self.to_str().cmp_ingnore_case(cmp)
    }

    fn peek_opt(self) -> Option<char> {
        self.to_str().peek_opt()
    }

    fn peek(self) -> char{
        self.to_str().peek()
    }

    fn adv_contains_any_str(self, search: &[&str]) -> (bool, usize, String) {
        self.to_str().adv_contains_any_str(search)
    }

    fn contains_any_str(self, search: &[&str]) -> bool {
        self.to_str().contains_any_str(search)
    }

    fn adv_contains_all_strs(self, search: &[&str])
        -> (bool,Vec<usize>,Vec<String>) {

        self.to_str().adv_contains_all_strs(search)
    }

    fn contains_all_strs(self, search: &[&str]) -> bool {
        self.to_str().contains_all_strs(search)
    }

    fn adv_contains_any_char(self, search: &[char]) -> (bool, usize, char) {
        self.to_str().adv_contains_any_char(search)
    }

    fn contains_any_char(self, search: &[char]) -> bool {
        self.to_str().contains_any_char(search)
    }

    fn adv_contains_all_chars(self, search: &[char])
        -> (bool, Vec<usize>, Vec<char>) {

        self.to_str().adv_contains_all_chars(search)
    }

    fn contains_all_chars(self, search: &[char]) -> bool {
        self.to_str().contains_all_chars(search)
    }

    fn adv_contains_none_str(self, search: &[&str]) -> (bool, usize, String) {
        self.to_str().adv_contains_none_str(search)
    }

    fn contains_none_str(self, search: &[&str]) -> bool {
        self.to_str().contains_none_str(search)
    }

    fn adv_contains_none_char(self, search: &[char]) -> (bool, usize, char) {
        self.to_str().adv_contains_none_char(search)
    }

    fn contains_none_char(self, search: &[char]) -> bool {
        self.to_str().contains_none_char(search)
    }

    fn difference_res(self, diff: &str)
        -> (Vec<usize>, Result<String,FromUtf8Error>) {

        self.to_str().difference_res(diff)
    }

    fn difference(self, diff: &str) -> (Vec<usize>, String) {
        self.to_str().difference(diff)
    }
}
