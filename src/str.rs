/*
use std::iter::Iterator;
use parse::ToStr;
use StringUtils;
use regex::Regex;


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

    fn adv_remove_regex(&self, regex: &str) -> (bool, usize, String) {
        unimplemented!()
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
*/
