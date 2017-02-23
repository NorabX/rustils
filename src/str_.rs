use StringUtils;
use CharProp;
use has_char_property;
use is_char_property;

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

    fn adv_has_numeric_space(&self) -> (bool, Vec<bool>) {
        has_char_property(self, CharProp::NumericSpace)
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

    fn adv_is_alphanumeric_space(&self) -> (bool, Vec<bool>) {
        is_char_property(self, CharProp::AlphaNumericSpace)
    }

    fn adv_is_alpha_space(&self) -> (bool, Vec<bool>) {
        is_char_property(self, CharProp::AlphaSpace)
    }

    fn adv_is_lowercase(&self) -> (bool, Vec<bool>) {
        is_char_property(self, CharProp::Lower)
    }

    fn adv_is_numeric(&self) -> (bool, Vec<bool>) {
        is_char_property(self, CharProp::Numeric)
    }

    fn adv_is_numeric_space(&self) -> (bool, Vec<bool>) {
        is_char_property(self, CharProp::NumericSpace)
    }

    fn adv_is_uppercase(&self) -> (bool, Vec<bool>) {
        is_char_property(self, CharProp::Upper)
    }

    fn adv_is_whitespace(&self) -> (bool, Vec<bool>) {
        is_char_property(self, CharProp::Whitespace)
    }

    fn adv_starts_with(&self, search: &str) -> (bool, String) {
        self.to_string().adv_starts_with(search)
    }

    fn adv_remove_all_regex(&self, regex: &str) -> (bool, Vec<(usize, usize)>, String) {
        self.to_string().adv_remove_all_regex(regex)
    }

    fn adv_remove_regex(&self, regex: &str) -> (bool, usize, String) {
        self.to_string().adv_remove_regex(regex)
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

    fn has_alpha(&self) -> bool {
        self.adv_has_alpha().0
    }

    fn has_alphanumeric(&self) -> bool {
        self.adv_has_alphanumeric().0
    }

    fn has_alphanumeric_space(&self) -> bool {
        self.adv_has_alphanumeric_space().0
    }

    fn has_alpha_space(&self) -> bool {
        self.adv_has_alpha_space().0
    }

    fn has_lowercase(&self) -> bool {
        self.adv_has_lowercase().0
    }

    fn has_numeric(&self) -> bool {
        self.adv_has_numeric().0
    }

    fn has_numeric_space(&self) -> bool {
        self.adv_has_numeric_space().0
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

    fn is_alphanumeric_space(&self) -> bool {
        self.adv_is_alphanumeric_space().0
    }

    fn is_alpha_space(&self) -> bool {
        self.adv_is_alpha_space().0
    }

    fn is_lowercase(&self) -> bool {
        self.adv_is_lowercase().0
    }

    fn is_numeric(&self) -> bool {
        self.adv_is_numeric().0
    }

    fn is_numeric_space(&self) -> bool {
        self.adv_is_numeric_space().0
    }

    fn is_uppercase(&self) -> bool {
        self.adv_is_uppercase().0
    }

    fn is_whitespace(&self) -> bool {
        self.adv_is_whitespace().0
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

    fn remove_all_regex_mut(&mut self, _regex: &str) -> bool {
        unimplemented!()
    }

    fn remove_regex(&self, regex: &str) -> String {
        self.to_string().remove_regex(regex)
    }

    fn remove_regex_mut(&mut self, _regex: &str) -> bool {
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
