
use std::fs::File;
use crate::ToFlock;


mod file;
pub use self::file::*;

mod file_fn;
pub use self::file_fn::*;


impl ToFlock for File {}
