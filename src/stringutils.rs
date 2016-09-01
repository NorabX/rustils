use std::mem;
use std::string::FromUtf8Error;
use std::iter::Iterator;
//use regex::Regex;

/// Return a reverse string from the given string `s`.
/// Returning Err if `s` contains any invalid data.
///
/// # Example
///
/// ```
/// let x = rustils::stringutils::reverse("Hello");
/// assert_eq!(String::from("olleH"), x.unwrap());
/// ```
pub fn reverse(s: &str) -> Result<String,FromUtf8Error>{
    let mut bytes = String::from(s).into_bytes();
    let len = bytes.len();

    for i in 0..len/2{
        let l = len-1-i;
        let temp = bytes[l];
        bytes[l] = bytes[i];
        bytes[i] = temp;
    }

    String::from_utf8(bytes)
}

/// Return a reverse string from the given string `s`.
/// `panic!` by any error.
///
/// # Example
///
/// ```
/// let x = rustils::stringutils::reversep("Hello");
/// assert_eq!(String::from("olleH"),x);
/// ```
pub fn reversep(s: &str) -> String{
    match reverse(s){
        Ok(n) => n,
        Err(err) => panic!("{}",err)
    }
}

pub fn adv_starts_with(target: &str, search: &str) -> (bool,String) {
    let mut temp = String::from(target);
    let x: String = temp.drain(..search.len()).collect();
    (x == search, temp)
}

pub fn starts_with(target: &str, search: &str) -> bool {
    adv_starts_with(target,search).0
}

pub fn adv_ends_with(target: &str, search: &str) -> (bool,String) {
    let mut temp = String::from(target);
    let len = temp.len();
    let x: String = temp.drain(len-search.len()..len).collect();
    (x == search, temp)
}

pub fn ends_with(target: &str, search: &str) -> bool {
    adv_ends_with(target,search).0
}

pub fn cmp_ingnore_case(str1: &str, str2: &str) -> bool {
    String::from(str1).to_lowercase() == String::from(str2).to_lowercase()
}

pub fn peek(str1: &str) -> Option<char> {
    String::from(str1).chars().last()
}

pub fn peekp(str1: &str) -> char{
    match peek(str1) {
        Some(n) => n,
        None => panic!("No more char")
    }
}

pub fn string_to_str(s: String) -> &'static str {
    unsafe {
        let ret = mem::transmute(&s as &str);
        mem::forget(s);
        ret
    }
}

pub fn adv_contains_any_str(target: &str, search: &[&str]) -> (bool,usize,String) {
    for i in 0..search.len() {
        match target.find(search[i]) {
            Some(n) => return (true,n,String::from(search[i])),
            None => {}
        }
    }

    (false,0,String::new())
}

pub fn contains_any_str(target: &str, search: &[&str]) -> bool {
    adv_contains_any_str(target,search).0
}

pub fn adv_contains_all_strs(target: &str, search: &[&str]) -> (bool,Vec<usize>,Vec<String>) {
    let mut idxs = Vec::<usize>::new();
    let mut strs = Vec::<String>::new();

    for i in 0..search.len() {
        match target.find(search[i]) {
            Some(n) => {
                idxs.push(n);
                strs.push(String::from(search[i]));
            },
            None => return (false,idxs,strs)
        }
    }

    (true,idxs,strs)
}

pub fn contains_all_strs(target: &str, search: &[&str]) -> bool {
    adv_contains_all_strs(target,search).0
}

pub fn adv_contains_any_char(target: &str, search: &[char]) -> (bool,usize,char) {
    for i in 0..search.len() {
        match target.find(search[i]) {
            Some(n) => return (true,n,search[i]),
            None => {}
        }
    }

    (false,0,' ')
}

pub fn contains_any_char(target: &str, search: &[char]) -> bool {
    adv_contains_any_char(target,search).0
}

pub fn adv_contains_all_chars(target: &str, search: &[char]) -> (bool,Vec<usize>,Vec<char>) {
    let mut idxs = Vec::<usize>::new();
    let mut chars = Vec::<char>::new();

    for i in 0..search.len() {
        match target.find(search[i]) {
            Some(n) => {
                idxs.push(n);
                chars.push(search[i]);
            },
            None => return (false,idxs,chars)
        }
    }

    (true,idxs,chars)
}

pub fn contains_all_chars(target: &str, search: &[char]) -> bool{
    adv_contains_all_chars(target,search).0
}

pub fn adv_contains_none_str(target: &str, search: &[&str]) -> (bool,usize,String) {
    for i in 0..search.len() {
        match target.find(search[i]) {
            Some(_) => {
                return (false,i,String::from(search[i]));
            },
            None => {}
        }
    }

    (true,0,String::new())
}

pub fn contains_none_str(target: &str, search: &[&str]) -> bool{
    adv_contains_none_str(target,search).0
}

pub fn adv_contains_none_char(target: &str, search: &[char]) -> (bool,usize,char) {
    for i in 0..search.len() {
        match target.find(search[i]) {
            Some(_) => {
                return (false,i,search[i]);
            },
            None => {}
        }
    }

    (true,0,' ')
}

pub fn contains_none_char(target: &str, search: &[char]) -> bool {
    adv_contains_none_char(target,search).0
}

pub fn difference(str1: &str, str2: &str) -> (Vec<usize>,Result<String,FromUtf8Error>) {
    let bytes1 = String::from(str1).into_bytes();
    let bytes2 = String::from(str2).into_bytes();
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

    (idxs,String::from_utf8(res))
}

pub fn differencep(str1: &str, str2: &str) -> (Vec<usize>,String) {
    let temp = difference(str1,str2);
    let idxs = temp.0;
    let s = match difference(str1, str2).1{
        Ok(n) => n,
        Err(err) => panic!("{}",err)
    };

    (idxs,s)
}

pub fn join<T: ToString>(ary: &[T], sep: &str) -> String {
    let mut temp = String::new();
    for i in 0..ary.len() {
        temp.push_str(&ary[i].to_string());

        if i != ary.len()-1 {
            temp.push_str(sep);
        }
    }
    temp
}

pub fn join_vec<T: ToString>(ary: Vec<T>, sep: &str) -> String {
    let mut temp = String::new();
    for i in 0..ary.len() {
        temp.push_str(&ary[i].to_string());

        if i != ary.len()-1 {
            temp.push_str(sep);
        }
    }
    temp
}

/*TODO: is_match() BUG --- "123" matched r"[0]*"
pub fn remove(target: &str, regex: &str) -> String {
    let re = Regex::new(regex).unwrap();
    println!("{}",re.is_match(target));
    if re.is_match(target){
        match re.find(target) {
            Some(n) => {
                let mut temp = String::from(target);
                let mut i = 0;
                println!("{:?}",n);
                while i < n.1 - n.0 {
                    println!("{:?}",temp);
                    temp.remove(n.0);
                    i = i + 1;
                }
                return temp;
            },
            None => {}
        }
    }

    String::from(target)
}

pub fn remove_all(target: &str, regex: &str) -> String {
    let mut prev = String::new();
    let mut temp = remove(target,regex);

    while temp != String::new(){
        prev = temp.clone();
        temp = remove(&temp,regex);
    }

    return prev;
}
*/

/*TODO
contains_only
ends_with_any
stars_with_any
ends_with_none
starts_with_none
is_all_lowercase
is_all_uppercase
*/
