use std::iter::Iterator;
use parse::ToStr;
use StringUtils;

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

    fn peek(&self) -> char{
        match self.peek_opt() {
            Some(n) => n,
            None => panic!("string is empty")
        }
    }

    fn peek_opt(&self) -> Option<char> {
        self.chars().last()
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

    fn peek(&self) -> char {
        self.to_string().peek()
    }

    fn peek_opt(&self) -> Option<char> {
        self.to_string().peek_opt()
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
