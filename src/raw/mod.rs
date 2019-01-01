
pub mod unix;

pub trait RawConstFlock {
     type Lock;
     type Arg;
     
     fn next(f: Self::Arg) -> Self::Lock;
}

