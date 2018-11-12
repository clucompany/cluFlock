

use std::ops::DerefMut;
use std::ops::Deref;
use FlockLock;
use InitFlockLock;
use std::fs::File;
use std::io;

///Can retain the general lock on the given file more than one process.
#[derive(Debug)]
pub struct SharedLock(File);

impl SharedLock {
     pub fn lock(f: File) -> Result<Self, io::Error> {
          crate::sys::lock_shared::<Self>(f)
     }

     pub fn try_lock(f: File) -> Result<Option<Self>, io::Error> {
          crate::sys::try_lock_shared::<Self>(f)
     }
}

impl InitFlockLock for SharedLock {
     type Lock = Self;
     type Arg = File;

     fn new(f: Self::Arg) -> Self::Lock {
          SharedLock(f)
     }
}

impl Deref for SharedLock {
     type Target = File;

     #[inline(always)]
     fn deref(&self) -> &Self::Target {
          &self.0
     }
}

impl DerefMut for SharedLock {
     #[inline(always)]
     fn deref_mut(&mut self) -> &mut Self::Target {
          &mut self.0
     }
}

impl AsRef<File> for SharedLock {
     #[inline(always)]
     fn as_ref(&self) -> &File {
          &*self
     }
}

impl FlockLock for SharedLock {}

impl Drop for SharedLock {
     fn drop(&mut self) {
          let _e = crate::sys::unlock(&self.0);
     }
}
