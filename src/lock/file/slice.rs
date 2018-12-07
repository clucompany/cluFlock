

use raw::RawConstFlock;
use std::ops::Deref;
use FlockLock;
use std::fs::File;
use std::io;

///Only one process can retain exclusive lock of the file.
#[derive(Debug)]
pub struct FileSliceFlock<'a>(&'a File, SliceFileUnlock<'a>);

impl<'a> FileSliceFlock<'a> {
     #[inline]
     const fn new(f: &'a File) -> Self {
          FileSliceFlock(f, SliceFileUnlock::new(f))
     }

     pub fn wait_lock_exclusive(f: &'a File) -> Result<Self, io::Error> {
          crate::sys::wait_lock_exclusive::<Self>(f)
     }

     pub fn wait_lock_shared(f: &'a File) -> Result<Self, io::Error> {
          crate::sys::wait_lock_shared::<Self>(f)
     }


     pub fn try_lock_exclusive(f: &'a File) -> Result<Self, io::Error> {
          crate::sys::try_lock_exclusive::<Self>(f)
     }
     
     pub fn try_lock_shared(f: &'a File) -> Result<Self, io::Error> {
          crate::sys::try_lock_shared::<Self>(f)
     }

     #[inline(always)]
     pub fn unlock(self) -> &'a File {
          self.into()
     }
}

impl<'a> RawConstFlock<'a> for FileSliceFlock<'a> {
     type Lock = Self;
     type Arg = &'a File;

     fn new(a: Self::Arg) -> Self::Lock {
          Self::Lock::new(a)
     }
}

impl<'a> Into<&'a File> for FileSliceFlock<'a> {
     #[inline(always)]
     fn into(self) -> &'a File {
          self.0
     }
}


impl<'a> Deref for FileSliceFlock<'a> {
     type Target = File;

     #[inline(always)]
     fn deref(&self) -> &Self::Target {
          &self.0
     }
}

impl<'a> AsRef<File> for FileSliceFlock<'a> {
     #[inline(always)]
     fn as_ref(&self) -> &File {
          &*self
     }
}

impl<'a> FlockLock for FileSliceFlock<'a> {}




#[derive(Debug)]
struct SliceFileUnlock<'a>(&'a File);

impl<'a> SliceFileUnlock<'a> {
     #[inline]
     const fn new(f: &'a File) -> Self {
          SliceFileUnlock(f)
     }
}


impl<'a> Drop for SliceFileUnlock<'a> {
     fn drop(&mut self) {
          let _e = crate::sys::unlock(&self.0);
     }
}
