use std::mem;
use std::string::FromUtf8Error;
use std::iter::Iterator;

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

pub fn peek(str1: &str) -> Option<char>{
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
