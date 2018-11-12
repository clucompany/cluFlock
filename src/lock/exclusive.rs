

use std::ops::DerefMut;
use std::ops::Deref;
use FlockLock;
use InitFlockLock;
use std::fs::File;
use std::io;

///Only one process can retain exclusive lock of the file.
#[derive(Debug)]
pub struct ExclusiveLock(File);

impl ExclusiveLock {
     pub fn lock(f: File) -> Result<Self, io::Error> {
          crate::sys::lock_exclusive::<Self>(f)
     }

     pub fn try_lock(f: File) -> Result<Option<Self>, io::Error> {
          crate::sys::try_lock_exclusive::<Self>(f)
     }
}

impl InitFlockLock for ExclusiveLock {
     type Lock = Self;
     type Arg = File;

     fn new(f: Self::Arg) -> Self::Lock {
          ExclusiveLock(f)
     }
}


impl Deref for ExclusiveLock {
     type Target = File;

     #[inline(always)]
     fn deref(&self) -> &Self::Target {
          &self.0
     }
}

impl DerefMut for ExclusiveLock {
     #[inline(always)]
     fn deref_mut(&mut self) -> &mut Self::Target {
          &mut self.0
     }
}

impl AsRef<File> for ExclusiveLock {
     #[inline(always)]
     fn as_ref(&self) -> &File {
          &*self
     }
}

impl FlockLock for ExclusiveLock {}

impl Drop for ExclusiveLock {
     fn drop(&mut self) {
          let _e = crate::sys::unlock(&self.0);
     }
}
