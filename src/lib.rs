/*!

# Usage

This crate is [on crates.io](https://crates.io/crates/rustils/) and can be
used by adding `rustils` to your dependencies in your project's `Cargo.toml`.

```toml
[dependencies]
rustils = "0.1.22"
```

and this to your crate root:

```rust
extern crate rustils;
```

# Examples

```
use rustils::parse::byte::ToI8;
use rustils::error::ParseError::InvalidNumber;

let a = -128_i32;
let b = 127_i32;
let c = -129_i32;
let d = 128_i32;

//Rust
assert_eq!(a as i8, -128_i8);
assert_eq!(b as i8, 127_i8);
assert_eq!(c as i8, 127_i8);
assert_eq!(d as i8, -128_i8);

//rustils
assert_eq!(a.to_i8(), -128_i8);
assert_eq!(b.to_i8(), 127_i8);
assert_eq!(c.to_i8_res(), Err(InvalidNumber("-129".to_string())));
assert_eq!(d.to_i8_res(), Err(InvalidNumber("128".to_string())));
```

```
use rustils::string::StringUtils;

let text = "你好。How are you?";

//Rust function
assert_eq!(text.find('好'), Some(3));

//rustils functions
assert_eq!(text.find_char_opt('好'), Some(1));
assert_eq!(text.find_char('好'), 1);
```

```
use rustils::string::StringUtils;

let text1 = &mut String::from("你好。How are you?");

//Rust function
assert_eq!(text1.remove(3), '好');
assert_eq!(text1, "你。How are you?");

let text3 = String::from("你好。How are you?");
let text4 = &mut String::from("你好。How are you?");
let regex = r"[aeiou]+|[好]+";

//rustils functions
assert_eq!(text3.remove_regex(regex), String::from("你。How are you?"));
assert_eq!(text3.remove_all_regex(regex), String::from("你。Hw r y?"));

assert_eq!(true, text4.remove_regex_mut(regex));
assert_eq!(text4, "你。How are you?");

assert_eq!(true, text4.remove_all_regex_mut(regex));
assert_eq!(text4, "你。Hw r y?");
```
*/

extern crate rand;
extern crate regex;
extern crate core;

#[doc(hidden)] pub mod impls;
#[doc(hidden)] pub mod boolean;

/// Array manipulation
pub mod array;
pub mod error;

/// Parsing primitives to others
pub mod parse;
pub mod random;
pub mod sorting;

/// String manipulation
pub mod string;

pub enum RoundingMode { Trunc, Round, Ceil, Floor }

#[derive(Debug)]
enum CharProp { Alpha, AlphaNumeric, AlphaNumericSpace, AlphaSpace, Lower, Numeric, NumericSpace, Upper, Whitespace }

fn char_property(s: &str, prop: CharProp, logic: bool) -> (bool, Vec<bool>) {
    let mut b = logic;

    let mut c = (*s).chars();
    let mut vec = Vec::<bool>::new();

    loop {
        let n = c.next();
        if n == None { break; }
        else {
            let nu = n.unwrap();
            let temp = match prop {
                CharProp::Alpha => nu.is_alphabetic(),
                CharProp::AlphaNumeric => nu.is_alphanumeric(),
                CharProp::AlphaNumericSpace => nu.is_alphanumeric() || nu.is_whitespace(),
                CharProp::AlphaSpace => nu.is_alphabetic() || nu.is_whitespace(),
                CharProp::Lower => nu.is_lowercase(),
                CharProp::Numeric => nu.is_numeric(),
                CharProp::NumericSpace => nu.is_numeric() || nu.is_whitespace(),
                CharProp::Upper => nu.is_uppercase(),
                CharProp::Whitespace => nu.is_whitespace()
            };

            if logic { b &= temp; }
            else { b |= temp; }

            vec.push(temp);
        }
    }

    (b, vec)
}

fn has_char_property(s: &str, prop: CharProp) -> (bool, Vec<bool>){
    char_property(s, prop, false)
}

fn is_char_property(s: &str, prop: CharProp) -> (bool, Vec<bool>) {
    char_property(s, prop, true)
}
