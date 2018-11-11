

/*!
Control of lock of the file using the 'flock' functions.


# Capabilities

1. Convenient and transparent trait of a call of locks.
2. Automatic unlocking of lock.

# Locks

1. ExclusiveLock - Only one process can retain exclusive lock of the file.
2. SharedLock - Can retain the general lock on the given file more than one process.

# Use

```
extern crate cluFlock;

use cluFlock::Flock;
use std::fs::File;

fn main() {
     let file = File::create("/tmp/1").unwrap();

     let lock = file.exclusive_lock();
     //Only one process can retain exclusive lock of the file.

     println!("{:?}", lock);

     drop(lock);
}
```

*/



#[cfg(unix)]
mod unix;
#[cfg(unix)]
use unix as sys;

use std::fmt::Debug;
use std::fs::File;
use std::io;

mod lock;
pub use self::lock::*;


pub trait Flock<'a> {
     type ExclusiveLock: FlockLock + 'a;
     type SharedLock: FlockLock + 'a;

     fn try_exclusive_lock(&'a self) -> Result<Option<Self::ExclusiveLock>, io::Error>;
     
     ///Set exclusive lock. Lock current thread in case of file lock. Only one process can retain exclusive lock of the file.
     fn exclusive_lock(&'a self) -> Result<Self::ExclusiveLock, io::Error>;

     
     fn try_shared_lock(&'a self) -> Result<Option<Self::SharedLock>, io::Error>;

     ///Set shared lock. Lock current thread in case of file lock. Can retain the general lock on the given file more than one process.
     fn shared_lock(&'a self) -> Result<Self::SharedLock, io::Error>;
}

impl<'l, 'a, F: Flock<'a>> Flock<'a> for &'l F {
     type ExclusiveLock = F::ExclusiveLock;
     type SharedLock = F::SharedLock;

     #[inline(always)]
     fn try_exclusive_lock(&'a self) -> Result<Option<Self::ExclusiveLock>, io::Error> {(**self).try_exclusive_lock()}

     #[inline(always)]
     fn exclusive_lock(&'a self) -> Result<Self::ExclusiveLock, io::Error> {(**self).exclusive_lock()}

     #[inline(always)]
     fn try_shared_lock(&'a self) -> Result<Option<Self::SharedLock>, io::Error> {(**self).try_shared_lock()}

     #[inline(always)]
     fn shared_lock(&'a self) -> Result<Self::SharedLock, io::Error> {(**self).shared_lock()}
}

impl<'l, 'a, F: Flock<'a>> Flock<'a> for &'l mut F {
     type ExclusiveLock = F::ExclusiveLock;
     type SharedLock = F::SharedLock;

     #[inline(always)]
     fn try_exclusive_lock(&'a self) -> Result<Option<Self::ExclusiveLock>, io::Error> {(**self).try_exclusive_lock()}

     #[inline(always)]
     fn exclusive_lock(&'a self) -> Result<Self::ExclusiveLock, io::Error> {(**self).exclusive_lock()}

     #[inline(always)]
     fn try_shared_lock(&'a self) -> Result<Option<Self::SharedLock>, io::Error> {(**self).try_shared_lock()}

     #[inline(always)]
     fn shared_lock(&'a self) -> Result<Self::SharedLock, io::Error> {(**self).shared_lock()}
}


/*
impl<'l, A: AsRef<File>> Flock for &'l A {
     #[inline]
     fn try_exclusive_lock<'a>(&'a self) -> Result<ExclusiveFlockLock<'a>, io::Error> {
          ExclusiveFlockLock::try_lock(self.as_ref())
     }

     #[inline]
     fn exclusive_lock<'a>(&'a self) -> Result<ExclusiveFlockLock<'a>, io::Error> {
          ExclusiveFlockLock::lock(self.as_ref())
     }

     #[inline]
     fn try_shared_lock<'a>(&'a self) -> Result<SharedFlockLock<'a>, io::Error> {
          SharedFlockLock::try_lock(self.as_ref())
     }

     #[inline]
     fn shared_lock<'a>(&'a self) -> Result<SharedFlockLock<'a>, io::Error> {
          SharedFlockLock::lock(self.as_ref())
     }
}*/


impl<'a> Flock<'a> for File {
     type ExclusiveLock = ExclusiveFlockLock<'a>;
     type SharedLock = SharedFlockLock<'a>;

     #[inline]
     fn try_exclusive_lock(&'a self) -> Result<Option<Self::ExclusiveLock>, io::Error> {
          ExclusiveFlockLock::try_lock(self)
     }

     #[inline]
     fn exclusive_lock(&'a self) -> Result<Self::ExclusiveLock, io::Error> {
          ExclusiveFlockLock::lock(self)
     }

     #[inline]
     fn try_shared_lock(&'a self) -> Result<Option<Self::SharedLock>, io::Error> {
          SharedFlockLock::try_lock(self)
     }

     #[inline]
     fn shared_lock(&'a self) -> Result<Self::SharedLock, io::Error> {
          SharedFlockLock::lock(self)
     }
}


pub trait FlockLock: Drop + Debug {
     fn unlock(self) where Self: Sized {}
     fn box_unlock(self: Box<Self>) where Self: Sized {}
}


pub (crate) trait InitFlockLock<'a> {
     type Lock: FlockLock + 'a;
     
     fn new(f: &'a File) -> Self::Lock;
}