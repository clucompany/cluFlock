

use std::ops::Deref;
use FlockLock;
use InitFlockLock;
use std::fs::File;
use std::io;

///Only one process can retain exclusive lock of the file.
#[derive(Debug)]
pub struct ExclusiveSliceLock<'a>(&'a File);

impl<'a> ExclusiveSliceLock<'a> {
     pub fn lock(f: &'a File) -> Result<Self, io::Error> {
          crate::sys::lock_exclusive::<Self>(f)
     }

     pub fn try_lock(f: &'a File) -> Result<Option<Self>, io::Error> {
          crate::sys::try_lock_exclusive::<Self>(f)
     }
}

impl<'a> InitFlockLock for ExclusiveSliceLock<'a> {
     type Lock = Self;
     type Arg = &'a File;

     fn new(f: Self::Arg) -> Self::Lock {
          ExclusiveSliceLock(f)
     }
}


impl<'a> Deref for ExclusiveSliceLock<'a> {
     type Target = File;

     #[inline(always)]
     fn deref(&self) -> &Self::Target {
          &self.0
     }
}


impl<'a> FlockLock for ExclusiveSliceLock<'a> {}

impl<'a> Into<&'a File> for ExclusiveSliceLock<'a> {
     #[inline(always)]
     fn into(self) -> &'a File {
          self.0
     }
}

impl<'a> AsRef<File> for ExclusiveSliceLock<'a> {
     #[inline(always)]
     fn as_ref(&self) -> &File {
          &*self
     }
}

impl<'a> Drop for ExclusiveSliceLock<'a> {
     fn drop(&mut self) {
          let _e = crate::sys::unlock(self.0);
     }
}
