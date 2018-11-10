
use std::fs::File;
use FlockLock;
use std::io;


#[derive(Debug)]
pub struct UniqueFlockLock<'a>(&'a File);

impl<'a> UniqueFlockLock<'a> {
     #[inline]
     const fn new(f: &'a File) -> Self {
          UniqueFlockLock(f)
     }

     pub fn lock(f: &'a File) -> Result<Self, io::Error> {
          crate::sys::lock_unigue(f)?;
          Ok( Self::new(f) )
     }

     pub fn try_lock(f: &'a File) -> Result<Self, io::Error> {
          crate::sys::try_lock_unigue(f)?;
          Ok( Self::new(f) )
     }
}

impl<'a> Drop for UniqueFlockLock<'a> {
     fn drop(&mut self) {
          let _e = crate::sys::unlock(self.0);
     }
}

impl<'a> FlockLock for UniqueFlockLock<'a> {}

#[derive(Debug)]
pub struct SharedFlockLock<'a>(&'a File);

impl<'a> SharedFlockLock<'a> {
     #[inline]
     const fn new(f: &'a File) -> Self {
          SharedFlockLock(f)
     }

     pub fn lock(f: &'a File) -> Result<Self, io::Error> {
          crate::sys::lock_shared(f)?;
          Ok( Self::new(f) )
     }

     pub fn try_lock(f: &'a File) -> Result<Self, io::Error> {
          crate::sys::try_lock_shared(f)?;
          Ok( Self::new(f) )
     }
}

impl<'a> Drop for SharedFlockLock<'a> {
     fn drop(&mut self) {
          let _e = crate::sys::unlock(self.0);
     }
}

impl<'a> FlockLock for SharedFlockLock<'a> {}
