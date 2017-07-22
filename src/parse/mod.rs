pub mod boolean;
pub mod byte;
pub mod double;
pub mod float;
pub mod int;
pub mod isize;
pub mod long;
pub mod short;
pub mod string;
pub mod ubyte;
pub mod uint;
pub mod ulong;
pub mod ushort;
// pub mod usize;

use { RoundingMode };
use error::ParseError;



pub type ParseResultUsize = Result<usize,ParseError>;





pub trait ToUsize {
    fn to_usize_res(self) -> ParseResultUsize;
    fn to_usize(self) -> usize;
}

pub trait ToUsizeRM {
    fn to_usize_rm_res(self, rm: RoundingMode) -> ParseResultUsize;
    fn to_usize_rm(self, rm: RoundingMode) -> usize;
}
