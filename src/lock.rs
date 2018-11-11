
use InitFlockLock;
use std::fs::File;
use FlockLock;
use std::io;

///Only one process can retain exclusive lock of the file.
#[derive(Debug)]
pub struct ExclusiveFlockLock<'a>(&'a File);

impl<'a> ExclusiveFlockLock<'a> {
     pub fn lock(f: &'a File) -> Result<Self, io::Error> {
          crate::sys::lock_unigue::<Self>(f)
     }

     pub fn try_lock(f: &'a File) -> Result<Option<Self>, io::Error> {
          crate::sys::try_lock_unigue::<Self>(f)
     }
}
impl<'a> InitFlockLock<'a> for ExclusiveFlockLock<'a> {
     type Lock = Self;

     #[inline]
     fn new(f: &'a File) -> Self::Lock {
          ExclusiveFlockLock(f)
     }
}

impl<'a> Drop for ExclusiveFlockLock<'a> {
     fn drop(&mut self) {
          let _e = crate::sys::unlock(self.0);
     }
}

impl<'a> FlockLock for ExclusiveFlockLock<'a> {}


///Can retain the general lock on the given file more than one process.
#[derive(Debug)]
pub struct SharedFlockLock<'a>(&'a File);

impl<'a> SharedFlockLock<'a> {
     pub fn lock(f: &'a File) -> Result<Self, io::Error> {
          crate::sys::lock_shared::<Self>(f)
     }

     pub fn try_lock(f: &'a File) -> Result<Option<Self>, io::Error> {
          crate::sys::try_lock_shared::<Self>(f)
     }
}

impl<'a> InitFlockLock<'a> for SharedFlockLock<'a> {
     type Lock = Self;

     #[inline]
     fn new(f: &'a File) -> Self::Lock {
          SharedFlockLock(f)
     }
}

impl<'a> Drop for SharedFlockLock<'a> {
     fn drop(&mut self) {
          let _e = crate::sys::unlock(self.0);
     }
}

impl<'a> FlockLock for SharedFlockLock<'a> {}
