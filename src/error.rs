// <editor-fold> # Enums
#[derive(PartialEq, Debug)]
pub enum ParseError {
    InvalidNumber(String),
    InvalidString(String)
}

#[derive(PartialEq, Debug)]
pub enum ArithmeticError {
    DivideByZero
}
// </editor-fold>
