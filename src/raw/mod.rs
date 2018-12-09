

use crate::FlockLock;

pub mod unix;

pub trait RawConstFlock<'a> {
     type Lock: FlockLock + 'a;
     type Arg: 'a;
     
     fn new(f: Self::Arg) -> Self::Lock;
}

