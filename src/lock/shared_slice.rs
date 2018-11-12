

use std::ops::Deref;
use FlockLock;
use InitFlockLock;
use std::fs::File;
use std::io;

///Can retain the general lock on the given file more than one process.
#[derive(Debug)]
pub struct SharedSliceLock<'a>(&'a File);

impl<'a> SharedSliceLock<'a> {
     pub fn lock(f: &'a File) -> Result<Self, io::Error> {
          crate::sys::lock_shared::<Self>(f)
     }

     pub fn try_lock(f: &'a File) -> Result<Option<Self>, io::Error> {
          crate::sys::try_lock_shared::<Self>(f)
     }
}

impl<'a> InitFlockLock for SharedSliceLock<'a> {
     type Lock = Self;
     type Arg = &'a File;

     fn new(f: Self::Arg) -> Self::Lock {
          SharedSliceLock(f)
     }
}

impl<'a> Deref for SharedSliceLock<'a> {
     type Target = File;

     #[inline(always)]
     fn deref(&self) -> &Self::Target {
          &self.0
     }
}

impl<'a> FlockLock for SharedSliceLock<'a> {}

impl<'a> Drop for SharedSliceLock<'a> {
     fn drop(&mut self) {
          let _e = crate::sys::unlock(&self.0);
     }
}
