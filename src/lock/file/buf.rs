

use raw::RawConstFlock;
use std::ops::DerefMut;
use std::ops::Deref;
use FlockLock;
use std::fs::File;
use std::io;

///Only one process can retain exclusive lock of the file.
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

     fn new(a: Self::Arg) -> Self::Lock {
          Self::Lock::new(a)
     }
}


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
          &*self
     }
}

impl FlockLock for FileFlock {}

impl Drop for FileFlock {
     fn drop(&mut self) {
          let _e = crate::sys::unlock(&self.0);
     }
}
