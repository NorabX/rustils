pub mod parse;

#[test]
fn parse_bool_to_u16(){
    let a: u16 = 0;
    let b: bool = false;
    assert_eq!(a,parse::boolean::tou16(b));
}
