
#[cfg(unix)]
mod unix;
#[cfg(unix)]
use unix as sys;

use std::fmt::Debug;
use std::fs::File;
use std::io;

mod lock;
pub use self::lock::*;


pub trait Flock {
     fn try_unique_lock<'a>(&'a self) -> Result<UniqueFlockLock<'a>, io::Error>;
     fn unique_lock<'a>(&'a self) -> Result<UniqueFlockLock<'a>, io::Error>;

     fn try_shared_lock<'a>(&'a self) -> Result<SharedFlockLock<'a>, io::Error>;
     fn shared_lock<'a>(&'a self) -> Result<SharedFlockLock<'a>, io::Error>;
}

impl<'a, F: Flock> Flock for &'a F {
     #[inline(always)]
     fn try_unique_lock(&self) -> Result<UniqueFlockLock, io::Error> {(**self).try_unique_lock()}

     #[inline(always)]
     fn unique_lock(&self) -> Result<UniqueFlockLock, io::Error> {(**self).unique_lock()}

     #[inline(always)]
     fn try_shared_lock(&self) -> Result<SharedFlockLock, io::Error> {(**self).try_shared_lock()}

     #[inline(always)]
     fn shared_lock(&self) -> Result<SharedFlockLock, io::Error> {(**self).shared_lock()}
}

impl<'a, F: Flock> Flock for &'a mut F {
     #[inline(always)]
     fn try_unique_lock(&self) -> Result<UniqueFlockLock, io::Error> {(**self).try_unique_lock()}

     #[inline(always)]
     fn unique_lock(&self) -> Result<UniqueFlockLock, io::Error> {(**self).unique_lock()}

     #[inline(always)]
     fn try_shared_lock(&self) -> Result<SharedFlockLock, io::Error> {(**self).try_shared_lock()}

     #[inline(always)]
     fn shared_lock(&self) -> Result<SharedFlockLock, io::Error> {(**self).shared_lock()}
}


/*
impl<'l, A: AsRef<File>> Flock for &'l A {
     #[inline]
     fn try_unique_lock<'a>(&'a self) -> Result<UniqueFlockLock<'a>, io::Error> {
          UniqueFlockLock::try_lock(self.as_ref())
     }

     #[inline]
     fn unique_lock<'a>(&'a self) -> Result<UniqueFlockLock<'a>, io::Error> {
          UniqueFlockLock::lock(self.as_ref())
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

impl Flock for File {
     #[inline]
     fn try_unique_lock<'a>(&'a self) -> Result<UniqueFlockLock<'a>, io::Error> {
          UniqueFlockLock::try_lock(self)
     }

     #[inline]
     fn unique_lock<'a>(&'a self) -> Result<UniqueFlockLock<'a>, io::Error> {
          UniqueFlockLock::lock(self)
     }

     #[inline]
     fn try_shared_lock<'a>(&'a self) -> Result<SharedFlockLock<'a>, io::Error> {
          SharedFlockLock::try_lock(self)
     }

     #[inline]
     fn shared_lock<'a>(&'a self) -> Result<SharedFlockLock<'a>, io::Error> {
          SharedFlockLock::lock(self)
     }
}

/*
impl<'a, F: Flock> Flock for &'a F {
     #[inline(always)]
     fn try_unique_lock(&self) -> Result<UniqueFlockLock, io::Error> {(**self).try_unique_lock()}

     #[inline(always)]
     fn unique_lock(&self) -> Result<UniqueFlockLock, io::Error> {(**self).unique_lock()}

     #[inline(always)]
     fn try_shared_lock(&self) -> Result<SharedFlockLock, io::Error> {(**self).try_shared_lock()}

     #[inline(always)]
     fn shared_lock(&self) -> Result<SharedFlockLock, io::Error> {(**self).shared_lock()}
}*/


pub trait FlockLock: Drop + Debug {
     fn unlock(self) where Self: Sized {}
}