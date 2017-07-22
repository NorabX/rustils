// <editor-fold> # Uses

use string;
// </editor-fold>

// <editor-fold> # Impl

// <editor-fold> ## StringUtils

impl string::StringUtils for String {

    fn adv_contains_all_chars(&self, search: &[char])
        -> (bool, Vec<usize>, Vec<char>) {

        string::adv_contains_all_chars(self, search)
    }

    fn adv_contains_all_strs(&self, search: &[&str])
        -> (bool, Vec<usize>, Vec<String>) {

        string::adv_contains_all_strs(self, search)
    }

    fn adv_contains_any_char(&self, search: &[char])
        -> (bool, usize, char) {

        string::adv_contains_any_char(self, search)
    }

    fn adv_contains_any_str(&self, search: &[&str])
        -> (bool, usize, String) {

        string::adv_contains_any_str(self, search)
    }

    fn adv_contains_none_char(&self, search: &[char])
        -> (bool, usize, char) {

        string::adv_contains_none_char(self, search)
    }

    fn adv_contains_none_str(&self, search: &[&str])
        -> (bool, usize, String) {

        string::adv_contains_none_str(self, search)
    }

    fn adv_ends_with(&self, search: &str)
        -> (bool, String) {

        string::adv_ends_with(self, search)
    }

    fn adv_has_alpha(&self)
        -> (bool, Vec<bool>) {

        string::adv_has_alpha(self)
    }

    fn adv_has_alphanumeric(&self)
        -> (bool, Vec<bool>) {

        string::adv_has_alphanumeric(self)
    }

    fn adv_has_alphanumeric_space(&self)
        -> (bool, Vec<bool>) {

        string::adv_has_alphanumeric_space(self)
    }

    fn adv_has_alpha_space(&self)
        -> (bool, Vec<bool>) {

        string::adv_has_alpha_space(self)
    }

    fn adv_has_lowercase(&self)
        -> (bool, Vec<bool>) {

        string::adv_has_lowercase(self)
    }

    fn adv_has_numeric(&self)
        -> (bool, Vec<bool>) {

        string::adv_has_numeric(self)
    }

    fn adv_has_numeric_space(&self)
        -> (bool, Vec<bool>) {

        string::adv_has_numeric_space(self)
    }

    fn adv_has_uppercase(&self)
        -> (bool, Vec<bool>) {

        string::adv_has_uppercase(self)
    }

    fn adv_has_whitespace(&self)
        -> (bool, Vec<bool>) {

        string::adv_has_whitespace(self)
    }

    fn adv_is_alpha(&self)
        -> (bool, Vec<bool>) {

        string::adv_is_alpha(self)
    }

    fn adv_is_alphanumeric(&self)
        -> (bool, Vec<bool>) {

        string::adv_is_alphanumeric(self)
    }

    fn adv_is_alphanumeric_space(&self)
        -> (bool, Vec<bool>) {

        string::adv_is_alphanumeric_space(self)
    }

    fn adv_is_alpha_space(&self)
        -> (bool, Vec<bool>) {

        string::adv_is_alpha_space(self)
    }

    fn adv_is_lowercase(&self)
        -> (bool, Vec<bool>) {

        string::adv_is_lowercase(self)
    }

    fn adv_is_numeric(&self)
        -> (bool, Vec<bool>) {

        string::adv_is_numeric(self)
    }

    fn adv_is_numeric_space(&self)
        -> (bool, Vec<bool>) {

        string::adv_is_numeric_space(self)
    }

    fn adv_is_uppercase(&self)
        -> (bool, Vec<bool>) {

        string::adv_is_uppercase(self)
    }

    fn adv_is_whitespace(&self)
        -> (bool, Vec<bool>) {

        string::adv_is_whitespace(self)
    }

    fn adv_remove_all_regex(&self, regex: &str)
        -> (bool, Vec<(usize, usize)>, String) {

        string::adv_remove_all_regex(self, regex)
    }

    fn adv_remove_regex(&self, regex: &str)
        -> (bool, usize, String) {

        string::adv_remove_regex(self, regex)
    }

    fn adv_starts_with(&self, search: &str)
        -> (bool, String) {

        string::adv_starts_with(self, search)
    }

    fn contains_all_chars(&self, search: &[char])
        -> bool {

        string::contains_all_chars(self, search)
    }

    fn contains_all_strs(&self, search: &[&str])
        -> bool {

        string::contains_all_strs(self, search)
    }

    fn contains_any_char(&self, search: &[char])
        -> bool {

        string::contains_any_char(self, search)
    }

    fn contains_any_str(&self, search: &[&str])
        -> bool {

        string::contains_any_str(self, search)
    }

    fn contains_none_char(&self, search: &[char])
        -> bool {

        string::contains_none_char(self, search)
    }

    fn contains_none_str(&self, search: &[&str])
        -> bool {

        string::contains_none_str(self, search)
    }

    fn cmp_ignore_case(&self, cmp: &str)
        -> bool {

        string::cmp_ignore_case(self, cmp)
    }

    fn difference(&self, diff: &str)
        -> Vec<usize> {

        string::difference(self, diff)
    }

    fn find_char(&self, search: char)
        -> usize {

        string::find_char(self, search)
    }

    fn find_char_opt(&self, search: char)
        -> Option<usize> {

        string::find_char_opt(self, search)
    }

    fn has_alpha(&self)
        -> bool {

        string::has_alpha(self)
    }

    fn has_alphanumeric(&self)
        -> bool {

        string::has_alphanumeric(self)
    }

    fn has_alphanumeric_space(&self)
        -> bool {

        string::has_alphanumeric_space(self)
    }

    fn has_alpha_space(&self)
        -> bool {

        string::has_alpha_space(self)
    }

    fn has_lowercase(&self)
        -> bool {

        string::has_lowercase(self)
    }

    fn has_numeric(&self)
        -> bool {

        string::has_numeric(self)
    }

    fn has_numeric_space(&self)
        -> bool {

        string::has_numeric_space(self)
    }

    fn has_uppercase(&self)
        -> bool {

        string::has_uppercase(self)
    }

    fn has_whitespace(&self)
        -> bool {

        string::has_whitespace(self)
    }

    fn is_alpha(&self)
        -> bool {

        string::is_alpha(self)
    }

    fn is_alphanumeric(&self)
        -> bool {

        string::is_alphanumeric(self)
    }

    fn is_alphanumeric_space(&self)
        -> bool {

        string::is_alphanumeric_space(self)
    }

    fn is_alpha_space(&self)
        -> bool {

        string::is_alpha_space(self)
    }

    fn is_lowercase(&self)
        -> bool {

        string::is_lowercase(self)
    }

    fn is_numeric(&self)
        -> bool {

        string::is_numeric(self)
    }

    fn is_numeric_space(&self)
        -> bool {

        string::is_numeric_space(self)
    }

    fn is_uppercase(&self)
        -> bool {

        string::is_uppercase(self)
    }

    fn is_whitespace(&self)
        -> bool {

        string::is_whitespace(self)
    }

    fn peek(&self)
        -> char {

        string::peek(self)
    }

    fn peek_opt(&self)
        -> Option<char> {

        string::peek_opt(self)
    }

    fn remove_all_regex(&self, regex: &str)
        -> String {

        string::remove_all_regex(self, regex)
    }

    fn remove_all_regex_mut(&mut self, regex: &str)
        -> bool {

        string::remove_all_regex_mut(self, regex)
    }

    fn remove_regex(&self, regex: &str)
        -> String {

        string::remove_regex(self, regex)
    }

    fn remove_regex_mut(&mut self, regex: &str)
        -> bool {

        string::remove_regex_mut(self, regex)
    }

    fn reverse(&self)
        -> String {

        string::reverse(self)
    }

    fn reverse_mut(&mut self) {

        string::reverse_mut(self)
    }

    fn reverse_str(&self)
        -> &'static str {

        string::reverse_str(self)
    }
}
// </editor-fold>

// </editor-fold>
