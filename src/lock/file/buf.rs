
use crate::FlockLock;
use crate::raw::RawConstFlock;

use std::ops::DerefMut;
use std::ops::Deref;
use std::fs::File;
use std::io;

#[derive(Debug)]
pub struct FileFlock(File);

impl FileFlock {
     #[inline]
     const fn new(f: File) -> Self {
          FileFlock(f)
     }

     pub fn wait_lock_exclusive(f: File) -> Result<Self, io::Error> {
          crate::sys::wait_lock_exclusive::<Self>(f)
     }  

     pub fn wait_lock_shared(f: File) -> Result<Self, io::Error> {
          crate::sys::wait_lock_shared::<Self>(f)
     }

     pub fn try_lock_exclusive(f: File) -> Result<Self, io::Error> {
          crate::sys::try_lock_exclusive::<Self>(f)
     }
     
     pub fn try_lock_shared(f: File) -> Result<Self, io::Error> {
          crate::sys::try_lock_shared::<Self>(f)
     }
}

impl<'a> RawConstFlock<'a> for FileFlock {
     type Lock = Self;
     type Arg = File;

     #[inline(always)]
     fn new(a: Self::Arg) -> Self::Lock {
          Self::Lock::new(a)
     }
}

/*
impl Into<File> for FileFlock {
     fn into(self) -> File {
          (self.0).0
     }
}*/

impl Deref for FileFlock {
     type Target = File;

     #[inline(always)]
     fn deref(&self) -> &Self::Target {
          &self.0
     }
}

impl DerefMut for FileFlock {
     #[inline(always)]
     fn deref_mut(&mut self) -> &mut Self::Target {
          &mut self.0
     }
}

impl AsRef<File> for FileFlock {
     #[inline(always)]
     fn as_ref(&self) -> &File {
          &self.0
     }
}

impl AsMut<File> for FileFlock {
     #[inline(always)]
     fn as_mut(&mut self) -> &mut File {
          &mut self.0
     }
}

impl FlockLock for FileFlock {}


impl Drop for FileFlock {
     fn drop(&mut self) {
          let _e = crate::sys::unlock(&self.0);
     }
}



