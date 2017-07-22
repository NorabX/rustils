// <editor-fold> # Uses

use string;
// </editor-fold>

// <editor-fold> # Impls

// <editor-fold> ## StringUtils

impl string::StringUtils for &'static str {

    fn adv_contains_all_chars(&self, search: &[char])
        -> (bool, Vec<usize>, Vec<char>) {

        string::adv_contains_all_chars(&self.to_string(), search)
    }

    fn adv_contains_all_strs(&self, search: &[&str])
        -> (bool, Vec<usize>, Vec<String>) {

        string::adv_contains_all_strs(&self.to_string(), search)
    }

    fn adv_contains_any_char(&self, search: &[char])
        -> (bool, usize, char) {

        string::adv_contains_any_char(&self.to_string(), search)
    }

    fn adv_contains_any_str(&self, search: &[&str])
        -> (bool, usize, String) {

        string::adv_contains_any_str(&self.to_string(), search)
    }

    fn adv_contains_none_char(&self, search: &[char])
        -> (bool, usize, char) {

        string::adv_contains_none_char(&self.to_string(), search)
    }

    fn adv_contains_none_str(&self, search: &[&str])
        -> (bool, usize, String) {

        string::adv_contains_none_str(&self.to_string(), search)
    }

    fn adv_ends_with(&self, search: &str)
        -> (bool, String) {

        string::adv_ends_with(&self.to_string(), search)
    }

    fn adv_has_alpha(&self)
        -> (bool, Vec<bool>) {

        string::adv_has_alpha(&self.to_string())
    }

    fn adv_has_alphanumeric(&self)
        -> (bool, Vec<bool>) {

        string::adv_has_alphanumeric(&self.to_string())
    }

    fn adv_has_alphanumeric_space(&self)
        -> (bool, Vec<bool>) {

        string::adv_has_alphanumeric_space(&self.to_string())
    }

    fn adv_has_alpha_space(&self)
        -> (bool, Vec<bool>) {

        string::adv_has_alpha_space(&self.to_string())
    }

    fn adv_has_lowercase(&self)
        -> (bool, Vec<bool>) {

        string::adv_has_lowercase(&self.to_string())
    }

    fn adv_has_numeric(&self)
        -> (bool, Vec<bool>) {

        string::adv_has_numeric(&self.to_string())
    }

    fn adv_has_numeric_space(&self)
        -> (bool, Vec<bool>) {

        string::adv_has_numeric_space(&self.to_string())
    }

    fn adv_has_uppercase(&self)
        -> (bool, Vec<bool>) {

        string::adv_has_uppercase(&self.to_string())
    }

    fn adv_has_whitespace(&self)
        -> (bool, Vec<bool>) {

        string::adv_has_whitespace(&self.to_string())
    }

    fn adv_is_alpha(&self)
        -> (bool, Vec<bool>) {

        string::adv_is_alpha(&self.to_string())
    }

    fn adv_is_alphanumeric(&self)
        -> (bool, Vec<bool>) {

        string::adv_is_alphanumeric(&self.to_string())
    }

    fn adv_is_alphanumeric_space(&self)
        -> (bool, Vec<bool>) {

        string::adv_is_alphanumeric_space(&self.to_string())
    }

    fn adv_is_alpha_space(&self)
        -> (bool, Vec<bool>) {

        string::adv_is_alpha_space(&self.to_string())
    }

    fn adv_is_lowercase(&self)
        -> (bool, Vec<bool>) {

        string::adv_is_lowercase(&self.to_string())
    }

    fn adv_is_numeric(&self)
        -> (bool, Vec<bool>) {

        string::adv_is_numeric(&self.to_string())
    }

    fn adv_is_numeric_space(&self)
        -> (bool, Vec<bool>) {

        string::adv_is_numeric_space(&self.to_string())
    }

    fn adv_is_uppercase(&self)
        -> (bool, Vec<bool>) {

        string::adv_is_uppercase(&self.to_string())
    }

    fn adv_is_whitespace(&self)
        -> (bool, Vec<bool>) {

        string::adv_is_whitespace(&self.to_string())
    }

    fn adv_remove_all_regex(&self, regex: &str)
        -> (bool, Vec<(usize, usize)>, String) {

        string::adv_remove_all_regex(&self.to_string(), regex)
    }

    fn adv_remove_regex(&self, regex: &str)
        -> (bool, usize, String) {

        string::adv_remove_regex(&self.to_string(), regex)
    }

    fn adv_starts_with(&self, search: &str)
        -> (bool, String) {

        string::adv_starts_with(&self.to_string(), search)
    }

    fn contains_all_chars(&self, search: &[char])
        -> bool {

        string::contains_all_chars(&self.to_string(), search)
    }

    fn contains_all_strs(&self, search: &[&str])
        -> bool {

        string::contains_all_strs(&self.to_string(), search)
    }

    fn contains_any_char(&self, search: &[char])
        -> bool {

        string::contains_any_char(&self.to_string(), search)
    }

    fn contains_any_str(&self, search: &[&str])
        -> bool {

        string::contains_any_str(&self.to_string(), search)
    }

    fn contains_none_char(&self, search: &[char])
        -> bool {

        string::contains_none_char(&self.to_string(), search)
    }

    fn contains_none_str(&self, search: &[&str])
        -> bool {

        string::contains_none_str(&self.to_string(), search)
    }

    fn cmp_ignore_case(&self, cmp: &str)
        -> bool {

        string::cmp_ignore_case(&self.to_string(), cmp)
    }

    fn difference(&self, diff: &str)
        -> Vec<usize> {

        string::difference(&self.to_string(), diff)
    }

    fn find_char(&self, search: char)
        -> usize {

        string::find_char(&self.to_string(), search)
    }

    fn find_char_opt(&self, search: char)
        -> Option<usize> {

        string::find_char_opt(&self.to_string(), search)
    }

    fn has_alpha(&self)
        -> bool {

        string::has_alpha(&self.to_string())
    }

    fn has_alphanumeric(&self)
        -> bool {

        string::has_alphanumeric(&self.to_string())
    }

    fn has_alphanumeric_space(&self)
        -> bool {

        string::has_alphanumeric_space(&self.to_string())
    }

    fn has_alpha_space(&self)
        -> bool {

        string::has_alpha_space(&self.to_string())
    }

    fn has_lowercase(&self)
        -> bool {

        string::has_lowercase(&self.to_string())
    }

    fn has_numeric(&self)
        -> bool {

        string::has_numeric(&self.to_string())
    }

    fn has_numeric_space(&self)
        -> bool {

        string::has_numeric_space(&self.to_string())
    }

    fn has_uppercase(&self)
        -> bool {

        string::has_uppercase(&self.to_string())
    }

    fn has_whitespace(&self)
        -> bool {

        string::has_whitespace(&self.to_string())
    }

    fn is_alpha(&self)
        -> bool {

        string::is_alpha(&self.to_string())
    }

    fn is_alphanumeric(&self)
        -> bool {

        string::is_alphanumeric(&self.to_string())
    }

    fn is_alphanumeric_space(&self)
        -> bool {

        string::is_alphanumeric_space(&self.to_string())
    }

    fn is_alpha_space(&self)
        -> bool {

        string::is_alpha_space(&self.to_string())
    }

    fn is_lowercase(&self)
        -> bool {

        string::is_lowercase(&self.to_string())
    }

    fn is_numeric(&self)
        -> bool {

        string::is_numeric(&self.to_string())
    }

    fn is_numeric_space(&self)
        -> bool {

        string::is_numeric_space(&self.to_string())
    }

    fn is_uppercase(&self)
        -> bool {

        string::is_uppercase(&self.to_string())
    }

    fn is_whitespace(&self)
        -> bool {

        string::is_whitespace(&self.to_string())
    }

    fn peek(&self)
        -> char {

        string::peek(&self.to_string())
    }

    fn peek_opt(&self)
        -> Option<char> {

        string::peek_opt(&self.to_string())
    }

    fn remove_all_regex(&self, regex: &str)
        -> String {

        string::remove_all_regex(&self.to_string(), regex)
    }

    fn remove_all_regex_mut(&mut self, _regex: &str)
        -> bool {

        unimplemented!()
    }

    fn remove_regex(&self, regex: &str)
        -> String {

        string::remove_regex(&self.to_string(), regex)
    }

    fn remove_regex_mut(&mut self, _regex: &str)
        -> bool {

        unimplemented!()
    }

    fn reverse(&self)
        -> String {

        string::reverse(&self.to_string())
    }

    fn reverse_mut(&mut self) {

        unimplemented!()
    }

    fn reverse_str(&self)
        -> &'static str {

        string::reverse_str(&self.to_string())
    }
}
// </editor-fold>

// </editor-fold>
